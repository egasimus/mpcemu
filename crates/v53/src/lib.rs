/// <https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF>

mod bit;
mod reg;
mod flag;
mod inst;
mod dump;
#[cfg(test)] mod test;

pub(crate) use self::{bit::*, reg::*, flag::*, inst::*};

use std::collections::BTreeMap;

pub struct CPU {
    memory:   [u8;0x100000],
    extended: [u8;0xA0000],
    ports:    [u8;0x10000],
    internal: [u8;0x100],

    aw:  u16,
    bw:  u16,
    cw:  u16,
    dw:  u16,

    ps:  u16,
    ss:  u16,
    ds0: u16,
    ds1: u16,

    sp:  u16,
    bp:  u16,
    pc:  u16,
    psw: u16,

    ix:  u16,
    iy:  u16,

    pub segment: Option<Segment>,
    opcode:      u8,
    pub clock:   u64,

    outputs: BTreeMap<u16, Box<dyn Fn(&CPU)->()>>,
}

/// Segment override
#[derive(Debug, Copy, Clone)]
pub enum Segment {
    /// Use data segment 0
    DS0,
    /// Use data segment 1
    DS1,
    /// Use program segment
    PS,
    /// Use stack segment
    SS,
}

impl CPU {

    pub fn new (image: Vec<u8>) -> Self {
        let mut memory = [0x00;0x100000];
        if image.len() > memory.len() {
            panic!("Memory image too big (0x{:X}/0x{:X} bytes)", image.len(), memory.len());
        }
        for i in 0..image.len() {
            memory[i] = image[i];
        }
        Self {
            memory,
            extended: [0x00;0xA0000],
            ports:    [0x00;0x10000],
            internal: [0x00;0x100],
            aw:       0x0000,
            bw:       0x0000,
            cw:       0x0000,
            dw:       0x0000,
            ps:       0xffff,
            ss:       0x0000,
            ds0:      0x0000,
            ds1:      0x0000,
            sp:       0x0000,
            bp:       0x0000,
            pc:       0x0000,
            psw:      W15 | W14 | W13 | W12 | W2,
            ix:       0x0000,
            iy:       0x0000,
            segment:  None,
            opcode:   0xF1,
            clock:    0x0000,
            outputs:  BTreeMap::new(),
        }
    }

    /// Read and execute the next instruction in the program
    pub fn step (&mut self, debug: bool) {
        let (addr, pc, (name, bytes, instruction)) = self.fetch_instruction();
        if debug {
            self.dump_state(pc);
            self.dump_instruction(addr, &name, &bytes);
        }
        self.execute_instruction(instruction)
    }

    pub fn fetch_instruction (&mut self) -> (
        u32, u16, (String, Vec<u8>, Box<dyn Fn(&mut CPU)->u64>)
    ) {
        let addr   = self.program_address();
        let pc     = self.pc();
        let opcode = self.next_u8();
        self.opcode = opcode;
        (addr, pc, v53_instruction(self, opcode))
    }

    pub fn execute_instruction (&mut self, instruction: Box<dyn Fn(&mut CPU)->u64>) {
        self.clock += instruction(self);
        // Reset segment override, except if it was just set:
        // FIXME: make this a part of instruction decoding
        let opcode = self.opcode;
        if !(
            (opcode == 0x26) || (opcode == 0x2E) || (opcode == 0x36) || (opcode == 0x3E)
        ) {
            self.segment = None
        }
    }

    /// Get the opcode that is currently being executed
    pub fn opcode (&self) -> u8 {
        self.opcode
    }

    pub fn jump_i8 (&mut self, displace: i8) {
        self.pc = ((self.pc as i16) + (displace as i16)) as u16;
    }

    pub fn jump_i16 (&mut self, displace: i16) {
        self.pc = ((self.pc as i16) + displace) as u16;
    }

    pub fn parse_effective_address (&mut self, mode: u8, mem: u8)
        -> (String, Vec<u8>, Box<dyn Fn(&mut CPU)->u16>)
    {
        match mode {
            0b00 => match mem {
                0b000 => (
                    "BW + IX".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.bw() + cpu.ix()})
                ),
                0b001 => (
                    "BW + IY".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.bw() + cpu.iy()})
                ),
                0b010 => (
                    "BP + IX".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.bp() + cpu.ix()})
                ),
                0b011 => (
                    "BP + IY".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.bp() + cpu.iy()})
                ),
                0b100 => (
                    "IX".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.ix()})
                ),
                0b101 => (
                    "IY".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.iy()})
                ),
                0b110 => {
                    let direct = self.next_u16();
                    (
                        format!("{direct}"), direct.to_le_bytes().to_vec(),
                        Box::new(move |_|{direct})
                    )
                },
                0b111 => (
                    "BW".into(), vec![],
                    Box::new(|cpu: &mut CPU|{cpu.bw()})
                ),
                _ => panic!("invalid memory inner mode {:b}", mem)
            },
            0b01 => {
                let disp  = self.next_u8();
                let bytes = vec![disp];
                let disp  = disp as u16;
                match mem {
                    0b000 => (
                        format!("BW + IX + {disp:02X}"), bytes,
                        Box::new(move |cpu: &mut CPU|{cpu.bw() + cpu.ix() + disp})
                    ),
                    0b001 => (
                        format!("BW + IY + {disp:02X}"), bytes,
                        Box::new(move |cpu: &mut CPU|{cpu.bw() + cpu.iy() + disp})
                    ),
                    0b010 => (
                        format!("BP + IX + {disp:02X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bp() + cpu.ix() + disp})
                    ),
                    0b011 => (
                        format!("BP + IY + {disp:02X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bp() + cpu.iy() + disp})
                    ),
                    0b100 => (
                        format!("IX + {disp:02X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.ix() + disp})
                    ),
                    0b101 => (
                        format!("IY + {disp:02X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.iy() + disp})
                    ),
                    0b110 => (
                        format!("BP + {disp:02X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bp() + disp})
                    ),
                    0b111 => (
                        format!("BW + {disp:02X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bw() + disp})
                    ),
                    _ => panic!("invalid memory inner mode {:b}", mem)
                }
            },
            0b10 => {
                let disp  = self.next_u16();
                let bytes = disp.to_le_bytes().to_vec();
                match mem {
                    0b000 => (
                        format!("BW + IX + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bw() + cpu.ix() + disp})
                    ),
                    0b001 => (
                        format!("BW + IY + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bw() + cpu.iy() + disp})
                    ),
                    0b010 => (
                        format!("BP + IX + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bp() + cpu.ix() + disp})
                    ),
                    0b011 => (
                        format!("BP + IY + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bp() + cpu.iy() + disp})
                    ),
                    0b100 => (
                        format!("IX + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.ix() + disp})
                    ),
                    0b101 => (
                        format!("IY + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.iy() + disp})
                    ),
                    0b110 => (
                        format!("BP + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bp() + disp})
                    ),
                    0b111 => (
                        format!("BW + {disp:04X}"), bytes,
                        Box::new(move|cpu: &mut CPU|{cpu.bw() + disp})
                    ),
                    _ => panic!("invalid memory inner mode {:b}", mem)
                }
            },
            0b11 => panic!(),
            _ => unreachable!()
        }
    }

    #[inline]
    pub fn memory_address (&mut self, mode: u8, mem: u8) -> u32 {
        match mode {
            0b00 => self.memory_address_00(mem),
            0b01 => {
                let displace = self.next_u8();
                self.memory_address_01(mem, displace)
            },
            0b10 => {
                let displace = self.next_u16();
                self.memory_address_10(mem, displace)
            },
            _ => panic!("invalid memory outer mode {:b}", mode)
        }
    }

    //#[inline]
    //pub fn memory_address_dasm (&mut self, dasm: &[u8], mode: u8, mem: u8) -> (u16, Vec<u8>) {
        //let dasm = Vec::from(dasm);
        //match mode {
            //0b00 => self.memory_address_00(mem),
            //0b01 => {
                //let displace = self.next_u8();
                //self.memory_address_01(mem, displace)
            //},
            //0b10 => {
                //let displace = self.next_u16();
                //self.memory_address_10(mem, displace)
            //},
            //_ => panic!("invalid memory outer mode {:b}", mode)
        //}
    //}

    #[inline]
    pub fn memory_address_00 (&mut self, mem: u8) -> u32 {
        match mem {
            0b000 => self.bw() as u32 + self.ix() as u32,
            0b001 => self.bw() as u32 + self.iy() as u32,
            0b010 => self.bp() as u32 + self.ix() as u32,
            0b011 => self.bp() as u32 + self.iy() as u32,
            0b100 => self.ix() as u32,
            0b101 => self.iy() as u32,
            0b110 => self.next_u16() as u32,
            0b111 => self.bw() as u32,
            _ => panic!("invalid memory inner mode {:b}", mem)
        }
    }

    #[inline]
    pub fn memory_address_01 (&self, mem: u8, displace: u8) -> u32 {
        match mem {
            0b000 => self.bw() as u32 + self.ix() as u32 + displace as u32,
            0b001 => self.bw() as u32 + self.iy() as u32 + displace as u32,
            0b010 => self.bp() as u32 + self.ix() as u32 + displace as u32,
            0b011 => self.bp() as u32 + self.iy() as u32 + displace as u32,
            0b100 => self.ix() as u32 + displace as u32,
            0b101 => self.iy() as u32 + displace as u32,
            0b110 => self.bp() as u32 + displace as u32,
            0b111 => self.bw() as u32 + displace as u32,
            _ => panic!("invalid memory inner mode {:b}", mem)
        }
    }

    #[inline]
    pub fn memory_address_10 (&self, mem: u8, displace: u16) -> u32 {
        match mem {
            0b000 => self.bw() as u32 + self.ix() as u32 + displace as u32,
            0b001 => self.bw() as u32 + self.iy() as u32 + displace as u32,
            0b010 => self.bp() as u32 + self.ix() as u32 + displace as u32,
            0b011 => self.bp() as u32 + self.iy() as u32 + displace as u32,
            0b100 => self.ix() as u32 + displace as u32,
            0b101 => self.iy() as u32 + displace as u32,
            0b110 => self.bp() as u32 + displace as u32,
            0b111 => self.bw() as u32 + displace as u32,
            _ => panic!("invalid memory inner mode {:b}", mem)
        }
    }

    /// Read-only handle to memory
    pub fn memory (&self) -> &[u8] {
        &self.memory
    }

    /// Read-only handle to extended memory
    pub fn extended (&self) -> &[u8] {
        &self.extended
    }

    /// Read-only handle to IO ports memory
    pub fn ports (&self) -> &[u8] {
        &self.ports
    }

    /// Read-only handle to internal IO memory
    pub fn internal (&self) -> &[u8] {
        &self.internal
    }

    pub fn xa (&self) -> bool {
        self.ports[0xff80] > 0
    }

    pub fn set_xa (&mut self, value: bool) {
        self.ports[0xff80] = if value { 1 } else { 0 };
    }

    pub fn get_byte (&self, addr: u32) -> u8 {
        if addr < 0xA0000 {
            if self.xa() {
                self.extended[addr as usize]
            } else {
                self.memory[addr as usize]
            }
        } else {
            self.memory[addr as usize]
        }
    }

    pub fn set_byte (&mut self, addr: u32, value: u8) {
        if addr < 0xA0000 {
            if self.xa() {
                self.extended[addr as usize] = value
            } else {
                self.memory[addr as usize] = value
            }
        } else {
            self.memory[addr as usize] = value
        }
    }

    /// Program address
    pub fn program_address (&self) -> u32 {
        ((self.ps as u32) * 0x10) + self.pc as u32
    }

    /// Stack address
    pub fn stack_address (&self) -> u32 {
        ((self.ss as u32) * 0x10) + self.sp as u32
    }

    /// Effective address
    pub fn effective_address (&self, addr: u32) -> u32 {
        let segment = match self.segment {
            None               => self.ds0,
            Some(Segment::DS0) => self.ds0,
            Some(Segment::DS1) => self.ds1,
            Some(Segment::PS)  => self.ps,
            Some(Segment::SS)  => self.ss
        } as u32 * 0x10;
        (segment + addr as u32) % 0xFFFFF
    }

    /// Target address (always offset from DS1)
    pub fn ds1_address (&self, addr: u32) -> u32 {
        (self.ds1 as u32 * 0x10) + addr as u32
    }

    pub fn peek_u8 (&mut self) -> u8 {
        self.get_byte(self.program_address())
    }

    pub fn peek_i8 (&mut self) -> i8 {
        self.get_byte(self.program_address()) as i8
    }

    pub fn next_u8 (&mut self) -> u8 {
        let byte = self.peek_u8();
        self.pc += 1;
        byte
    }

    pub fn next_i8 (&mut self) -> i8 {
        let byte = self.peek_i8();
        self.pc += 1;
        byte
    }

    pub fn next_u16 (&mut self) -> u16 {
        let lo = self.next_u8() as u16;
        let hi = self.next_u8() as u16;
        hi << 8 | lo
    }

    pub fn next_i16 (&mut self) -> i16 {
        let lo = self.next_u8();
        let hi = self.next_u8();
        i16::from_le_bytes([lo, hi])
    }

    /// Read byte from effective address
    pub fn read_u8 (&mut self, addr: u32) -> u8 {
        self.get_byte(self.effective_address(addr))
    }

    /// Read word from effective address
    pub fn read_u16 (&mut self, addr: u32) -> u16 {
        let lo = self.read_u8(addr);
        let hi = self.read_u8(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    /// Write byte to effective address
    pub fn write_u8 (&mut self, addr: u32, value: u8) {
        let ea = self.effective_address(addr);
        self.set_byte(ea, value);
    }

    /// Write word to effective address
    pub fn write_u16 (&mut self, addr: u32, value: u16) {
        let ea = self.effective_address(addr);
        let [lo, hi] = value.to_le_bytes();
        self.set_byte(ea + 0, lo);
        self.set_byte(ea + 1, hi);
    }

    /// Read byte from input port
    pub fn input_u8 (&self, addr: u32) -> u8 {
        self.ports[addr as usize]
    }

    /// Read word from input port
    pub fn input_u16 (&self, addr: u32) -> u16 {
        let lo = self.input_u8(addr) as u16;
        let hi = self.input_u8(addr + 1) as u16;
        hi << 8 | lo
    }

    pub fn on_output (&mut self, addr: u16, callback: Box<dyn Fn(&CPU)->()>) {
        self.outputs.insert(addr, callback);
    }

    /// Write byte to input port
    pub fn output_u8 (&mut self, addr: u16, data: u8) {
        self.ports[addr as usize] = data;
        if let Some(callback) = self.outputs.get(&addr) {
            callback(&self);
        }
    }

    /// Write byte to output port
    pub fn output_u16 (&mut self, addr: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.output_u8(addr + 0, lo);
        self.output_u8(addr + 1, hi);
    }

    pub fn push_u16 (&mut self, data: u16) {
        //panic!("push {data}");
        self.set_sp(self.sp() - 2);
        let sp = self.stack_address() as usize;
        let [lo, hi] = data.to_le_bytes();
        self.memory[sp + 0] = lo;
        self.memory[sp + 1] = hi;
        //self.dump_stack(4);
    }

    pub fn pop_u16 (&mut self) -> u16 {
        let sp = self.stack_address() as usize;
        let lo = self.memory[sp + 0];
        let hi = self.memory[sp + 1];
        self.set_sp(self.sp() + 2);
        //self.dump_stack(4);
        u16::from_le_bytes([lo, hi])
    }

}

#[inline]
pub fn get_mode_reg_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, reg, mem]
}

#[inline]
pub fn get_mode_sreg_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let sreg = (arg & B_SREG) >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, sreg, mem]
}

#[inline]
pub fn get_mode_code_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let code = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, code, mem]
}

#[inline]
pub fn sign_extend_16 (data: u16, size: u16) -> i16 {
    assert!(size > 0 && size <= 16);
    ((data << (16 - size)) as i16) >> (16 - size)
}

#[inline]
pub fn sign_extend_32 (data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}

#[inline]
pub fn get_source_word (state: &mut CPU, arg: u8) -> u16 {
    let mode = (arg & B_MODE) >> 6;
    let mem  = arg & B_MEM;
    match mode {
        0b11 => state.get_register_u16(mem),
        _ => {
            let addr = state.memory_address(mode, mem);
            state.read_u16(addr)
        }
    }
}

#[inline]
pub fn set_source_word (state: &mut CPU, arg: u8, val: u16) {
    let mode = (arg & B_MODE) >> 6;
    let mem  = arg & B_MEM;
    match mode {
        0b11 => {
            state.set_register_u16(mem, val);
        },
        _ => {
            let addr = state.memory_address(mode, mem);
            state.write_u16(addr, val);
        }
    }
}
