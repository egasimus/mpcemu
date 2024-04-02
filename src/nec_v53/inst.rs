use super::State;

macro_rules! define_instruction_set (

    ($([$code:literal, $inst:literal, $info:literal, $impl:ident],)+$(,)?) => {

        #[allow(unused)]
        pub fn get_instruction_name (code: u8) -> &'static str {
            match code {
                $($code => $inst),+
            }
        }

        #[allow(unused)]
        pub fn get_instruction_description (code: u8) -> &'static str {
            match code {
                $($code => $info),+
            }
        }

        #[allow(unused)]
        pub fn execute_instruction (state: &mut State, code: u8) -> u64 {
            match code {
                $($code => $impl(state)),+
            }
        }

    }

);

define_instruction_set! {
    [0x00, "ADD", "8-bit add to memory from register",       add_b_f_rm],
    [0x01, "ADD", "16-bit add to memory from register",      add_w_f_rm],
    [0x02, "ADD", "8-bit add to register from memory",       add_b_t_rm],
    [0x03, "ADD", "16-bit add to register from memory",      add_w_t_rm],
    [0x04, "ADD", "8-bit add to accumulator from constant",  add_b_ia],
    [0x05, "ADD", "16-bit add to accumulator from constant", add_w_ia],
    [0x06, "", "", unimplemented],
    [0x07, "", "", unimplemented],
    [0x08, "", "", unimplemented],
    [0x09, "", "", unimplemented],
    [0x0A, "", "", unimplemented],
    [0x0B, "", "", unimplemented],
    [0x0C, "", "", unimplemented],
    [0x0D, "", "", unimplemented],
    [0x0E, "", "", unimplemented],
    [0x0F, "GROUP3", "See Group 3", unimplemented],

    [0x10, "", "", unimplemented],
    [0x11, "", "", unimplemented],
    [0x12, "", "", unimplemented],
    [0x13, "", "", unimplemented],
    [0x14, "", "", unimplemented],
    [0x15, "", "", unimplemented],
    [0x16, "", "", unimplemented],
    [0x17, "", "", unimplemented],
    [0x18, "", "", unimplemented],
    [0x19, "", "", unimplemented],
    [0x1A, "", "", unimplemented],
    [0x1B, "", "", unimplemented],
    [0x1C, "", "", unimplemented],
    [0x1D, "", "", unimplemented],
    [0x1E, "", "", unimplemented],
    [0x1F, "", "", unimplemented],

    [0x20, "", "", unimplemented],
    [0x21, "", "", unimplemented],
    [0x22, "", "", unimplemented],
    [0x23, "", "", unimplemented],
    [0x24, "", "", unimplemented],
    [0x25, "", "", unimplemented],
    [0x26, "", "", unimplemented],
    [0x27, "", "", unimplemented],
    [0x28, "", "", unimplemented],
    [0x29, "", "", unimplemented],
    [0x2A, "", "", unimplemented],
    [0x2B, "", "", unimplemented],
    [0x2C, "", "", unimplemented],
    [0x2D, "", "", unimplemented],
    [0x2E, "PS:", "", unimplemented],
    [0x2F, "", "", unimplemented],

    [0x30, "", "", unimplemented],
    [0x31, "", "", unimplemented],
    [0x32, "", "", unimplemented],
    [0x33, "", "", unimplemented],
    [0x34, "", "", unimplemented],
    [0x35, "", "", unimplemented],
    [0x36, "", "", unimplemented],
    [0x37, "", "", unimplemented],
    [0x38, "", "", unimplemented],
    [0x39, "", "", unimplemented],
    [0x3A, "", "", unimplemented],
    [0x3B, "", "", unimplemented],
    [0x3C, "", "", unimplemented],
    [0x3D, "CMP", "w, ia", unimplemented],
    [0x3E, "", "", unimplemented],
    [0x3F, "", "", unimplemented],

    [0x40, "", "", unimplemented],
    [0x41, "", "", unimplemented],
    [0x42, "INC", "Increment", unimplemented],
    [0x43, "", "", unimplemented],
    [0x44, "", "", unimplemented],
    [0x45, "", "", unimplemented],
    [0x46, "", "", unimplemented],
    [0x47, "", "", unimplemented],
    [0x48, "DEC AW", "Decrement AW by 1", dec_aw],
    [0x49, "DEC CW", "Decrement CW by 1", dec_cw],
    [0x4A, "DEC DW", "Decrement DW by 1", dec_dw],
    [0x4B, "DEC BW", "Decrement BW by 1", dec_bw],
    [0x4C, "", "", unimplemented],
    [0x4D, "", "", unimplemented],
    [0x4E, "", "", unimplemented],
    [0x4F, "", "", unimplemented],

    [0x50, "", "", unimplemented],
    [0x51, "", "", unimplemented],
    [0x52, "", "", unimplemented],
    [0x53, "", "", unimplemented],
    [0x54, "", "", unimplemented],
    [0x55, "", "", unimplemented],
    [0x56, "", "", unimplemented],
    [0x57, "", "", unimplemented],
    [0x58, "", "", unimplemented],
    [0x59, "", "", unimplemented],
    [0x5A, "", "", unimplemented],
    [0x5B, "", "", unimplemented],
    [0x5C, "", "", unimplemented],
    [0x5D, "", "", unimplemented],
    [0x5E, "RET", "Return from subroutine", unimplemented],
    [0x5F, "", "", unimplemented],

    [0x60, "", "", unimplemented],
    [0x61, "", "", unimplemented],
    [0x62, "", "", unimplemented],
    [0x63, "", "", unimplemented],
    [0x64, "", "", unimplemented],
    [0x65, "", "", unimplemented],
    [0x66, "", "", unimplemented],
    [0x67, "", "", unimplemented],
    [0x68, "", "", unimplemented],
    [0x69, "", "", unimplemented],
    [0x6A, "", "", unimplemented],
    [0x6B, "", "", unimplemented],
    [0x6C, "", "", unimplemented],
    [0x6D, "", "", unimplemented],
    [0x6E, "OUTM", "8-bit",  unimplemented],
    [0x6F, "OUTM", "16-bit", unimplemented],

    [0x70, "", "", unimplemented],
    [0x71, "", "", unimplemented],
    [0x72, "", "", unimplemented],
    [0x73, "", "", unimplemented],
    [0x74, "BE",  "",                      unimplemented],
    [0x75, "BNE", "Branch if Z flag is 0", bne],
    [0x76, "", "", unimplemented],
    [0x77, "", "", unimplemented],
    [0x78, "", "", unimplemented],
    [0x79, "", "", unimplemented],
    [0x7A, "", "", unimplemented],
    [0x7B, "", "", unimplemented],
    [0x7C, "", "", unimplemented],
    [0x7D, "", "", unimplemented],
    [0x7E, "", "", unimplemented],
    [0x7F, "", "", unimplemented],

    [0x80, "", "", unimplemented],
    [0x81, "", "", unimplemented],
    [0x82, "", "", unimplemented],
    [0x83, "", "", unimplemented],
    [0x84, "", "", unimplemented],
    [0x85, "", "", unimplemented],
    [0x86, "", "", unimplemented],
    [0x87, "", "", unimplemented],
    [0x88, "", "", unimplemented],
    [0x89, "", "", unimplemented],
    [0x8A, "", "", unimplemented],
    [0x8B, "", "", unimplemented],
    [0x8C, "", "", unimplemented],
    [0x8D, "", "", unimplemented],
    [0x8E, "", "", unimplemented],
    [0x8F, "", "", unimplemented],

    [0x90, "NOP",  "Do nothing",        nop],
    [0x91, "", "", unimplemented],
    [0x92, "", "", unimplemented],
    [0x93, "", "", unimplemented],
    [0x94, "", "", unimplemented],
    [0x95, "", "", unimplemented],
    [0x96, "", "", unimplemented],
    [0x97, "", "", unimplemented],
    [0x98, "", "", unimplemented],
    [0x99, "", "", unimplemented],
    [0x9A, "CALL", "Call a subroutine", unimplemented],
    [0x9B, "", "", unimplemented],
    [0x9C, "", "", unimplemented],
    [0x9D, "", "", unimplemented],
    [0x9E, "", "", unimplemented],
    [0x9F, "", "", unimplemented],

    [0xA0, "MOV", "Move 8-bit value into AL from memory",  mov_al_m],
    [0xA1, "MOV", "Move 16-bit value into AW from memory", mov_aw_m],
    [0xA2, "MOV", "Move 8-bit value into memory from AL",  mov_m_al],
    [0xA3, "MOV", "Move 16-bit value into memory from AW", mov_m_aw],
    [0xA4, "", "", unimplemented],
    [0xA5, "", "", unimplemented],
    [0xA6, "", "", unimplemented],
    [0xA7, "", "", unimplemented],
    [0xA8, "", "", unimplemented],
    [0xA9, "", "", unimplemented],
    [0xAA, "", "", unimplemented],
    [0xAB, "", "", unimplemented],
    [0xAC, "", "", unimplemented],
    [0xAD, "", "", unimplemented],
    [0xAE, "", "", unimplemented],
    [0xAF, "", "", unimplemented],
    
    [0xB0, "MOV", "Move 8-bit constant into AL",  mov_al_i],
    [0xB1, "MOV", "Move 8-bit constant into CL",  mov_cl_i],
    [0xB2, "MOV", "Move 8-bit constant into DL",  mov_dl_i],
    [0xB3, "MOV", "Move 8-bit constant into BL",  mov_bl_i],
    [0xB4, "MOV", "Move 8-bit constant into AH",  mov_ah_i],
    [0xB5, "MOV", "Move 8-bit constant into CH",  mov_ch_i],
    [0xB6, "MOV", "Move 8-bit constant into BH",  mov_bh_i],
    [0xB7, "MOV", "Move 8-bit constant into DH",  mov_dh_i],
    [0xB8, "MOV", "Move 16-bit constant into AW", mov_aw_i],
    [0xB9, "MOV", "Move 16-bit constant into CW", mov_cw_i],
    [0xBA, "MOV", "Move 16-bit constant into DW", mov_dw_i],
    [0xBB, "MOV", "Move 16-bit constant into BW", mov_bw_i],
    [0xBC, "MOV", "Move 16-bit constant into SP", mov_sp_i],
    [0xBD, "MOV", "Move 16-bit constant into BP", mov_bp_i],
    [0xBE, "MOV", "Move 16-bit constant into IX", mov_ix_i],
    [0xBF, "MOV", "Move 16-bit constant into IY", mov_iy_i],

    [0xC0, "",        "", unimplemented],
    [0xC1, "",        "", unimplemented],
    [0xC2, "",        "", unimplemented],
    [0xC3, "",        "", unimplemented],
    [0xC4, "MOV",     "Move 16-bit value to DS1 from AW", mov_ds1_aw],
    [0xC5, "MOV",     "Move 16-bit value to DS0 from AW", mov_ds0_aw],
    [0xC6, "MOV",     "Move 8-bit constant to memory",    mov_mb_imm],
    [0xC7, "MOV",     "Move 16-bit constant to memory",   mov_mw_imm],
    [0xC8, "",        "", unimplemented],
    [0xC9, "DISPOSE", "Delete a stack frame", unimplemented],
    [0xCA, "",        "", unimplemented],
    [0xCB, "",        "", unimplemented],
    [0xCC, "",        "", unimplemented],
    [0xCD, "",        "", unimplemented],
    [0xCE, "",        "", unimplemented],
    [0xCF, "RETI",    "Return from interrupt, restoring PC, PS, and PSW", unimplemented],

    [0xD0, "", "", unimplemented],
    [0xD1, "", "", unimplemented],
    [0xD2, "", "", unimplemented],
    [0xD3, "", "", unimplemented],
    [0xD4, "", "", unimplemented],
    [0xD5, "", "", unimplemented],
    [0xD6, "", "", unimplemented],
    [0xD7, "", "", unimplemented],
    [0xD8, "", "", unimplemented],
    [0xD9, "", "", unimplemented],
    [0xDA, "", "", unimplemented],
    [0xDB, "", "", unimplemented],
    [0xDC, "", "", unimplemented],
    [0xDD, "", "", unimplemented],
    [0xDE, "", "", unimplemented],
    [0xDF, "", "", unimplemented],

    [0xE0, "DBNZE", "",                                    unimplemented],
    [0xE1, "DBNZE", "",                                    unimplemented],
    [0xE2, "DBNZ",  "Decrement CW and branch if not zero", dbnz],
    [0xE3, "BCWZ",  "",                                    unimplemented],
    [0xE4, "IN",    "b",                                   in_b],
    [0xE5, "IN",    "w",                                   in_w],
    [0xE6, "OUT",   "b",                                   out_b],
    [0xE7, "OUT",   "w",                                   out_w],
    [0xE8, "CALL",  "Call direct address",                 call_d],
    [0xE9, "BR",    "Branch near",                         br_near],
    [0xEA, "BR",    "Branch far",                          br_far],
    [0xEB, "BR",    "Branch si, direct address",           unimplemented],
    [0xEC, "IN",    "b, v",                                in_b_v],
    [0xED, "IN",    "w, v",                                in_w_v],
    [0xEE, "OUT",   "b, v",                                out_b_v],
    [0xEF, "OUT",   "w, v",                                out_w_v],

    [0xF0, "",     "",                                             unimplemented],
    [0xF1, "",     "",                                             unimplemented],
    [0xF2, "",     "",                                             unimplemented],
    [0xF3, "",     "",                                             unimplemented],
    [0xF4, "",     "",                                             unimplemented],
    [0xF5, "",     "",                                             unimplemented],
    [0xF6, "",     "",                                             unimplemented],
    [0xF7, "",     "",                                             unimplemented],
    [0xF8, "CLR1", "Clear carry flag",                             clr1_cy],
    [0xF9, "SET1", "Set carry flag",                               set1_cy],
    [0xFA, "DI",   "Reset IE flag and disable maskable interrupt", di],
    [0xFB, "EI",   "Set IE flag and enable maskable interrupt",    unimplemented],
    [0xFC, "CLR1", "Clear direction flag",                         clr1_dir],
    [0xFD, "SET1", "Set direction flag",                           set1_dir],
    [0xFE, "",     "",                                             unimplemented],
    [0xFF, "",     "",                                             unimplemented],

}

#[inline]
fn nop (state: &mut State) -> u64 {
    1
}

#[inline]
fn unimplemented (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn add_b_f_rm (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn add_w_f_rm (state: &mut State) -> u64 {
    let addr     = state.read_u8();
    let memory   = (addr & 0b00000111) >> 0;
    let register = (addr & 0b00111000) >> 3;
    let mode     = (addr & 0b11000000) >> 6;
    let source = match register {
        0b000 => state.aw,
        0b001 => state.cw,
        0b010 => state.dw,
        0b011 => state.bw,
        0b100 => state.sp,
        0b101 => state.bp,
        0b110 => state.ix,
        0b111 => state.iy,
        _ => unreachable!(),
    };
    match mode {
        0b00 => match memory {
            0b000 => unimplemented!(),
            0b001 => unimplemented!(),
            0b010 => unimplemented!(),
            0b011 => unimplemented!(),
            0b100 => unimplemented!(),
            0b101 => {
                state.memory[state.iy as usize]     += (source & 0xff) as u8;
                state.memory[state.iy as usize + 1] += (source >> 8) as u8;
                match state.iy % 2 {
                    0 => 7,
                    1 => 11,
                    _ => unreachable!()
                }
            },
            0b110 => unimplemented!(),
            0b111 => unimplemented!(),
            _ => unreachable!(),
        },
        0b01 => match memory {
            0b000 => unimplemented!(),
            0b001 => unimplemented!(),
            0b010 => unimplemented!(),
            0b011 => unimplemented!(),
            0b100 => unimplemented!(),
            0b101 => unimplemented!(),
            0b110 => unimplemented!(),
            0b111 => unimplemented!(),
            _ => unreachable!(),
        },
        0b10 => match memory {
            0b000 => unimplemented!(),
            0b001 => unimplemented!(),
            0b010 => unimplemented!(),
            0b011 => unimplemented!(),
            0b100 => unimplemented!(),
            0b101 => unimplemented!(),
            0b110 => unimplemented!(),
            0b111 => unimplemented!(),
            _ => unreachable!(),
        },
        0b11 => panic!("addressing mode can't be 0b11"),
        _ => unreachable!(),
    }
}

#[inline]
fn add_b_t_rm (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn add_w_t_rm (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn add_b_ia (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn add_w_ia (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_al_m (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_aw_m (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_m_al (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_m_aw (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_mb_imm (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_mw_imm (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_al_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_al(byte);
    2
}

#[inline]
fn mov_bl_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_bl(byte);
    2
}

#[inline]
fn mov_cl_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_cl(byte);
    2
}

#[inline]
fn mov_dl_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_dl(byte);
    2
}

#[inline]
fn mov_ah_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_ah(byte);
    2
}

#[inline]
fn mov_bh_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_bh(byte);
    2
}

#[inline]
fn mov_ch_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_ch(byte);
    2
}

#[inline]
fn mov_dh_i (state: &mut State) -> u64 {
    let byte = state.read_u8();
    state.set_dh(byte);
    2
}

#[inline]
fn mov_aw_i (state: &mut State) -> u64 {
    state.aw = state.read_u16();
    2
}

#[inline]
fn mov_bw_i (state: &mut State) -> u64 {
    state.bw = state.read_u16();
    2
}

#[inline]
fn mov_cw_i (state: &mut State) -> u64 {
    state.cw = state.read_u16();
    2
}

#[inline]
fn mov_dw_i (state: &mut State) -> u64 {
    state.dw = state.read_u16();
    2
}

#[inline]
fn mov_sp_i (state: &mut State) -> u64 {
    state.sp = state.read_u16();
    2
}

#[inline]
fn mov_bp_i (state: &mut State) -> u64 {
    state.bp = state.read_u16();
    2
}

#[inline]
fn mov_ix_i (state: &mut State) -> u64 {
    state.ix = state.read_u16();
    2
}

#[inline]
fn mov_iy_i (state: &mut State) -> u64 {
    state.iy = state.read_u16();
    2
}

#[inline]
fn mov_ds1_aw (state: &mut State) -> u64 {
    state.ds1 = state.aw;
    match state.aw % 2 {
        0 => 10,
        1 => 14,
        _ => unreachable!()
    }
}

#[inline]
fn mov_ds0_aw (state: &mut State) -> u64 {
    unimplemented!()
}

#[inline]
fn in_b (state: &mut State) -> u64 {
    let addr = state.read_u16();
    let data = state.input_u8(addr);
    state.set_al(data);
    5
}

#[inline]
fn in_w (state: &mut State) -> u64 {
    let addr = state.read_u16();
    let data = state.input_u16(addr);
    state.aw = data;
    7
}

#[inline]
fn in_b_v (state: &mut State) -> u64 {
    let addr = state.dw;
    let data = state.input_u8(addr);
    state.set_al(data);
    5
}

#[inline]
fn in_w_v (state: &mut State) -> u64 {
    let addr = state.dw;
    let data = state.input_u16(addr);
    state.aw = data;
    7
}

#[inline]
fn out_b (state: &mut State) -> u64 {
    let addr = state.read_u16();
    let data = state.al();
    state.output_u8(addr, data);
    3
}

#[inline]
fn out_w (state: &mut State) -> u64 {
    let addr = state.read_u16();
    let data = state.aw;
    state.output_u16(addr, data);
    5
}

#[inline]
fn out_b_v (state: &mut State) -> u64 {
    let addr = state.dw;
    let data = state.al();
    state.output_u8(addr, data);
    3
}

#[inline]
fn out_w_v (state: &mut State) -> u64 {
    let addr = state.dw;
    let data = state.aw;
    state.output_u16(addr, data);
    5
}

#[inline]
fn clr1_cy (state: &mut State) -> u64 {
    state.psw = state.psw & 0b1111111111111110;
    2
}

#[inline]
fn set1_cy (state: &mut State) -> u64 {
    state.psw = state.psw | 0b0000000000000001;
    2
}

#[inline]
fn clr1_dir (state: &mut State) -> u64 {
    state.psw = state.psw & 0b1111101111111111;
    2
}

#[inline]
fn set1_dir (state: &mut State) -> u64 {
    state.psw = state.psw | 0b0000010000000000;
    2
}

#[inline]
fn call_d (state: &mut State) -> u64 {
    let displacement = state.read_i16();
    state.push_u16(state.pc);
    state.pc = ((state.pc as i16) + displacement) as u16;
    match state.pc % 2 {
        0 => 7,
        1 => 9,
        _ => unreachable!()
    }
}

#[inline]
/// PC ← PC + disp
fn br_near (state: &mut State) -> u64 {
    let displacement = state.read_i16();
    state.pc = ((state.pc as i16) + displacement) as u16;
    7
}

#[inline]
/// PS ← seg
/// PC ← offset
fn br_far (state: &mut State) -> u64 {
    let offset  = state.read_u16();
    let segment = state.read_u16();
    state.pc = offset;
    state.ps = segment;
    7
}

#[inline]
/// IE ← 0
fn di (state: &mut State) -> u64 {
    state.psw = state.psw & 0b1111110111111111;
    2
}

#[inline]
/// CW ← CW – 1
/// Where CW ≠ 0: PC ← PC + ext-disp8
fn dbnz (state: &mut State) -> u64 {
    let displacement = state.read_i8();
    state.cw = state.cw.overflowing_sub(1).0;
    if state.cw > 0 {
        state.pc = ((state.pc as i32) + displacement as i32) as u16;
        6
    } else {
        3
    }
}

#[inline]
fn dec_aw (state: &mut State) -> u64 {
    state.aw = state.aw.overflowing_sub(1).0;
    2
}

#[inline]
fn dec_bw (state: &mut State) -> u64 {
    state.bw = state.aw.overflowing_sub(1).0;
    2
}

#[inline]
fn dec_cw (state: &mut State) -> u64 {
    state.cw = state.aw.overflowing_sub(1).0;
    2
}

#[inline]
fn dec_dw (state: &mut State) -> u64 {
    state.dw = state.aw.overflowing_sub(1).0;
    2
}

#[inline]
fn bne (state: &mut State) -> u64 {
    let displacement = state.read_i8();
    if state.z() {
        state.pc = ((state.pc as i32) + (displacement as i32)) as u16;
        3
    } else {
        6
    }
}
