use super::*;
use mpcemu_core::{instruction, Instruction};

instruction! (add_b_f_rm (cpu) {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    (
        format!("Add byte to memory from register"),
        vec![0x00, arg],
        Box::new(|cpu: &mut CPU| {
            let src  = cpu.register_value_u8(reg);
            let addr = cpu.memory_address(mode, mem);
            let dst  = cpu.read_u8(addr);
            let (result, unsigned_overflow) = dst.overflowing_add(src);
            let (_, signed_overflow) = (dst as i8).overflowing_add(src as i8);
            cpu.write_u8(addr, result);
            cpu.set_pzs(result as u16);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            if addr % 2 == 0 { 7 } else { 11 }
        })
    )
});

instruction! (add_w_f_rm (cpu) {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    (
        format!("Add word to memory from register"),
        vec![0x01, arg],
        Box::new(|cpu: &mut CPU| {
            let src  = cpu.register_value_u16(reg);
            let addr = cpu.memory_address(mode, mem);
            let dst  = cpu.read_u16(addr);
            let (result, unsigned_overflow) = dst.overflowing_add(src);
            let (_, signed_overflow) = (dst as i16).overflowing_add(src as i16);
            cpu.write_u16(addr, result);
            cpu.set_pzs(result as u16);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            if addr % 2 == 0 { 7 } else { 11 }
        })
    )
});

instruction! (add_b_t_rm (cpu) { unimplemented!() });
instruction! (add_w_t_rm (cpu) { unimplemented!() });
instruction! (add_b_ia (cpu) { unimplemented!() });

instruction! (add_w_ia (cpu) {
    let word = cpu.next_u16();
    let [lo, hi] = word.to_le_bytes();
    (
        format!("Add word to accumulator from constant"),
        vec![0x05, lo, hi],
        Box::new(|cpu: &mut CPU|{
            let (result, unsigned_overflow) = cpu.aw().overflowing_add(word);
            let (_, signed_overflow) = (cpu.aw() as i16).overflowing_add(word as i16);
            cpu.set_aw(result);
            cpu.set_pzs(result);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            2
        })
    )
});

instruction! (cmp_aw_imm (cpu) {
    let word = cpu.next_u16();
    let [lo, hi] = word.to_le_bytes();
    (
        format!("CMP w ia"),
        vec![0x3D, lo, hi],
        Box::new(|cpu: &mut CPU|{
            let (result, unsigned_overflow) = cpu.aw.overflowing_sub(word);
            let (_, signed_overflow) = (cpu.aw as i16).overflowing_sub(word as i16);
            cpu.set_pzs(result);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            2
        })
    )
});

instruction! (or_w_t_rm (cpu) {
    let arg  = cpu.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    (
        format!("Word bitwise OR to register from memory"),
        vec![0x0B, arg],
        Box::new(|cpu: &mut CPU|{
            if mode == 0b11 {
                let src = cpu.register_value_u16(arg & B_MEM);
                let dst = cpu.register_reference_u16((arg & B_REG) >> 3);
                let result = *dst | src;
                *dst = result;
                cpu.set_pzs(result);
                2
            } else {
                let addr = cpu.memory_address(mode, arg & B_MEM);
                let src  = cpu.read_u16(addr);
                let dst  = cpu.register_reference_u16((arg & B_REG) >> 3);
                let result = *dst | src;
                *dst = result;
                cpu.set_pzs(result);
                if addr % 2 == 0 {
                    6
                } else {
                    8
                }
            }
        })
    )
});

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
    // FIXME: sign extend https://en.wikipedia.org/wiki/Sign_extension
    match code {
        0b000 => {
            if mode == 0b11 {
                let dst = state.register_value_u16(mem) as i16;
                let src = state.next_u16() as i16;
                let (result, unsigned_overflow) = (dst as u16).overflowing_add(src as u16);
                let (_, signed_overflow) = dst.overflowing_add(src);
                state.set_register_u16(mem, result);
                state.set_pzs(result);
                state.set_cy(unsigned_overflow);
                state.set_v(signed_overflow);
                2
            } else {
                let addr = state.memory_address(mode, mem);
                let dst = state.read_u16(addr);
                let src = state.next_u16();
                let (result, unsigned_overflow) = (dst as u16).overflowing_add(src as u16);
                let (_, signed_overflow) = dst.overflowing_add(src);
                state.set_register_u16(mem, result);
                state.set_pzs(result);
                state.set_cy(unsigned_overflow);
                state.set_v(signed_overflow);
                if addr % 2 == 0 { 6 } else { 8 }
            }
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
                let dst = state.read_u16(addr);
                let src = state.next_u16();
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

pub fn group1_b (state: &mut CPU) -> u64 {
    let arg = state.next_u8();
    let code = (arg & B_REG) >> 3;
    match code {
        0b000 => {
            unimplemented!("test rm");
        },
        0b001 => {
            panic!("undefined group1 instruction");
        },
        0b010 => {
            unimplemented!("not rm");
        },
        0b011 => {
            unimplemented!("neg rm");
        },
        0b100 => {
            unimplemented!("mulu rm");
        },
        0b101 => {
            unimplemented!("mul rm");
        },
        0b110 => {
            unimplemented!("divu rm");
        },
        0b111 => {
            let t = state.aw() as i16;
            let mode = (arg & 0b11000000) >> 6;
            if mode == 0b11 {
                let dst = state.register_value_u8((arg & B_REG) >> 3) as i16;
                if (((t / dst) > 0) && ((t / dst) <= 0x7F)) ||
                   (((t / dst) < 0) && ((t / dst) > (0 - 0x7F - 1)))
                {
                    state.set_ah((t % dst) as u8);
                    state.set_al((t / dst) as u8);
                }
                state.push_u16(state.psw());
                state.set_ie(false);
                state.set_brk(false);
                //state.push_u16(state.ps());
                //state.set_ps(u16::from_le_bytes([0x2, 0x3]));
                //state.push_u16(state.pc());
                //state.set_pc(u16::from_le_bytes([0x0, 0x1]));
                17
            } else {
                let mem  = arg & 0b00000111;
                let addr = state.memory_address(mode, mem);
                let dst  = sign_extend_16(state.read_u8(addr) as u16, 8);
                println!("\n\naddr={addr:x} dst={dst:b} t={t:b}\n");
                state.memory_dump((addr / 0x10 - 0x8) * 0x10, 0x10, 0x10);
                if (((t / dst) > 0) && ((t / dst) <= 0x7F)) ||
                   (((t / dst) < 0) && ((t / dst) > (0 - 0x7F - 1)))
                {
                    state.set_ah((t % dst) as u8);
                    state.set_al((t / dst) as u8);
                }
                state.push_u16(state.psw());
                state.set_ie(false);
                state.set_brk(false);
                20
            }
        },
        _ => {
            unreachable!("group1 code {code:b}");
        }
    }
}

pub fn group1_w (state: &mut CPU) -> u64 {
    let arg = state.next_u8();
    let code = (arg & B_REG) >> 3;
    match code {
        0b000 => {
            unimplemented!("test rm");
        },
        0b001 => {
            panic!("undefined group1 instruction");
        },
        0b010 => {
            unimplemented!("not rm");
        },
        0b011 => {
            unimplemented!("neg rm");
        },
        0b100 => {
            unimplemented!("mulu rm");
        },
        0b101 => {
            unimplemented!("mul rm");
        },
        0b110 => {
            unimplemented!("divu rm");
        },
        0b111 => {
            let [b0, b1] = state.dw().to_le_bytes();
            let [b2, b3] = state.aw().to_le_bytes();
            let t = i32::from_le_bytes([b0, b1, b2, b3]);
            let mode = (arg & 0b11000000) >> 6;
            if mode == 0b11 {
                let dst = state.register_value_u16((arg & B_REG) >> 3) as i32;
                if (((t / dst) > 0) && ((t / dst) <= 0x7FFF)) ||
                   (((t / dst) < 0) && ((t / dst) > (0 - 0x7FFFF - 1)))
                {
                    state.set_dw((t % dst) as u16);
                    state.set_aw((t / dst) as u16);
                }
                state.push_u16(state.psw());
                state.set_ie(false);
                state.set_brk(false);
                //state.push_u16(state.ps());
                //state.set_ps(u16::from_le_bytes([0x2, 0x3]));
                //state.push_u16(state.pc());
                //state.set_pc(u16::from_le_bytes([0x0, 0x1]));
                24
            } else {
                unimplemented!();
            }
        },
        _ => {
            unreachable!("group1 code {code:b}");
        }
    }
}

#[inline]
pub fn sign_extend_16 (data: u16, size: u16) -> i16 {
    assert!(size > 0 && size <= 16);
    ((data << (16 - size)) as i16) >> (16 - size)
}

#[inline]
pub fn sign_extend_32 (data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}
