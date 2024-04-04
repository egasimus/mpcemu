use super::*;

/// Segment override
pub enum Segment {
    /// Use data segment 0
    DS0,
    /// Use data segment 1
    DS1,
    /// Use program segment
    PS,
    /// Use stack segment
    SS,
}

impl CPU {
    /// Effective address
    pub fn ea (&self, addr: u16) -> usize {
        ((match self.segment {
            None               => self.ds0,
            Some(Segment::DS0) => self.ds0,
            Some(Segment::DS1) => self.ds1,
            Some(Segment::PS)  => self.ps,
            Some(Segment::SS)  => self.ss
        } as u32 * 0x10) + addr as u32) as usize
    }
    /// Program address
    pub fn address (&self) -> usize {
        (((self.ps as u32) * 0x10) + self.pc as u32) as usize
    }
    pub fn peek_u8 (&mut self) -> u8 {
        self.memory[self.address()]
    }
    pub fn peek_i8 (&mut self) -> i8 {
        self.memory[self.address()] as i8
    }
    pub fn next_u8 (&mut self) -> u8 {
        let byte = self.peek_u8();
        self.pc += 1;
        byte
    }
    pub fn next_i8 (&mut self) -> i8 {
        let byte = self.peek_i8();
        self.pc += 1;
        byte
    }
    pub fn next_u16 (&mut self) -> u16 {
        let lo = self.next_u8() as u16;
        let hi = self.next_u8() as u16;
        hi << 8 | lo
    }
    pub fn next_i16 (&mut self) -> i16 {
        let lo = self.next_u8();
        let hi = self.next_u8();
        i16::from_le_bytes([lo, hi])
    }
    /// Read byte from effective address
    pub fn read_u8 (&mut self, addr: u16) -> u8 {
        self.memory[self.ea(addr)]
    }
    /// Read word from effective address
    pub fn read_u16 (&mut self, addr: u16) -> u16 {
        let lo = self.read_u8(addr);
        let hi = self.read_u8(addr + 1);
        u16::from_le_bytes([lo, hi])
    }
    /// Read byte from input port
    pub fn input_u8 (&self, addr: u16) -> u8 {
        self.ports[addr as usize]
    }
    /// Read word from input port
    pub fn input_u16 (&self, addr: u16) -> u16 {
        let lo = self.input_u8(addr) as u16;
        let hi = self.input_u8(addr + 1) as u16;
        hi << 8 | lo
    }
    /// Write byte to input port
    pub fn output_u8 (&mut self, addr: u16, data: u8) {
        self.ports[addr as usize] = data;
    }
    /// Write byte to output port
    pub fn output_u16 (&mut self, addr: u16, data: u16) {
        self.output_u8(addr, (data & 0b0000000011111111) as u8);
        self.output_u8(addr + 1, (data >> 8) as u8);
    }
    /// Push a byte to the stack
    pub fn push_u8 (&mut self, data: u8) {
        if self.sp < 1 {
            panic!("stack overflow")
        }
        self.sp = self.sp - 1;
        self.memory[self.sp as usize] = data;
    }
    /// Push a word to the stack
    pub fn push_u16 (&mut self, data: u16) {
        self.push_u8((data & 0b0000000011111111) as u8);
        self.push_u8((data >> 8) as u8);
    }
    pub fn pop_u8 (&mut self) -> u8 {
        let data = self.memory[self.sp as usize];
        self.sp = self.sp + 1;
        data
    }
    pub fn pop_u16 (&mut self) -> u16 {
        let lo = self.pop_u8() as u16;
        let hi = self.pop_u8() as u16;
        hi << 8 | lo
    }
}

#[inline]
pub fn ds0 (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::DS0);
    2
}

#[inline]
pub fn ds1 (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::DS1);
    2
}

#[inline]
pub fn ps (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::PS);
    2
}

#[inline]
pub fn ss (state: &mut CPU) -> u64 {
    state.segment = Some(Segment::SS);
    2
}

#[inline]
pub fn in_b (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.input_u8(addr);
    state.set_al(data);
    5
}

#[inline]
pub fn in_w (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.input_u16(addr);
    state.aw = data;
    7
}

#[inline]
pub fn in_b_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.input_u8(addr);
    state.set_al(data);
    5
}

#[inline]
pub fn in_w_v (state: &mut CPU) -> u64 {
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
pub fn outm_b (state: &mut CPU) -> u64 {
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
pub fn outm_w (state: &mut CPU) -> u64 {
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
pub fn out_b (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.al();
    state.output_u8(addr, data);
    3
}

#[inline]
pub fn out_w (state: &mut CPU) -> u64 {
    let addr = state.next_u16();
    let data = state.aw;
    state.output_u16(addr, data);
    5
}

#[inline]
pub fn out_b_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.al();
    state.output_u8(addr, data);
    3
}

#[inline]
pub fn out_w_v (state: &mut CPU) -> u64 {
    let addr = state.dw;
    let data = state.aw;
    state.output_u16(addr, data);
    5
}

#[inline]
pub fn mov_al_m (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn mov_aw_m (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn mov_m_al (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn mov_m_aw (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn mov_mb_imm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn mov_mw_imm (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
/// Move word from register
pub fn mov_w_from_reg_to_mem (state: &mut CPU) -> u64 {
    let target   = state.next_u8();
    let address  = memory_address(state, (target & 0b11000000) >> 6, target & 0b00000111);
    let value    = word_register_value(state, (target & 0b00111000) >> 3);
    state.memory[address as usize + 0] = value as u8;
    state.memory[address as usize + 1] = (value >> 8) as u8;
    if address % 2 == 0 {
        3
    } else {
        5
    }
}

#[inline]
/// Move word to register
pub fn mov_w_to_reg (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = word_register_value(state, arg & 0b00000111);
        let dst = word_register_reference(state, (arg >> 3) & 0b0000111);
        *dst = src;
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
pub fn mov_w_from_sreg (state: &mut CPU) -> u64 {
    let arg   = state.next_u8();
    let mode  = (arg & 0b11000000) >> 6;
    let value = segment_register_value(state, (arg >> 3) & 0b00000011);
    if mode == 0b11 {
        let dst = word_register_reference(state, arg & 0b00000111);
        *dst = value;
        2
    } else {
        let address = memory_address(state, mode, arg & 0b00000111);
        state.memory[address as usize + 0] = value as u8;
        state.memory[address as usize + 1] = (value >> 8) as u8;
        if address % 2 == 0 {
            3
        } else {
            5
        }
    }
}

#[inline]
/// Move word to segment register
pub fn mov_w_to_sreg (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & 0b11000000) >> 6;
    if mode == 0b11 {
        let src = word_register_value(state, arg & 0b00000111);
        let dst = segment_register_reference(state, (arg >> 3) & 0b00000011);
        *dst = src;
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
pub fn mov_ds1_aw (state: &mut CPU) -> u64 {
    state.ds1 = state.aw;
    match state.aw % 2 {
        0 => 10,
        1 => 14,
        _ => unreachable!()
    }
}

#[inline]
pub fn mov_ds0_aw (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn stm_w (state: &mut CPU) -> u64 {
    let iy = state.iy();
    state.memory[iy as usize] = state.al();
    state.memory[iy as usize + 1] = state.ah();
    state.set_iy(if state.dir() {
        iy.overflowing_sub(2).0
    } else {
        iy.overflowing_add(2).0
    });
    if iy % 2 == 0 {
        5
    } else {
        3
    }
}

#[inline]
pub fn ldm_b (state: &mut CPU) -> u64 {
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
pub fn ldm_w (state: &mut CPU) -> u64 {
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
pub fn rep (state: &mut CPU) -> u64 {
    if state.cw() == 0 {
        state.set_pc(state.pc() + 1);
    } else {
        let op = state.peek_u8();
        if (op == 0xA4) || (op == 0xA5) ||        // MOVBK
           (op == 0xAC) || (op == 0xAD) ||        // LDM
           (op == 0xAA) || (op == 0xAB) ||        // STM
           (op == 0x6E) || (op == 0x6F) ||        // OUTM
           (op == 0x6C) || (op == 0x6D)           // INM
        {
            // repeat while cw != 0
            while state.cw() != 0 {
                state.clock += execute_instruction(state, op);
                state.set_cw(state.cw() - 1);
            }
        } else if (op == 0xA6) || (op == 0xA7) || // CMPBK
            (op == 0xAE) || (op == 0xAF)          // CMPM
        {
            unimplemented!("REPZ/REPE {:x}", op);
            // repeat while cw != 0 && z == 0
        } else {
            panic!("invalid instruction after REP")
        }
    }
    2
}

#[inline]
pub fn memory_address (state: &mut CPU, mode: u8, mem: u8) -> u16 {
    match mode {
        0b00 => match mem {
            0b000 => state.bw() + state.ix(),
            0b001 => state.bw() + state.iy(),
            0b010 => state.bp() + state.ix(),
            0b011 => state.bp() + state.iy(),
            0b100 => state.ix(),
            0b101 => state.iy(),
            0b110 => unimplemented!("direct address"),
            0b111 => state.bw(),
            _ => panic!("invalid memory inner mode {:b}", mem)
        },
        0b01 => {
            let displace = state.next_u8() as u16;
            match mem {
                0b000 => state.bw() + state.ix() + displace,
                0b001 => state.bw() + state.iy() + displace,
                0b010 => state.bp() + state.ix() + displace,
                0b011 => state.bp() + state.iy() + displace,
                0b100 => state.ix() + displace,
                0b101 => state.iy() + displace,
                0b110 => state.bp() + displace,
                0b111 => state.bw() + displace,
                _ => panic!("invalid memory inner mode {:b}", mem)
            }
        },
        0b10 => {
            let displace = state.next_u16();
            match mem {
                0b000 => state.bw() + state.ix() + displace,
                0b001 => state.bw() + state.iy() + displace,
                0b010 => state.bp() + state.ix() + displace,
                0b011 => state.bp() + state.iy() + displace,
                0b100 => state.ix() + displace,
                0b101 => state.iy() + displace,
                0b110 => state.bp() + displace,
                0b111 => state.bw() + displace,
                _ => panic!("invalid memory inner mode {:b}", mem)
            }
        },
        _ => panic!("invalid memory outer mode {:b}", mode)
    }
}
