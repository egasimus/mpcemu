use crate::define_instruction_set;
use super::{
    CPU,
    Segment,
    to_source_register_value,
    to_target_register_reference,
};

define_instruction_set! {
    [0x00, "ADD",    "Add byte to memory from register",      add_b_f_rm],
    [0x01, "ADD",    "Add word to memory from register",      add_w_f_rm],
    [0x02, "ADD",    "Add byte to register from memory",      add_b_t_rm],
    [0x03, "ADD",    "Add word to register from memory",      add_w_t_rm],
    [0x04, "ADD",    "Add byte to accumulator from constant", add_b_ia],
    [0x05, "ADD",    "Add word to accumulator from constant", add_w_ia],
    [0x06, "",       "",                                      unimplemented],
    [0x07, "",       "",                                      unimplemented],
    [0x08, "",       "",                                      unimplemented],
    [0x09, "",       "",                                      unimplemented],
    [0x0A, "",       "",                                      unimplemented],
    [0x0B, "",       "",                                      unimplemented],
    [0x0C, "",       "",                                      unimplemented],
    [0x0D, "",       "",                                      unimplemented],
    [0x0E, "",       "",                                      unimplemented],
    [0x0F, "GROUP3", "See Group 3",                           group3_instruction],

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

    [0x20, "",      "",                                        unimplemented],
    [0x21, "",      "",                                        unimplemented],
    [0x22, "",      "",                                        unimplemented],
    [0x23, "",      "",                                        unimplemented],
    [0x24, "",      "",                                        unimplemented],
    [0x25, "",      "",                                        unimplemented],
    [0x26, "DS1:",  "Set segment override to data segment 1",  ds1],
    [0x27, "ADJ4A", "",                                        unimplemented],
    [0x28, "SUB",   "b f rm",                                  unimplemented],
    [0x29, "SUB",   "w f rm",                                  unimplemented],
    [0x2A, "SUB",   "b t rm",                                  unimplemented],
    [0x2B, "SUB",   "w t rm",                                  sub_w_t_rm],
    [0x2C, "SUB",   "b ia",                                    unimplemented],
    [0x2D, "SUB",   "w ia",                                    unimplemented],
    [0x2E, "PS:",   "Set segment override to program segment", ps],
    [0x2F, "ADJ4S", "",                                        unimplemented],

    [0x30, "",      "",                                       unimplemented],
    [0x31, "",      "",                                       unimplemented],
    [0x32, "",      "",                                       unimplemented],
    [0x33, "",      "",                                       unimplemented],
    [0x34, "",      "",                                       unimplemented],
    [0x35, "",      "",                                       unimplemented],
    [0x36, "SS:",   "Set segment override to stack segment",  ss],
    [0x37, "ADJBA", "",                                       unimplemented],
    [0x38, "CMP",   "b f rm",                                 unimplemented],
    [0x39, "CMP",   "w f rm",                                 unimplemented],
    [0x3A, "CMP",   "b t rm",                                 unimplemented],
    [0x3B, "CMP",   "w t rm",                                 unimplemented],
    [0x3C, "CMP",   "b, ia",                                  unimplemented],
    [0x3D, "CMP",   "w, ia",                                  cmp_aw_imm],
    [0x3E, "DS0:",  "Set segment override to data segment 0", ds0],
    [0x3F, "ADJBS", "",                                       unimplemented],

    [0x40, "INC AW", "Increment AW by 1", inc_aw],
    [0x41, "INC CW", "Increment CW by 1", inc_cw],
    [0x42, "INC DW", "Increment DW by 1", inc_dw],
    [0x43, "INC BW", "Increment BW by 1", inc_bw],
    [0x44, "INC SP", "Increment SP by 1", inc_sp],
    [0x45, "INC BP", "Increment BP by 1", inc_bp],
    [0x46, "INC IX", "Increment IX by 1", inc_ix],
    [0x47, "INC IY", "Increment IY by 1", inc_iy],
    [0x48, "DEC AW", "Decrement AW by 1", dec_aw],
    [0x49, "DEC CW", "Decrement CW by 1", dec_cw],
    [0x4A, "DEC DW", "Decrement DW by 1", dec_dw],
    [0x4B, "DEC BW", "Decrement BW by 1", dec_bw],
    [0x4C, "DEC SP", "Decrement SP by 1", dec_sp],
    [0x4D, "DEC BP", "Decrement BP by 1", dec_bp],
    [0x4E, "DEC IX", "Decrement IX by 1", dec_ix],
    [0x4F, "DEC IY", "Decrement IY by 1", dec_iy],

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
    [0x6E, "OUTM", "Output byte from memory at IX", outm_b],
    [0x6F, "OUTM", "Output word from memory at IX", outm_w],

    [0x70, "", "", unimplemented],
    [0x71, "", "", unimplemented],
    [0x72, "", "", unimplemented],
    [0x73, "", "", unimplemented],
    [0x74, "BE",  "Branch if Z flag is 1", be],
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

    [0x80, "",     "",                                  unimplemented],
    [0x81, "",     "",                                  unimplemented],
    [0x82, "",     "",                                  unimplemented],
    [0x83, "",     "",                                  unimplemented],
    [0x84, "",     "",                                  unimplemented],
    [0x85, "",     "",                                  unimplemented],
    [0x86, "",     "",                                  unimplemented],
    [0x87, "",     "",                                  unimplemented],
    [0x88, "MOV",  "Move byte to memory from register", unimplemented],
    [0x89, "MOV",  "Move word to memory from register", unimplemented],
    [0x8A, "MOV",  "Move byte to register from memory", unimplemented],
    [0x8B, "MOV",  "Move word to register from memory", mov_w_to_reg],
    [0x8C, "MOV",  "from sreg, rm",                     unimplemented],
    [0x8D, "LDEA", "",                                  unimplemented],
    [0x8E, "MOV",  "to sreg, rm",                       unimplemented],
    [0x8F, "POP",  "rm",                                unimplemented],

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

    [0xA0, "MOV AL", "Move byte into AL from memory", mov_al_m],
    [0xA1, "MOV AW", "Move word into AW from memory", mov_aw_m],
    [0xA2, "MOV",    "Move byte into memory from AL", mov_m_al],
    [0xA3, "MOV",    "Move word into memory from AW", mov_m_aw],
    [0xA4, "",       "",                              unimplemented],
    [0xA5, "",       "",                              unimplemented],
    [0xA6, "",       "",                              unimplemented],
    [0xA7, "",       "",                              unimplemented],
    [0xA8, "",       "",                              unimplemented],
    [0xA9, "",       "",                              unimplemented],
    [0xAA, "",       "",                              unimplemented],
    [0xAB, "",       "",                              unimplemented],
    [0xAC, "LDM",    "b",                             ldm_b],
    [0xAD, "LDM",    "w",                             ldm_w],
    [0xAE, "",       "",                              unimplemented],
    [0xAF, "",       "",                              unimplemented],
    
    [0xB0, "MOV AL", "Move byte constant into AL", mov_al_i],
    [0xB1, "MOV CL", "Move byte constant into CL", mov_cl_i],
    [0xB2, "MOV DL", "Move byte constant into DL", mov_dl_i],
    [0xB3, "MOV BL", "Move byte constant into BL", mov_bl_i],
    [0xB4, "MOV AH", "Move byte constant into AH", mov_ah_i],
    [0xB5, "MOV CH", "Move byte constant into CH", mov_ch_i],
    [0xB6, "MOV BH", "Move byte constant into BH", mov_bh_i],
    [0xB7, "MOV DH", "Move byte constant into DH", mov_dh_i],
    [0xB8, "MOV AW", "Move word constant into AW", mov_aw_i],
    [0xB9, "MOV CW", "Move word constant into CW", mov_cw_i],
    [0xBA, "MOV DW", "Move word constant into DW", mov_dw_i],
    [0xBB, "MOV BW", "Move word constant into BW", mov_bw_i],
    [0xBC, "MOV SP", "Move word constant into SP", mov_sp_i],
    [0xBD, "MOV BP", "Move word constant into BP", mov_bp_i],
    [0xBE, "MOV IX", "Move word constant into IX", mov_ix_i],
    [0xBF, "MOV IY", "Move word constant into IY", mov_iy_i],

    [0xC0, "",        "",                             unimplemented],
    [0xC1, "",        "",                             unimplemented],
    [0xC2, "",        "",                             unimplemented],
    [0xC3, "",        "",                             unimplemented],
    [0xC4, "MOV",     "Move word to DS1 from AW",     mov_ds1_aw],
    [0xC5, "MOV",     "Move word to DS0 from AW",     mov_ds0_aw],
    [0xC6, "MOV",     "Move byte constant to memory", mov_mb_imm],
    [0xC7, "MOV",     "Move word constant to memory", mov_mw_imm],
    [0xC8, "",        "",                             unimplemented],
    [0xC9, "DISPOSE", "Delete a stack frame",         unimplemented],
    [0xCA, "",        "",                             unimplemented],
    [0xCB, "",        "",                             unimplemented],
    [0xCC, "",        "",                             unimplemented],
    [0xCD, "",        "",                             unimplemented],
    [0xCE, "",        "",                             unimplemented],
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
fn nop (state: &mut CPU) -> u64 {
    1
}

#[inline]
fn unimplemented (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn add_b_f_rm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn add_w_f_rm (state: &mut CPU) -> u64 {
    let target   = state.next_u8();
    let memory   = (target & 0b00000111) >> 0;
    let register = (target & 0b00111000) >> 3;
    let mode     = (target & 0b11000000) >> 6;
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
fn add_b_t_rm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn add_w_t_rm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn add_b_ia (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn add_w_ia (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_al_m (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_aw_m (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_m_al (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_m_aw (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_mb_imm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_mw_imm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn mov_al_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_al(byte);
    2
}

#[inline]
fn mov_bl_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_bl(byte);
    2
}

#[inline]
fn mov_cl_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_cl(byte);
    2
}

#[inline]
fn mov_dl_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_dl(byte);
    2
}

#[inline]
fn mov_ah_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_ah(byte);
    2
}

#[inline]
fn mov_bh_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_bh(byte);
    2
}

#[inline]
fn mov_ch_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_ch(byte);
    2
}

#[inline]
fn mov_dh_i (state: &mut CPU) -> u64 {
    let byte = state.next_u8();
    state.set_dh(byte);
    2
}

#[inline]
fn mov_aw_i (state: &mut CPU) -> u64 {
    let value = state.next_u16();
    state.set_aw(value);
    2
}

#[inline]
fn mov_bw_i (state: &mut CPU) -> u64 {
    state.bw = state.next_u16();
    2
}

#[inline]
fn mov_cw_i (state: &mut CPU) -> u64 {
    state.cw = state.next_u16();
    2
}

#[inline]
fn mov_dw_i (state: &mut CPU) -> u64 {
    state.dw = state.next_u16();
    2
}

#[inline]
fn mov_sp_i (state: &mut CPU) -> u64 {
    state.sp = state.next_u16();
    2
}

#[inline]
fn mov_bp_i (state: &mut CPU) -> u64 {
    state.bp = state.next_u16();
    2
}

#[inline]
fn mov_ix_i (state: &mut CPU) -> u64 {
    state.ix = state.next_u16();
    2
}

#[inline]
fn mov_iy_i (state: &mut CPU) -> u64 {
    state.iy = state.next_u16();
    2
}

#[inline]
fn mov_w_to_reg (state: &mut CPU) -> u64 {
    let arg   = state.next_u8();
    let mode  = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let value = to_source_register_value(state, arg & 0b00000111);
        let target = to_target_register_reference(state, (arg & 0b00111000) >> 3);
        *target = value;
        2
    } else {
        let value = state.next_u16();
        let memory = arg & 0b00000111;
        if mode == 0b01 {
            match memory {
                0b000 => unimplemented!(),
                0b001 => unimplemented!(),
                0b010 => unimplemented!(),
                0b011 => unimplemented!(),
                0b100 => unimplemented!(),
                0b101 => unimplemented!(),
                0b110 => unimplemented!(),
                0b111 => unimplemented!(),
                _ => unreachable!(),
            }
        } else if mode == 0b10 {
            match memory {
                0b000 => unimplemented!(),
                0b001 => unimplemented!(),
                0b010 => unimplemented!(),
                0b011 => unimplemented!(),
                0b100 => unimplemented!(),
                0b101 => unimplemented!(),
                0b110 => unimplemented!(),
                0b111 => unimplemented!(),
                _ => unreachable!(),
            }
        } else if mode == 0b00 {
            match memory {
                0b000 => unimplemented!(),
                0b001 => unimplemented!(),
                0b010 => unimplemented!(),
                0b011 => unimplemented!(),
                0b100 => unimplemented!(),
                0b101 => unimplemented!(),
                0b110 => unimplemented!(),
                0b111 => unimplemented!(),
                _ => unreachable!(),
            }
        } else {
            unreachable!();
        }
    }
}

#[inline]
fn mov_ds1_aw (state: &mut CPU) -> u64 {
    state.ds1 = state.aw;
    match state.aw % 2 {
        0 => 10,
        1 => 14,
        _ => unreachable!()
    }
}

#[inline]
fn mov_ds0_aw (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
fn in_b (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.input_u8(addr);
    state.set_al(data);
    5
}

#[inline]
fn in_w (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.input_u16(addr);
    state.aw = data;
    7
}

#[inline]
fn in_b_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.input_u8(addr);
    state.set_al(data);
    5
}

#[inline]
fn in_w_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.input_u16(addr);
    state.aw = data;
    7
}

#[inline]
/// (DW) ← (IX)
/// DIR = 0: IX ← IX + 1
/// DIR = 1: IX ← IX – 1
/// TODO: rep
fn outm_b (state: &mut CPU) -> u64 {
    let data = state.read_u8(state.ix);
    state.output_u8(state.dw, data);
    if state.dir() {
        state.ix = state.ix - 1;
    } else {
        state.ix = state.ix + 1;
    }
    let rep = 1; // TODO
    8 * rep - 2
}

#[inline]
/// (DW + 1, DW) ← (IX + 1, IX)
/// DIR = 0: IX ← IX + 2
/// DIR = 1: IX ← IX – 2
/// TODO: rep
fn outm_w (state: &mut CPU) -> u64 {
    let data = state.read_u16(state.ix);
    state.output_u16(state.dw, data);
    if state.dir() {
        state.ix = state.ix - 2;
    } else {
        state.ix = state.ix + 2;
    }
    let rep = 1; // TODO
    if (state.dw % 2 == 1) && (state.ix % 2 == 1) {
        14 * rep - 2
    } else if state.dw % 2 == 1 {
        12 * rep - 2
    } else if state.ix % 2 == 1 {
        10 * rep - 2
    } else {
        8 * rep - 2
    }
}

#[inline]
fn out_b (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.al();
    state.output_u8(addr, data);
    3
}

#[inline]
fn out_w (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.aw;
    state.output_u16(addr, data);
    5
}

#[inline]
fn out_b_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.al();
    state.output_u8(addr, data);
    3
}

#[inline]
fn out_w_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.aw;
    state.output_u16(addr, data);
    5
}

#[inline]
fn clr1_cy (state: &mut CPU) -> u64 {
    state.psw = state.psw & 0b1111111111111110;
    2
}

#[inline]
fn set1_cy (state: &mut CPU) -> u64 {
    state.psw = state.psw | 0b0000000000000001;
    2
}

#[inline]
fn clr1_dir (state: &mut CPU) -> u64 {
    state.psw = state.psw & 0b1111101111111111;
    2
}

#[inline]
fn set1_dir (state: &mut CPU) -> u64 {
    state.psw = state.psw | 0b0000010000000000;
    2
}

#[inline]
fn call_d (state: &mut CPU) -> u64 {
    let displacement = state.next_i16();
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
fn br_near (state: &mut CPU) -> u64 {
    let displacement = state.next_i16();
    state.pc = ((state.pc as i16) + displacement) as u16;
    7
}

#[inline]
/// PS ← seg
/// PC ← offset
fn br_far (state: &mut CPU) -> u64 {
    let offset  = state.next_u16();
    let segment = state.next_u16();
    state.pc = offset;
    state.ps = segment;
    7
}

#[inline]
/// IE ← 0
fn di (state: &mut CPU) -> u64 {
    state.psw = state.psw & 0b1111110111111111;
    2
}

#[inline]
/// CW ← CW – 1
/// Where CW ≠ 0: PC ← PC + ext-disp8
fn dbnz (state: &mut CPU) -> u64 {
    let displacement = state.next_i8();
    state.cw = state.cw.overflowing_sub(1).0;
    if state.cw > 0 {
        state.pc = ((state.pc as i32) + displacement as i32) as u16;
        6
    } else {
        3
    }
}

#[inline]
fn inc_aw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.aw.overflowing_add(1);
    state.set_aw(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_bw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.bw.overflowing_add(1);
    state.set_bw(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_cw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.cw.overflowing_add(1);
    state.set_cw(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_dw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.dw.overflowing_add(1);
    state.set_dw(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_sp (state: &mut CPU) -> u64 {
    let (value, overflow) = state.sp.overflowing_add(1);
    state.set_sp(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_bp (state: &mut CPU) -> u64 {
    let (value, overflow) = state.bp.overflowing_add(1);
    state.set_bp(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_ix (state: &mut CPU) -> u64 {
    let (value, overflow) = state.ix.overflowing_add(1);
    state.set_ix(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn inc_iy (state: &mut CPU) -> u64 {
    let (value, overflow) = state.ix.overflowing_add(1);
    state.set_iy(value);
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_aw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.aw.overflowing_sub(1);
    state.aw = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_bw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.bw.overflowing_sub(1);
    state.bw = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_cw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.cw.overflowing_sub(1);
    state.cw = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_dw (state: &mut CPU) -> u64 {
    let (value, overflow) = state.dw.overflowing_sub(1);
    state.dw = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_sp (state: &mut CPU) -> u64 {
    let (value, overflow) = state.sp.overflowing_sub(1);
    state.sp = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_bp (state: &mut CPU) -> u64 {
    let (value, overflow) = state.bp.overflowing_sub(1);
    state.bp = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_ix (state: &mut CPU) -> u64 {
    let (value, overflow) = state.ix.overflowing_sub(1);
    state.ix = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn dec_iy (state: &mut CPU) -> u64 {
    let (value, overflow) = state.ix.overflowing_sub(1);
    state.ix = value;
    state.set_cy(overflow);
    2
}

#[inline]
fn be (state: &mut CPU) -> u64 {
    let displacement = state.next_i8();
    if state.z() {
        6
    } else {
        state.pc = ((state.pc as i32) + (displacement as i32)) as u16;
        3
    }
}

#[inline]
fn bne (state: &mut CPU) -> u64 {
    let displacement = state.next_i8();
    if state.z() {
        state.pc = ((state.pc as i32) + (displacement as i32)) as u16;
        3
    } else {
        6
    }
}

#[inline]
fn ldm_b (state: &mut CPU) -> u64 {
    let data = state.read_u8(state.ix);
    state.set_al(data);
    if state.dir() {
        state.ix = state.ix - 1;
    } else {
        state.ix = state.ix + 1;
    }
    5
}

#[inline]
fn ldm_w (state: &mut CPU) -> u64 {
    let data = state.read_u16(state.ix);
    state.aw = data;
    if state.dir() {
        state.ix = state.ix - 2;
    } else {
        state.ix = state.ix + 2;
    }
    if state.ix % 2 == 1 {
        7
    } else {
        5
    }
}

#[inline]
fn ds0 (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::DS0);
    2
}

#[inline]
fn ds1 (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::DS1);
    2
}

#[inline]
fn ps (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::PS);
    2
}

#[inline]
fn ss (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::PS);
    2
}

#[inline]
fn cmp_aw_imm (state: &mut CPU) -> u64 {
    let value = state.next_u16();
    let (result, unsigned_overflow) = state.aw.overflowing_sub(value);
    let (_, signed_overflow) = (state.aw as i16).overflowing_sub(value as i16);
    state.set_pzs(result);
    state.set_cy(unsigned_overflow);
    state.set_v(signed_overflow);
    2
}

#[inline]
fn sub_w_t_rm (state: &mut CPU) -> u64 {
    unimplemented!();
}

#[inline]
fn group3_instruction (state: &mut CPU) -> u64 {
    let opcode = state.next_u8();
    group3::execute_instruction(state, opcode)
}

mod group3 {
    use crate::define_instruction_set;
    use super::CPU;

    define_instruction_set! {
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
        [0x22, "", "", unimplemented],
        [0x26, "", "", unimplemented],
        [0x28, "", "", unimplemented],
        [0x2A, "", "", unimplemented],

        [0x31, "", "", unimplemented],
        [0x33, "", "", unimplemented],
        [0x39, "", "", unimplemented],
        [0x3B, "", "", unimplemented],

        [0xE0, "BRKXA", "Start/break extended addressing mode", brkxa],

        [0xF0, "RETXA", "Exit extended addressing mode", retxa],
    }

    #[inline]
    fn unimplemented (state: &mut CPU) -> u64 {
        unimplemented!()
    }

    #[inline]
    fn brkxa (state: &mut CPU) -> u64 {
        let addr = state.next_u8();
        state.pc = state.read_u16(addr as u16 * 4);
        state.ps = state.read_u16(addr as u16 * 4 + 2);
        12
    }

    #[inline]
    fn retxa (state: &mut CPU) -> u64 {
        unimplemented!();
        12
    }
}
