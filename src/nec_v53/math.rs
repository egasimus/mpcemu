use super::*;

#[inline]
pub fn add_b_f_rm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn add_w_f_rm (state: &mut CPU) -> u64 {
    let target   = state.next_u8();
    let mode     = (target & B_MODE) >> 6;
    let register = (target & B_REG)  >> 3;
    let memory   = (target & B_MEM)  >> 0;
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
pub fn sub_w_t_rm (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = word_register_value(state, arg & B_MEM);
        let dst = word_register_reference(state, (arg & B_REG) >> 3);
        let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
        let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
        *dst = result;
        state.set_pzs(result);
        state.set_cy(unsigned_overflow);
        state.set_v(signed_overflow);
        2
    } else {
        let addr = memory_address(state, mode, arg & B_MEM) as usize;
        let src  = u16::from_le_bytes([state.memory[addr], state.memory[addr + 1]]);
        let dst  = word_register_reference(state, (arg & B_REG) >> 3);
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
