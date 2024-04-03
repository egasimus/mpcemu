use super::CPU;

impl CPU {
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
}

pub fn to_source_register_value (state: &CPU, arg: u8) -> u16 {
    match (arg & 0b00111000) >> 3 {
        0b000 => state.aw,
        0b001 => state.cw,
        0b010 => state.dw,
        0b011 => state.bw,
        0b100 => state.sp,
        0b101 => state.bp,
        0b110 => state.ix,
        0b111 => state.iy,
        _ => unreachable!(),
    }
}

pub fn to_target_register_reference (state: &mut CPU, reg: u8) -> &mut u16 {
    match reg {
        0b000 => &mut state.aw,
        0b001 => &mut state.cw,
        0b010 => &mut state.dw,
        0b011 => &mut state.bw,
        0b100 => &mut state.sp,
        0b101 => &mut state.bp,
        0b110 => &mut state.ix,
        0b111 => &mut state.iy,
        _ => unreachable!(),
    }
}
