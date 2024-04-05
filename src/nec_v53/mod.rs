/// <https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF>

mod bit;
mod reg;
mod flag;
mod mem;
mod math;
mod shift;
mod inst;
#[cfg(test)] mod test;

pub use self::{
    bit::*,
    reg::*,
    flag::*,
    mem::*,
    math::*,
    shift::*,
    inst::*,
};

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
    pub fn step (&mut self) {
        let opcode = self.next_u8();
        self.opcode = opcode;
        self.clock += execute_instruction(self, opcode);
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
            0b00 => match mem {
                0b000 => self.bw() + self.ix(),
                0b001 => self.bw() + self.iy(),
                0b010 => self.bp() + self.ix(),
                0b011 => self.bp() + self.iy(),
                0b100 => self.ix(),
                0b101 => self.iy(),
                0b110 => unimplemented!("direct address"),
                0b111 => self.bw(),
                _ => panic!("invalid memory inner mode {:b}", mem)
            },
            0b01 => {
                let displace = self.next_u8() as u16;
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
            },
            0b10 => {
                let displace = self.next_u16();
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
            },
            _ => panic!("invalid memory outer mode {:b}", mode)
        }
    }

}
