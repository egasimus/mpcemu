use super::*;

#[inline]
pub fn shift_w (state: &mut CPU) -> u64 {
    let arg = state.next_u8();
    let source = get_source_word(state, arg);
    let code = (arg & 0b00111000) >> 3;
    match code {
        0b000 => {
            unimplemented!("rol");
        },
        0b001 => {
            unimplemented!("ror");
        },
        0b010 => {
            let cy  = state.cy() as u16;
            let msb = (source & 0b1000000000000000) >> 15;
            let nsb = (source & 0b0100000000000000) >> 14;
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
            let lsb        = source & 0b0000000000000001;
            let msb_before = (source & 0b1000000000000000) >> 15;
            let shifted    = source >> 1;
            let msb_after  = (source & 0b1000000000000000) >> 15;
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
    let mode = (arg & 0b11000000) >> 6;
    let mem  = arg & 0b00000111;
    match mode {
        0b11 => word_register_value(state, mem),
        _ => {
            let addr = memory_address(state, mode, mem) as usize;
            u16::from_le_bytes([state.memory[addr], state.memory[addr + 1]])
        }
    }
}

#[inline]
pub fn set_source_word (state: &mut CPU, arg: u8, val: u16){
    let mode = (arg & 0b11000000) >> 6;
    let mem  = arg & 0b00000111;
    match mode {
        0b11 => {
            *word_register_reference(state, mem) = val;
        },
        _ => {
            let addr = memory_address(state, mode, mem) as usize;
            let [lo, hi] = val.to_le_bytes();
            state.memory[addr] = lo;
            state.memory[addr + 1] = hi;
        }
    }
}
