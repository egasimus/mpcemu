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
        }
    }
    pub fn step (&mut self) {
        let instruction = self.read_u8();
        self.clock += execute_instruction(self, instruction);
    }
    pub fn address (&self) -> usize {
        (((self.ps as u32) * 0x10) + self.pc as u32) as usize
    }
    pub fn peek_u8 (&mut self) -> u8 {
        self.memory[self.address()]
    }
    pub fn peek_i8 (&mut self) -> i8 {
        self.memory[self.address()] as i8
    }
    pub fn read_u8 (&mut self) -> u8 {
        let byte = self.peek_u8();
        self.pc += 1;
        byte
    }
    pub fn read_i8 (&mut self) -> i8 {
        let byte = self.peek_i8();
        self.pc += 1;
        byte
    }
    pub fn read_u16 (&mut self) -> u16 {
        let lo = self.read_u8() as u16;
        let hi = self.read_u8() as u16;
        hi << 8 | lo
    }
    pub fn read_i16 (&mut self) -> i16 {
        let lo = self.read_u8();
        let hi = self.read_u8();
        i16::from_le_bytes([lo, hi])
    }
    pub fn input_u8 (&self, addr: u16) -> u8 {
        self.ports[addr as usize]
    }
    pub fn input_u16 (&self, addr: u16) -> u16 {
        let lo = self.input_u8(addr) as u16;
        let hi = self.input_u8(addr + 1) as u16;
        hi << 8 | lo
    }
    pub fn output_u8 (&mut self, addr: u16, data: u8) {
        self.ports[addr as usize] = data;
    }
    pub fn output_u16 (&mut self, addr: u16, data: u16) {
        self.output_u8(addr, (data & 0b0000000011111111) as u8);
        self.output_u8(addr + 1, (data >> 8) as u8);
    }
    pub fn push_u8 (&mut self, data: u8) {
        if self.sp < 1 {
            panic!("stack overflow")
        }
        self.sp = self.sp - 1;
        self.memory[self.sp as usize] = data;
    }
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

    pub fn v (&self) -> bool {
        (self.psw & (1 << 11)) > 0
    }
    pub fn v_on (&mut self) {
    }
    pub fn v_off (&mut self) {
    }

    pub fn dir (&mut self) {
    }
    pub fn dir_on (&mut self) {
    }
    pub fn dir_off (&mut self) {
    }

    pub fn ie (&mut self) {
    }
    pub fn ie_on (&mut self) {
    }
    pub fn ie_off (&mut self) {
    }

    pub fn brk (&mut self) {
    }
    pub fn brk_on (&mut self) {
    }
    pub fn brk_off (&mut self) {
    }

    pub fn s (&mut self) {
    }
    pub fn s_on (&mut self) {
    }
    pub fn s_off (&mut self) {
    }

    pub fn z (&mut self) -> bool {
        (self.psw & (1 << 6)) > 0
    }
    pub fn z_on (&mut self) {
        self.psw = self.psw | (1<<6)
    }
    pub fn z_off (&mut self) {
    }

    pub fn ac (&mut self) {
    }
    pub fn ac_on (&mut self) {
    }
    pub fn ac_off (&mut self) {
    }

    pub fn p (&mut self) {
    }
    pub fn p_on (&mut self) {
    }
    pub fn p_off (&mut self) {
    }

    pub fn cy (&mut self) {
    }
    pub fn cy_on (&mut self) {
    }
    pub fn cy_off (&mut self) {
    }
}
