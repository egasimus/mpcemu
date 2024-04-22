use crate::*;

impl CPU {

    pub fn get_register_u8 (&self, reg: u8) -> u8 {
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

    pub fn get_register_u16 (&self, reg: u8) -> u16 {
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

    pub fn get_segment_register (&self, sreg: u8) -> u16 {
        match sreg {
            0b00 => self.ds1,
            0b01 => self.ps,
            0b10 => self.ss,
            0b11 => self.ds0,
            _ => unreachable!(),
        }
    }

    pub fn set_segment_register (&mut self, sreg: u8, value: u16) {
        match sreg {
            0b00 => self.ds1 = value,
            0b01 => self.ps = value,
            0b10 => self.ss = value,
            0b11 => self.ds0 = value,
            _ => unreachable!(),
        }
    }
}

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
        $push:ident
        $pop:ident
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
                self.$w.to_le_bytes()[1]
            }
            pub fn $h_set (&mut self, value: u8) {
                self.$w = u16::from_le_bytes([self.$l(), value])
            }
            pub fn $l (&self) -> u8 {
                self.$w.to_le_bytes()[0]
            }
            pub fn $l_set (&mut self, value: u8) {
                self.$w = u16::from_le_bytes([value, self.$h()])
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
            state.set_pzs(result);
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
            state.set_pzs(result);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
        }

        #[inline]
        pub fn $push (state: &mut CPU) -> u64 {
            let data = state.$w();
            state.push_u16(data);
            if state.sp() % 2 == 0 { 5 } else { 9 }
        }

        #[inline]
        pub fn $pop (state: &mut CPU) {
            let value = state.pop_u16();
            state.$w_set(value);
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
    aw set_aw ah set_ah al set_al mov_aw_i mov_ah_i mov_al_i inc_aw dec_aw push_aw pop_aw
);

define_general_purpose_register!(
    /// General purpose register B
    ///
    /// - BW is default for:
    ///   - Data exchange (table reference)
    bw set_bw bh set_bh bl set_bl mov_bw_i mov_bh_i mov_bl_i inc_bw dec_bw push_bw pop_bw
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
    cw set_cw ch set_ch cl set_cl mov_cw_i mov_ch_i mov_cl_i inc_cw dec_cw push_cw pop_cw
);

define_general_purpose_register!(
    /// General purpose register D
    ///
    /// - DW is default for:
    ///   - Word multiplication/division
    ///   - Indirect addressing input/output
    dw set_dw dh set_dh dl set_dl mov_dw_i mov_dh_i mov_dl_i inc_dw dec_dw push_dw pop_dw
);

macro_rules! define_special_register {
    (
        $(#[$attr:meta])*
        $w:ident $w_set:ident
        $mov:ident
        $inc:ident
        $dec:ident
        $push:ident
        $pop:ident
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
            state.set_pzs(result);
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
            state.set_pzs(result);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
        }

        #[inline]
        pub fn $push (state: &mut CPU) -> u64 {
            let data = state.$w();
            state.push_u16(data);
            if state.sp() % 2 == 0 { 5 } else { 9 }
        }

        #[inline]
        pub fn $pop (state: &mut CPU) {
            let value = state.pop_u16();
            state.$w_set(value);
        }
    }
}

define_special_register!(
    /// The PS register contains the location of the program segment.
    ps  set_ps   mov_ps_i    inc_ps  dec_ps  push_ps  pop_ps
);
define_special_register!(
    /// The SS register contains the location of the stack segment.
    ss  set_ss   mov_ss_i    iec_ss  dec_ss  push_ss  pop_ss
);
define_special_register!(
    /// The DS0 register contains the location of data segment 0.
    ds0 set_ds0  mov_ds0_i   inc_ds0 dec_ds0 push_ds0 pop_ds0
);
define_special_register!(
    /// The DS1 register contains the location of data segment 1.
    ds1 set_ds1  mov_ds1_i   inc_ds1 dec_ds1 push_ds1 pop_ds1
);
define_special_register!(
    /// The stack pointer register.
    sp  set_sp   mov_sp_i    inc_sp  dec_sp  push_sp  pop_sp
);
define_special_register!(
    /// The block pointer register.
    bp  set_bp   mov_bp_i    inc_bp  dec_bp  push_bp  pop_bp
);
define_special_register!(
    /// The program counter register.
    pc  set_pc   mov_pc_i    inc_pc  dec_pc  push_pc  pop_pc
);
define_special_register!(
    /// The IX register.
    ix  set_ix   mov_ix_i    inc_ix  dec_ix  push_ix  pop_ix
);
define_special_register!(
    /// The IY register.
    iy  set_iy   mov_iy_i    inc_iy  dec_iy  push_iy  pop_iy
);
define_special_register!(
    /// The PSW (program status word) register contains flags.
    psw set_psw  mov_psw_i   inc_psw dec_psw push_psw pop_psw
);

pub fn register_name_u8 (reg: u8) -> &'static str {
    match reg {
        0b000 => "AL",
        0b001 => "CL",
        0b010 => "DL",
        0b011 => "BL",
        0b100 => "AH",
        0b101 => "CH",
        0b110 => "DH",
        0b111 => "BH",
        _ => unreachable!()
    }
}

pub fn register_name_u16 (reg: u8) -> &'static str {
    match reg {
        0b000 => "AW",
        0b001 => "CW",
        0b010 => "DW",
        0b011 => "BW",
        0b100 => "SP",
        0b101 => "BP",
        0b110 => "IX",
        0b111 => "IY",
        _ => unreachable!()
    }
}

pub fn segment_register_name (sreg: u8) -> &'static str {
    match sreg {
        0b00 => "DS1",
        0b01 => "PS",
        0b10 => "SS",
        0b11 => "DS0",
        _ => unreachable!()
    }
}
