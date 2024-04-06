use super::*;

#[inline]
pub fn add_b_f_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    let src  = state.register_value_u8(reg);
    let addr = state.memory_address(mode, mem);
    let dst  = state.read_u8(addr);
    let (result, unsigned_overflow) = dst.overflowing_add(src);
    let (_, signed_overflow) = (dst as i8).overflowing_add(src as i8);
    state.write_u8(addr, result);
    state.set_pzs(result as u16);
    state.set_cy(unsigned_overflow);
    state.set_v(signed_overflow);
    if addr % 2 == 0 {
        7
    } else {
        11
    }
}

#[inline]
pub fn add_w_f_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    let src  = state.register_value_u16(reg);
    match mode {
        0b00 => match mem {
            0b000 => unimplemented!(),
            0b001 => unimplemented!(),
            0b010 => unimplemented!(),
            0b011 => unimplemented!(),
            0b100 => unimplemented!(),
            0b101 => {
                state.write_u16(state.iy(), src);
                if state.iy % 2 == 0 { 7 } else { 11 }
            },
            0b110 => unimplemented!(),
            0b111 => unimplemented!(),
            _ => unreachable!(),
        },
        0b01 => match mem {
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
        0b10 => match mem {
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
pub fn add_b_t_rm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn add_w_t_rm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn add_b_ia (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn add_w_ia (state: &mut CPU) -> u64 {
    let word = state.next_u16();
    let (result, unsigned_overflow) = state.aw().overflowing_add(word);
    let (_, signed_overflow) = (state.aw() as i16).overflowing_add(word as i16);
    state.set_aw(result);
    state.set_pzs(result);
    state.set_cy(unsigned_overflow);
    state.set_v(signed_overflow);
    2
}

#[inline]
pub fn cmp_aw_imm (state: &mut CPU) -> u64 {
    let value = state.next_u16();
    let (result, unsigned_overflow) = state.aw.overflowing_sub(value);
    let (_, signed_overflow) = (state.aw as i16).overflowing_sub(value as i16);
    state.set_pzs(result);
    state.set_cy(unsigned_overflow);
    state.set_v(signed_overflow);
    2
}

#[inline]
pub fn or_w_t_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u16(arg & B_MEM);
        let dst = state.register_reference_u16((arg & B_REG) >> 3);
        let result = *dst | src;
        *dst = result;
        state.set_pzs(result);
        2
    } else {
        let addr = state.memory_address(mode, arg & B_MEM);
        let src  = state.read_u16(addr);
        let dst  = state.register_reference_u16((arg & B_REG) >> 3);
        let result = *dst | src;
        *dst = result;
        state.set_pzs(result);
        if addr % 2 == 0 {
            6
        } else {
            8
        }
    }
}

#[inline]
pub fn sub_w_t_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u16(arg & B_MEM);
        let dst = state.register_reference_u16((arg & B_REG) >> 3);
        let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
        let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
        *dst = result;
        state.set_pzs(result);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        2
    } else {
        let addr = state.memory_address(mode, arg & B_MEM);
        let src  = state.read_u16(addr);
        let dst  = state.register_reference_u16((arg & B_REG) >> 3);
        let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
        let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
        *dst = result;
        state.set_pzs(result);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        if addr % 2 == 0 {
            6
        } else {
            8
        }
    }
}

#[inline]
pub fn cmp_b_f_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u8((arg & B_REG) >> 3);
        let dst = state.register_value_u8(arg & B_MEM);
        let (result, unsigned_overflow) = dst.overflowing_sub(src);
        let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
        state.set_pzs(result as u16);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        2
    } else {
        let src  = state.register_value_u8((arg & B_REG) >> 3);
        let addr = state.memory_address(mode, arg & B_MEM);
        let dst  = state.read_u8(addr);
        let (result, unsigned_overflow) = dst.overflowing_sub(src);
        let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
        state.set_pzs(result as u16);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        if addr % 2 == 0 {
            6
        } else {
            8
        }
    }
}

#[inline]
pub fn cmp_w_t_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u16(arg & B_MEM);
        let dst = state.register_reference_u16((arg & B_REG) >> 3);
        let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
        let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
        state.set_pzs(result);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        2
    } else {
        let addr = state.memory_address(mode, arg & B_MEM);
        let src  = state.read_u16(addr);
        let dst  = state.register_reference_u16((arg & B_REG) >> 3);
        let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
        let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
        state.set_pzs(result);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        if addr % 2 == 0 {
            6
        } else {
            8
        }
    }
}

#[inline]
pub fn sub_b_t_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u8(arg & B_MEM);
        let reg = (arg & B_REG) >> 3;
        let dst = state.register_value_u8(reg);
        let (result, unsigned_overflow) = dst.overflowing_sub(src);
        let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
        state.set_register_u8(reg, result);
        state.set_pzs(result as u16);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        2
    } else {
        let addr = state.memory_address(mode, arg & B_MEM);
        let src  = state.read_u8(addr);
        let reg  = (arg & B_REG) >> 3;
        let dst  = state.register_value_u8(reg);
        let (result, unsigned_overflow) = dst.overflowing_sub(src);
        let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
        state.set_register_u8(reg, result);
        state.set_pzs(result as u16);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        if addr % 2 == 0 {
            6
        } else {
            8
        }
    }
}

#[inline]
pub fn xor_w_to_reg (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u16(arg & B_MEM);
        let dst = state.register_reference_u16((arg & B_REG) >> 3);
        let result = *dst ^ src;
        *dst = result;
        state.set_pzs(result);
        2
    } else {
        let addr = state.memory_address(mode, arg & B_MEM);
        let src  = state.read_u16(addr);
        let dst  = state.register_reference_u16((arg & B_REG) >> 3);
        let result = *dst ^ src;
        *dst = result;
        state.set_pzs(result);
        if addr % 2 == 0 {
            6
        } else {
            8
        }
    }
}

#[inline]
pub fn imm_b (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let code = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    match code {
        0b000 => {
            unimplemented!("add");
        },
        0b001 => {
            unimplemented!("or");
        },
        0b010 => {
            unimplemented!("addc");
        },
        0b011 => {
            unimplemented!("sub");
        },
        0b100 => {
            unimplemented!("and");
        },
        0b101 => {
            unimplemented!("sub");
        },
        0b110 => {
            unimplemented!("xor");
        },
        0b111 => {
            if mode == 0b11 {
                unimplemented!("cmp reg, imm");
                2
            } else {
                let addr = state.memory_address(mode, mem);
                let dst = state.read_u8(addr);
                let src = state.next_u8();
                let (result, unsigned_overflow) = dst.overflowing_sub(src);
                let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
                state.set_pzs(result as u16);
                state.set_cy(unsigned_overflow);
                state.set_v(signed_overflow);
                if addr % 2 == 0 { 6 } else { 8 }
            }
        },
        _ => {
            unreachable!("imm code {code:b}");
        }
    }
}

#[inline]
pub fn imm_w (state: &mut CPU) -> u64 {
    unimplemented!();
}

#[inline]
pub fn imm_b_s (state: &mut CPU) -> u64 {
    unimplemented!();
}

#[inline]
pub fn imm_w_s (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let code = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    match code {
        0b000 => {
            unimplemented!("add");
        },
        0b001 => {
            unimplemented!("or");
        },
        0b010 => {
            unimplemented!("addc");
        },
        0b011 => {
            unimplemented!("sub");
        },
        0b100 => {
            unimplemented!("and");
        },
        0b101 => {
            unimplemented!("sub");
        },
        0b110 => {
            unimplemented!("xor");
        },
        0b111 => {
            // FIXME: signed
            if mode == 0b11 {
                let dst = state.register_value_u16(mem) as i16;
                let src = state.next_u16() as i16;
                let (result, unsigned_overflow) = (dst as u16).overflowing_sub(src as u16);
                let (_, signed_overflow) = dst.overflowing_sub(src);
                state.set_pzs(result);
                state.set_cy(unsigned_overflow);
                state.set_v(signed_overflow);
                2
            } else {
                let addr = state.memory_address(mode, mem);
                let dst = state.read_u8(addr);
                let src = state.next_u8();
                let (result, unsigned_overflow) = (dst as u16).overflowing_sub(src as u16);
                let (_, signed_overflow) = dst.overflowing_sub(src);
                state.set_pzs(result);
                state.set_cy(unsigned_overflow);
                state.set_v(signed_overflow);
                if addr % 2 == 0 { 6 } else { 8 }
            }
        },
        _ => {
            unreachable!("imm code {code:b}");
        }
    }
}
