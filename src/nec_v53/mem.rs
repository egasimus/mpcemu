use super::CPU;

/// Segment override
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
    /// Effective address
    pub fn ea (&self, addr: u16) -> usize {
        ((match self.segment {
            None               => self.ds0,
            Some(Segment::DS0) => self.ds0,
            Some(Segment::DS1) => self.ds1,
            Some(Segment::PS)  => self.ps,
            Some(Segment::SS)  => self.ss
        } as u32 * 0x10) + addr as u32) as usize
    }
    /// Program address
    pub fn address (&self) -> usize {
        (((self.ps as u32) * 0x10) + self.pc as u32) as usize
    }
    pub fn peek_u8 (&mut self) -> u8 {
        self.memory[self.address()]
    }
    pub fn peek_i8 (&mut self) -> i8 {
        self.memory[self.address()] as i8
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
        self.memory[self.ea(addr)]
    }
    /// Read word from effective address
    pub fn read_u16 (&mut self, addr: u16) -> u16 {
        let lo = self.read_u8(addr);
        let hi = self.read_u8(addr + 1);
        u16::from_le_bytes([lo, hi])
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
        self.output_u8(addr, (data & 0b0000000011111111) as u8);
        self.output_u8(addr + 1, (data >> 8) as u8);
    }
    /// Push a byte to the stack
    pub fn push_u8 (&mut self, data: u8) {
        if self.sp < 1 {
            panic!("stack overflow")
        }
        self.sp = self.sp - 1;
        self.memory[self.sp as usize] = data;
    }
    /// Push a word to the stack
    pub fn push_u16 (&mut self, data: u16) {
        self.push_u8((data & 0b0000000011111111) as u8);
        self.push_u8((data >> 8) as u8);
    }
    pub fn pop_u8 (&mut self) -> u8 {
        let data = self.memory[self.sp as usize];
        self.sp = self.sp + 1;
        data
    }
    pub fn pop_u16 (&mut self) -> u16 {
        let lo = self.pop_u8() as u16;
        let hi = self.pop_u8() as u16;
        hi << 8 | lo
    }
}
