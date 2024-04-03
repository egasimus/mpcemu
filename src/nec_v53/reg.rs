use super::*;

macro_rules! define_general_purpose_register {
    (
        $(#[$attr:meta])*
        $w:ident $w_set:ident
        $h:ident $h_set:ident
        $l:ident $l_set:ident
        $mov_w_i:ident
        $mov_h_i:ident
        $mov_l_i:ident
        $inc:ident
        $dec:ident
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

        #[inline]
        pub fn $mov_w_i (state: &mut CPU) -> u64 {
            let word = state.next_u16();
            state.$w_set(word);
            2
        }

        #[inline]
        pub fn $mov_h_i (state: &mut CPU) -> u64 {
            let byte = state.next_u8();
            state.$h_set(byte);
            2
        }

        #[inline]
        pub fn $mov_l_i (state: &mut CPU) -> u64 {
            let byte = state.next_u8();
            state.$l_set(byte);
            2
        }

        #[inline]
        pub fn $inc (state: &mut CPU) -> u64 {
            let value = state.$w();
            let (result, unsigned_overflow) = value.overflowing_add(1);
            let (_, signed_overflow) = (value as i16).overflowing_add(1);
            state.$w_set(result);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
        }

        #[inline]
        pub fn $dec (state: &mut CPU) -> u64 {
            let value = state.$w();
            let (result, unsigned_overflow) = value.overflowing_sub(1);
            let (_, signed_overflow) = (value as i16).overflowing_sub(1);
            state.$w_set(result);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
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
    mov_aw_i mov_ah_i mov_al_i
    inc_aw dec_aw
);

define_general_purpose_register!(
    /// General purpose register B
    ///
    /// - BW is default for:
    ///   - Data exchange (table reference)
    bw set_bw bh set_bh bl set_bl
    mov_bw_i mov_bh_i mov_bl_i
    inc_bw dec_bw
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
    mov_cw_i mov_ch_i mov_cl_i
    inc_cw dec_cw
);

define_general_purpose_register!(
    /// General purpose register D
    ///
    /// - DW is default for:
    ///   - Word multiplication/division
    ///   - Indirect addressing input/output
    dw set_dw dh set_dh dl set_dl
    mov_dw_i mov_dh_i mov_dl_i
    inc_dw dec_dw
);

macro_rules! define_special_register {
    (
        $(#[$attr:meta])*
        $w:ident $w_set:ident
        $mov:ident
        $inc:ident
        $dec:ident
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

        #[inline]
        pub fn $mov (state: &mut CPU) -> u64 {
            let value = state.next_u16();
            state.$w_set(value);
            2
        }

        #[inline]
        pub fn $inc (state: &mut CPU) -> u64 {
            let value = state.$w();
            let (result, unsigned_overflow) = value.overflowing_add(1);
            let (_, signed_overflow) = (value as i16).overflowing_add(1);
            state.$w_set(result);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
        }

        #[inline]
        pub fn $dec (state: &mut CPU) -> u64 {
            let value = state.$w();
            let (result, unsigned_overflow) = value.overflowing_sub(1);
            let (_, signed_overflow) = (value as i16).overflowing_sub(1);
            state.$w_set(result);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
        }
    }
}

define_special_register!(
    /// The PS register contains the location of the program segment.
    ps  set_ps   mov_ps      inc_ps  dec_ps
);
define_special_register!(
    /// The SS register contains the location of the stack segment.
    ss  set_ss   mov_ss      iec_ss  dec_ss
);
define_special_register!(
    /// The DS0 register contains the location of data segment 0.
    ds0 set_ds0  mov_ds0_i   inc_ds0 dec_ds0
);
define_special_register!(
    /// The DS1 register contains the location of data segment 1.
    ds1 set_ds1  mov_ds1_i   inc_ds1 dec_ds1
);
define_special_register!(
    /// The stack pointer register.
    sp  set_sp   mov_sp_i    inc_sp  dec_sp
);
define_special_register!(
    /// The block pointer register.
    bp  set_bp   mov_bp_i    inc_bp  dec_bp
);
define_special_register!(
    /// The program counter register.
    pc  set_pc   mov_pc_i    inc_pc  dec_pc
);
define_special_register!(
    /// The IX register.
    ix  set_ix   mov_ix_i    inc_ix  dec_ix
);
define_special_register!(
    /// The IY register.
    iy  set_iy   mov_iy_i    inc_iy  dec_iy
);
define_special_register!(
    /// The PSW (program status word) register contains flags.
    psw set_psw  mov_psw_i   inc_psw dec_psw
);

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

#[inline]
pub fn clr1_cy (state: &mut CPU) -> u64 {
    state.set_cy(false);
    2
}

#[inline]
pub fn set1_cy (state: &mut CPU) -> u64 {
    state.set_cy(true);
    2
}

#[inline]
pub fn clr1_dir (state: &mut CPU) -> u64 {
    state.set_dir(false);
    2
}

#[inline]
pub fn set1_dir (state: &mut CPU) -> u64 {
    state.set_dir(true);
    2
}
