use super::*;

#[inline]
pub fn shift_w (state: &mut CPU) -> u64 {
    let arg = state.next_u8();
    let source = get_source_word(state, arg);
    let code = (arg & B_REG) >> 3;
    match code {
        0b000 => {
            unimplemented!("rol");
        },
        0b001 => {
            unimplemented!("ror");
        },
        0b010 => {
            let cy  = state.cy() as u16;
            let msb = (source & W15) >> 15;
            let nsb = (source & W14) >> 14;
            let rotated = source << 1 | cy;
            set_source_word(state, arg, rotated);
            state.set_cy(msb > 0);
            state.set_v(msb != nsb);
            2
        },
        0b011 => {
            unimplemented!("rorc");
        },
        0b100 => {
            unimplemented!("shl");
        },
        0b101 => {
            let lsb        = source & W0;
            let msb_before = (source & W15) >> 15;
            let shifted    = source >> 1;
            let msb_after  = (source & W15) >> 15;
            set_source_word(state, arg, shifted);
            state.set_cy(lsb > 0);
            state.set_v(msb_before != msb_after);
            2
        },
        0b110 => {
            panic!("invalid shift code 0b110");
        },
        0b111 => {
            unimplemented!("shra");
        },
        _ => {
            unreachable!("shift code {code:b}");
        }
    }
}

#[inline]
pub fn get_source_word (state: &mut CPU, arg: u8) -> u16 {
    let mode = (arg & B_MODE) >> 6;
    let mem  = arg & B_MEM;
    match mode {
        0b11 => state.register_value_u16(mem),
        _ => {
            let addr = state.memory_address(mode, mem);
            state.read_u16(addr)
        }
    }
}

#[inline]
pub fn set_source_word (state: &mut CPU, arg: u8, val: u16){
    let mode = (arg & B_MODE) >> 6;
    let mem  = arg & B_MEM;
    match mode {
        0b11 => {
            *state.register_reference_u16(mem) = val;
        },
        _ => {
            let addr = state.memory_address(mode, mem);
            state.write_u16(addr, val);
        }
    }
}
