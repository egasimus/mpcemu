use super::CPU;

macro_rules! define_general_purpose_register {
    (
        $(#[$attr:meta])*
        $w:ident $w_set:ident
        $h:ident $h_set:ident
        $l:ident $l_set:ident
    ) => {
        $(#[$attr])*
        impl CPU {
            pub fn $w (&self) -> u16 {
                self.$w
            }
            pub fn $w_set (&mut self, value: u16) {
                self.$w = value;
            }
            pub fn $h (&self) -> u8 {
                (self.$w >> 8) as u8
            }
            pub fn $h_set (&mut self, value: u8) {
                self.$w = self.$w | ((value as u16) << 8);
            }
            pub fn $l (&self) -> u8 {
                (self.$w & 0xff) as u8
            }
            pub fn $l_set (&mut self, value: u8) {
                self.$w = self.$w | value as u16;
            }
        }
    }
}

define_general_purpose_register!(
    /// General purpose register A
    ///
    /// - AW is default for:
    ///   - Word multiplication/division
    ///   - Word input/output
    ///   - Data exchange
    /// - AH is default for:
    ///   - Byte multiplication/division
    /// - AL is default for:
    ///   - Byte multiplication/division
    ///   - Byte input/output
    ///   - BCD rotate
    ///   - Data exchange
    aw set_aw ah set_ah al set_al
);

define_general_purpose_register!(
    /// General purpose register B
    ///
    /// - BW is default for:
    ///   - Data exchange (table reference)
    bw set_bw bh set_bh bl set_bl
);

define_general_purpose_register!(
    /// General purpose register C
    ///
    /// - CW is default for:
    ///   - Loop control branch
    ///   - Repeat prefix
    /// - CL is default for:
    ///   - Shift instructions
    ///   - Rotate instructions
    ///   - BCD operation
    cw set_cw ch set_ch cl set_cl
);

define_general_purpose_register!(
    /// General purpose register D
    ///
    /// - DW is default for:
    ///   - Word multiplication/division
    ///   - Indirect addressing input/output
    dw set_dw dh set_dh dl set_dl
);

macro_rules! define_special_register {
    (
        $(#[$attr:meta])*
        $w:ident $w_set:ident
    ) => {
        $(#[$attr])*
        impl CPU {
            pub fn $w (&self) -> u16 {
                self.$w
            }
            pub fn $w_set (&mut self, value: u16) {
                self.$w = value;
            }
        }
    }
}

define_special_register!(ps  set_ps);
define_special_register!(ss  set_ss);
define_special_register!(ds0 set_ds0);
define_special_register!(ds1 set_ds1);
define_special_register!(sp  set_sp);
define_special_register!(bp  set_bp);
define_special_register!(pc  set_pc);
define_special_register!(psw set_psw);
define_special_register!(ix  set_ix);
define_special_register!(iy  set_iy);

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
