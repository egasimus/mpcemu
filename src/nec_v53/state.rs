use super::execute_instruction;

pub struct State {
    pub clock:   u64,
    pub memory:  Vec<u8>,
    pub ports:   Vec<u8>,
    /// General purpose register A, whole word
    /// Default for:
    /// - Word multiplication/division
    /// - Word input/output
    /// - Data exchange
    pub aw:      u16,
    /// General purpose register B, whole word
    /// Default for:
    /// - Data exchange (table reference)
    pub bw:      u16,
    /// General purpose register C, whole word
    /// Default for:
    /// - Loop control branch
    /// - Repeat prefix
    pub cw:      u16,
    /// General purpose register D, whole word
    /// Default for:
    /// - Word multiplication/division
    /// - Indirect addressing input/output
    pub dw:      u16,
    pub ps:      u16,
    pub ss:      u16,
    pub ds0:     u16,
    pub ds1:     u16,
    pub sp:      u16,
    pub bp:      u16,
    pub pc:      u16,
    pub psw:     u16,
    pub ix:      u16,
    pub iy:      u16,
    pub segment: Option<Segment>,
}

impl State {
    pub fn new () -> Self {
        Self {
            clock:   0x0000,
            memory:  vec![0x00;65536],
            ports:   vec![0x00;65536],
            aw:      0x0000,
            bw:      0x0000,
            cw:      0x0000,
            dw:      0x0000,
            ps:      0xffff,
            ss:      0x0000,
            ds0:     0x0000,
            ds1:     0x0000,
            sp:      0x0000,
            bp:      0x0000,
            pc:      0x0000,
            psw:     0b1111000000000010,
            ix:      0x0000,
            iy:      0x0000,
            segment: None,
        }
    }
    /// Read and execute the next instruction in the program
    pub fn step (&mut self) {
        let opcode = self.next_u8();
        self.clock += execute_instruction(self, opcode);
        // Reset segment override, except if it was just set:
        if !((opcode == 0x26) || (opcode == 0x2E) || (opcode == 0x36) || (opcode == 0x3E)) {
            self.segment = None
        }
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

    /// General purpose register A, high byte
    /// Default for:
    /// - Byte multiplication/division
    pub fn ah (&self) -> u8 {
        (self.aw >> 8) as u8
    }
    /// General purpose register B, high byte
    pub fn bh (&self) -> u8 {
        (self.bw >> 8) as u8
    }
    /// General purpose register C, high byte
    pub fn ch (&self) -> u8 {
        (self.cw >> 8) as u8
    }
    /// General purpose register D, high byte
    pub fn dh (&self) -> u8 {
        (self.dw >> 8) as u8
    }

    /// General purpose register A, low byte
    /// - Byte multiplication/division
    /// - Byte input/output
    /// - BCD rotate
    /// - Data exchange
    pub fn al (&self) -> u8 {
        (self.aw & 0xff) as u8
    }
    /// General purpose register B, low byte
    pub fn bl (&self) -> u8 {
        (self.bw & 0xff) as u8
    }
    /// General purpose register C, low byte
    /// Default for:
    /// - Shift instructions
    /// - Rotate instructions
    /// - BCD operation
    pub fn cl (&self) -> u8 {
        (self.cw & 0xff) as u8
    }
    /// General purpose register D, low byte
    pub fn dl (&self) -> u8 {
        (self.dw & 0xff) as u8
    }

    pub fn set_ah (&mut self, value: u8) {
        self.aw = self.aw | ((value as u16) << 8);
    }
    pub fn set_bh (&mut self, value: u8) {
        self.bw = self.bw | ((value as u16) << 8);
    }
    pub fn set_ch (&mut self, value: u8) {
        self.cw = self.cw | ((value as u16) << 8);
    }
    pub fn set_dh (&mut self, value: u8) {
        self.dw = self.dw | ((value as u16) << 8);
    }

    pub fn set_al (&mut self, value: u8) {
        self.aw = self.aw | value as u16;
    }
    pub fn set_bl (&mut self, value: u8) {
        self.bw = self.bw | value as u16;
    }
    pub fn set_cl (&mut self, value: u8) {
        self.cw = self.cw | value as u16;
    }
    pub fn set_dl (&mut self, value: u8) {
        self.dw = self.dw | value as u16;
    }

    /// Overflow flag.
    pub fn v (&self) -> bool {
        (self.psw & (1 << 11)) > 0
    }
    /// Set overflow flag:
    ///
    /// - TODO
    pub fn v_on (&mut self) {
        self.psw = self.psw | (1<<11)
    }
    /// Reset overflow flag:
    ///
    /// - TODO
    pub fn v_off (&mut self) {
        self.psw = self.psw & (1<<11)
    }

    /// Direction flag.
    pub fn dir (&mut self) -> bool {
        (self.psw & (1 << 10)) > 0
    }
    /// Set direction flag:
    ///
    /// - TODO
    pub fn dir_on (&mut self) {
        self.psw = self.psw | (1<<10)
    }
    /// Reset direction flag:
    ///
    /// - TODO
    pub fn dir_off (&mut self) {
        self.psw = self.psw & (1<<10)
    }

    /// Interrupt enable flag.
    pub fn ie (&mut self) -> bool {
        (self.psw & (1 << 9)) > 0
    }
    /// Set interrupt enable flag:
    ///
    /// - TODO
    pub fn ie_on (&mut self) {
        self.psw = self.psw | (1<<9)
    }
    /// Reset interrupt enable flag:
    ///
    /// - TODO
    pub fn ie_off (&mut self) {
        self.psw = self.psw & (1<<9)
    }

    /// Break flag
    pub fn brk (&mut self) -> bool {
        (self.psw & (1 << 8)) > 0
    }
    /// Set break flag:
    ///
    /// - TODO
    pub fn brk_on (&mut self) {
        self.psw = self.psw | (1<<8)
    }
    /// Clear break flag:
    ///
    /// - TODO
    pub fn brk_off (&mut self) {
        self.psw = self.psw & (1<<8)
    }

    /// Sign flag
    pub fn s (&mut self) -> bool {
        (self.psw & (1 << 7)) > 0
    }
    /// Set sign flag:
    ///
    /// - TODO
    pub fn s_on (&mut self) {
        self.psw = self.psw | (1<<7)
    }
    /// Reset sign flag:
    ///
    /// - TODO
    pub fn s_off (&mut self) {
        self.psw = self.psw & (1<<7)
    }

    /// Zero flag
    pub fn z (&mut self) -> bool {
        (self.psw & (1 << 6)) > 0
    }
    /// Set zero flag:
    ///
    /// - TODO
    pub fn z_on (&mut self) {
        self.psw = self.psw | (1<<6)
    }
    /// Reset zero flag:
    ///
    /// - TODO
    pub fn z_off (&mut self) {
        self.psw = self.psw & (1<<6)
    }

    /// Auxiliary carry flag.
    pub fn ac (&mut self) -> bool {
        (self.psw & (1 << 4)) > 0
    }
    /// Set auxiliary carry flag:
    ///
    /// - TODO
    pub fn ac_on (&mut self) {
        self.psw = self.psw | (1<<4)
    }
    /// Reset auxiliary carry flag:
    ///
    /// - TODO
    pub fn ac_off (&mut self) {
        self.psw = self.psw & (1<<4)
    }

    /// Parity flag.
    pub fn p (&mut self) -> bool {
        (self.psw & (1 << 2)) > 0
    }
    /// Set parity flag:
    ///
    /// - After binary add/sub, logical op, shift,
    ///   if there are zero or an even number of 1s
    ///   in the result.
    pub fn p_on (&mut self) {
        self.psw = self.psw | (1<<2)
    }
    /// Reset parity flag:
    ///
    /// - After binary add/sub, logical op, shift,
    ///   if there is an odd number of 1s in the result.
    pub fn p_off (&mut self) {
        self.psw = self.psw & !(1<<2)
    }

    /// Carry flag.
    pub fn cy (&mut self) -> bool {
        (self.psw & (1 << 0)) > 0
    }
    /// Set carry flag:
    ///
    /// - After byte add/sub if carry/borrow occurs from bit 7
    /// - After word add/sub if carry/borrow occurs from bit 15
    /// - After unsigned byte mul if AH is not 0
    /// - After signed byte mul of AH does not sign-extend AL
    /// - After unsigned word mul if DW is not 0
    /// - After signed word mul if DW does not sign-extend AW
    /// - After 8-bit immediate mul if product exceeds 16 bits
    /// - If a bit shifted/rotated to CY is 1
    pub fn cy_on (&mut self) {
        self.psw = self.psw | (1<<0)
    }
    /// Reset carry flag:
    ///
    /// - After byte add/sub if carry/borrow does not occur from bit 7
    /// - After word add/sub if carry/borrow does not occur from bit 15
    /// - Always every logical operation
    /// - After unsigned byte multiplication if AH is 0
    /// - After signed byte multiplication of AH sign-extends AL
    /// - After unsigned word mul if DW is 0
    /// - After signed word mul if DW sign-extends AW
    /// - After 8-bit immediate mul if product is within 16 bits
    /// - If a bit shifted/rotated to CY is 0
    pub fn cy_off (&mut self) {
        self.psw = self.psw & !(1<<0)
    }
}

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
