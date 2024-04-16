/// <https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF>

mod bit;
mod reg;
mod flag;
mod inst;
#[cfg(test)] mod test;

pub(crate) use self::{bit::*, reg::*, flag::*, inst::*};

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
    opcode: u8,
    pub clock: u64,
}

/// Segment override
#[derive(Debug)]
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
        }
    }

    /// Read and execute the next instruction in the program
    pub fn step (&mut self, debug: bool) {
        let addr   = self.program_address();
        let pc     = self.pc();
        let opcode = self.next_u8();
        self.opcode = opcode;
        let (name, bytes, instruction) = v53_instruction(self, opcode);
        if debug {
            print!("\n           AW   BW   CW   DW   DS0  DS1  BP   IX   IY   SS   SP   PS   PC   ");
            print!("\n           {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}:{:04X} {:04X}:{:04X}",
                self.aw(), self.bw(), self.cw(), self.dw(),
                self.ds0(), self.ds1(), self.bp(), self.ix(), self.iy(),
                self.ss(), self.sp(), self.ps(), pc);
            print!("\n           V={} DIR={} IE={} BRK={} S={} Z={} AC={} P={} CY={}",
                self.v() as u8, self.dir() as u8, self.ie() as u8, self.brk() as u8,
                self.s() as u8, self.z() as u8, self.ac() as u8, self.p() as u8,
                self.cy() as u8);
            self.dump_stack(4);
            print!("\n\n{:10} {addr:05X}  {name:15}  {:02X?}\n",
                self.clock, &bytes);
        }
        self.clock += instruction(self);
        // Reset segment override, except if it was just set:
        if !((opcode == 0x26) || (opcode == 0x2E) || (opcode == 0x36) || (opcode == 0x3E)) {
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

    pub fn register_value_u8 (&self, reg: u8) -> u8 {
        match reg {
            0b000 => self.al(),
            0b001 => self.cl(),
            0b010 => self.dl(),
            0b011 => self.bl(),
            0b100 => self.ah(),
            0b101 => self.ch(),
            0b110 => self.dh(),
            0b111 => self.bh(),
            _ => unreachable!(),
        }
    }

    pub fn register_value_u16 (&self, reg: u8) -> u16 {
        match reg {
            0b000 => self.aw(),
            0b001 => self.cw(),
            0b010 => self.dw(),
            0b011 => self.bw(),
            0b100 => self.sp(),
            0b101 => self.bp(),
            0b110 => self.ix(),
            0b111 => self.iy(),
            _ => unreachable!(),
        }
    }

    pub fn set_register_u8 (&mut self, reg: u8, value: u8) {
        match reg {
            0b000 => self.set_al(value),
            0b001 => self.set_cl(value),
            0b010 => self.set_dl(value),
            0b011 => self.set_bl(value),
            0b100 => self.set_ah(value),
            0b101 => self.set_ch(value),
            0b110 => self.set_dh(value),
            0b111 => self.set_bh(value),
            _ => unreachable!(),
        }
    }

    pub fn set_register_u16 (&mut self, reg: u8, value: u16) {
        match reg {
            0b000 => self.set_aw(value),
            0b001 => self.set_cw(value),
            0b010 => self.set_dw(value),
            0b011 => self.set_bw(value),
            0b100 => self.set_sp(value),
            0b101 => self.set_bp(value),
            0b110 => self.set_ix(value),
            0b111 => self.set_iy(value),
            _ => unreachable!(),
        }
    }

    pub fn register_reference_u16 (&mut self, reg: u8) -> &mut u16 {
        match reg {
            0b000 => &mut self.aw,
            0b001 => &mut self.cw,
            0b010 => &mut self.dw,
            0b011 => &mut self.bw,
            0b100 => &mut self.sp,
            0b101 => &mut self.bp,
            0b110 => &mut self.ix,
            0b111 => &mut self.iy,
            _ => unreachable!(),
        }
    }

    pub fn segment_register_value (&self, sreg: u8) -> u16 {
        match sreg {
            0b00 => self.ds1,
            0b01 => self.ps,
            0b10 => self.ss,
            0b11 => self.ds0,
            _ => unreachable!(),
        }
    }

    pub fn segment_register_reference (&mut self, sreg: u8) -> &mut u16 {
        match sreg {
            0b00 => &mut self.ds1,
            0b01 => &mut self.ps,
            0b10 => &mut self.ss,
            0b11 => &mut self.ds0,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn memory_address (&mut self, mode: u8, mem: u8) -> u16 {
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
    pub fn memory_address_00 (&mut self, mem: u8) -> u16 {
        match mem {
            0b000 => self.bw() + self.ix(),
            0b001 => self.bw() + self.iy(),
            0b010 => self.bp() + self.ix(),
            0b011 => self.bp() + self.iy(),
            0b100 => self.ix(),
            0b101 => self.iy(),
            0b110 => self.next_u16(),
            0b111 => self.bw(),
            _ => panic!("invalid memory inner mode {:b}", mem)
        }
    }

    #[inline]
    pub fn memory_address_01 (&self, mem: u8, displace: u8) -> u16 {
        match mem {
            0b000 => self.bw() + self.ix() + displace as u16,
            0b001 => self.bw() + self.iy() + displace as u16,
            0b010 => self.bp() + self.ix() + displace as u16,
            0b011 => self.bp() + self.iy() + displace as u16,
            0b100 => self.ix() + displace as u16,
            0b101 => self.iy() + displace as u16,
            0b110 => self.bp() + displace as u16,
            0b111 => self.bw() + displace as u16,
            _ => panic!("invalid memory inner mode {:b}", mem)
        }
    }

    #[inline]
    pub fn memory_address_10 (&self, mem: u8, displace: u16) -> u16 {
        match mem {
            0b000 => self.bw() + self.ix() + displace,
            0b001 => self.bw() + self.iy() + displace,
            0b010 => self.bp() + self.ix() + displace,
            0b011 => self.bp() + self.iy() + displace,
            0b100 => self.ix() + displace,
            0b101 => self.iy() + displace,
            0b110 => self.bp() + displace,
            0b111 => self.bw() + displace,
            _ => panic!("invalid memory inner mode {:b}", mem)
        }
    }

    pub fn dump (&self) {
        let start = self.ps.saturating_sub(4);
        let end   = self.ps.saturating_add(4);
        for row in start..=end {
            print!("\n{}{:6X}|", if row == self.ps {">"} else {" "}, row);
            for col in 0..0x10 {
                print!(" {:02x}", self.memory[row as usize * 0x10 + col]);
            }
        }
    }

    pub fn dump_stack (&self, rows: usize) {
        self.dump_segment(self.ss(), self.sp(), 4)
    }

    pub fn dump_segment (&self, segment: u16, offset: u16, count: u16) {
        for row in 0..count {
            let start = (((segment as usize * 0x10) + offset as usize) / 0x10 + row as usize) * 0x10;
            print!("\n{:6X}|", start);
            for col in 0..0x10 {
                print!(" {:02x}", self.memory()[start + col]);
            }
        }
    }

    pub fn dump_at (&self, start: usize, per_row: u8, rows: u8) {
        for i in 0..rows {
            let offset = start + i as usize * per_row as usize;
            print!("\n{:6X}|", offset);
            for j in 0..per_row {
                print!(" {:02x}", self.memory()[offset as usize + j as usize]);
            }
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

    pub fn get_byte (&self, addr: usize) -> u8 {
        if addr < 0xA0000 {
            if self.xa() {
                self.extended[addr]
            } else {
                self.memory[addr]
            }
        } else {
            self.memory[addr]
        }
    }

    pub fn set_byte (&mut self, addr: usize, value: u8) {
        if addr < 0xA0000 {
            if self.xa() {
                self.extended[addr] = value
            } else {
                self.memory[addr] = value
            }
        } else {
            self.memory[addr] = value
        }
    }

    /// Program address
    pub fn program_address (&self) -> usize {
        (((self.ps as u32) * 0x10) + self.pc as u32) as usize
    }

    /// Effective address
    pub fn effective_address (&self, addr: u16) -> usize {
        let segment = match self.segment {
            None               => self.ds0,
            Some(Segment::DS0) => self.ds0,
            Some(Segment::DS1) => self.ds1,
            Some(Segment::PS)  => self.ps,
            Some(Segment::SS)  => self.ss
        } as u32 * 0x10;

        (segment + addr as u32) as usize
    }

    /// Target address (always offset from DS1)
    pub fn ds1_address (&self, addr: u16) -> usize {
        ((self.ds1 as u32 * 0x10) + addr as u32) as usize
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
    pub fn read_u8 (&mut self, addr: u16) -> u8 {
        self.get_byte(self.effective_address(addr))
    }

    /// Read word from effective address
    pub fn read_u16 (&mut self, addr: u16) -> u16 {
        let lo = self.read_u8(addr);
        let hi = self.read_u8(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    /// Write byte to effective address
    pub fn write_u8 (&mut self, addr: u16, value: u8) {
        let ea = self.effective_address(addr);
        self.set_byte(ea, value);
    }

    /// Write word to effective address
    pub fn write_u16 (&mut self, addr: u16, value: u16) {
        let ea = self.effective_address(addr);
        let [lo, hi] = value.to_le_bytes();
        self.set_byte(ea + 0, lo);
        self.set_byte(ea + 1, hi);
    }

    /// Read byte from input port
    pub fn input_u8 (&self, addr: u16) -> u8 {
        self.ports[addr as usize]
    }

    /// Read word from input port
    pub fn input_u16 (&self, addr: u16) -> u16 {
        let lo = self.input_u8(addr) as u16;
        let hi = self.input_u8(addr + 1) as u16;
        hi << 8 | lo
    }

    /// Write byte to input port
    pub fn output_u8 (&mut self, addr: u16, data: u8) {
        self.ports[addr as usize] = data;
    }

    /// Write byte to output port
    pub fn output_u16 (&mut self, addr: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.output_u8(addr + 0, lo);
        self.output_u8(addr + 1, hi);
    }

    /// Push a byte to the stack
    pub fn push_u8 (&mut self, data: u8) {
        if self.sp < 1 {
            panic!("stack overflow")
        }
        self.sp = self.sp - 1;
        self.set_byte(self.sp as usize, data);
    }

    /// Push a word to the stack
    pub fn push_u16 (&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.push_u8(lo);
        self.push_u8(hi);
    }

    pub fn pop_u8 (&mut self) -> u8 {
        let data = self.get_byte(self.sp as usize);
        self.sp = self.sp + 1;
        data
    }

    pub fn pop_u16 (&mut self) -> u16 {
        let lo = self.pop_u8() as u16;
        let hi = self.pop_u8() as u16;
        hi << 8 | lo
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
        0b11 => state.register_value_u16(mem),
        _ => {
            let addr = state.memory_address(mode, mem);
            state.read_u16(addr)
        }
    }
}

#[inline]
pub fn set_source_word (state: &mut CPU, arg: u8, val: u16){
    let mode = (arg & B_MODE) >> 6;
    let mem  = arg & B_MEM;
    match mode {
        0b11 => {
            *state.register_reference_u16(mem) = val;
        },
        _ => {
            let addr = state.memory_address(mode, mem);
            state.write_u16(addr, val);
        }
    }
}
