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

    /// Read-only handle to memory
    pub fn memory (&self) -> &[u8] {
        &self.memory
    }

    /// Read-only handle to extended memory
    pub fn extended (&self) -> &[u8] {
        &self.extended
    }

    /// Read-only handle to IO ports memory
    pub fn ports (&self) -> &[u8] {
        &self.ports
    }

    /// Read-only handle to internal IO memory
    pub fn internal (&self) -> &[u8] {
        &self.internal
    }

    pub fn xa (&self) -> bool {
        self.ports[0xff80] > 0
    }

    pub fn set_xa (&mut self, value: bool) {
        self.ports[0xff80] = if value { 1 } else { 0 };
    }

    pub fn get_byte (&self, addr: usize) -> u8 {
        if addr < 0xA0000 {
            if self.xa() {
                self.extended[addr]
            } else {
                self.memory[addr]
            }
        } else {
            self.memory[addr]
        }
    }

    pub fn set_byte (&mut self, addr: usize, value: u8) {
        if addr < 0xA0000 {
            if self.xa() {
                self.extended[addr] = value
            } else {
                self.memory[addr] = value
            }
        } else {
            self.memory[addr] = value
        }
    }

    /// Program address
    pub fn program_address (&self) -> usize {
        (((self.ps as u32) * 0x10) + self.pc as u32) as usize
    }

    /// Effective address
    pub fn effective_address (&self, addr: u16) -> usize {
        let segment = match self.segment {
            None               => self.ds0,
            Some(Segment::DS0) => self.ds0,
            Some(Segment::DS1) => self.ds1,
            Some(Segment::PS)  => self.ps,
            Some(Segment::SS)  => self.ss
        } as u32 * 0x10;

        (segment + addr as u32) as usize
    }

    /// Target address (always offset from DS1)
    pub fn ds1_address (&self, addr: u16) -> usize {
        ((self.ds1 as u32 * 0x10) + addr as u32) as usize
    }

    pub fn peek_u8 (&mut self) -> u8 {
        self.get_byte(self.program_address())
    }

    pub fn peek_i8 (&mut self) -> i8 {
        self.get_byte(self.program_address()) as i8
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
        self.get_byte(self.effective_address(addr))
    }

    /// Read word from effective address
    pub fn read_u16 (&mut self, addr: u16) -> u16 {
        let lo = self.read_u8(addr);
        let hi = self.read_u8(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    /// Write byte to effective address
    pub fn write_u8 (&mut self, addr: u16, value: u8) {
        let ea = self.effective_address(addr);
        self.set_byte(ea, value);
    }

    /// Write word to effective address
    pub fn write_u16 (&mut self, addr: u16, value: u16) {
        let ea = self.effective_address(addr);
        let [lo, hi] = value.to_le_bytes();
        self.set_byte(ea + 0, lo);
        self.set_byte(ea + 1, hi);
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
        let [lo, hi] = data.to_le_bytes();
        self.output_u8(addr + 0, lo);
        self.output_u8(addr + 1, hi);
    }

    /// Push a byte to the stack
    pub fn push_u8 (&mut self, data: u8) {
        if self.sp < 1 {
            panic!("stack overflow")
        }
        self.sp = self.sp - 1;
        self.set_byte(self.sp as usize, data);
    }

    /// Push a word to the stack
    pub fn push_u16 (&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.push_u8(lo);
        self.push_u8(hi);
    }

    pub fn pop_u8 (&mut self) -> u8 {
        let data = self.get_byte(self.sp as usize);
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
pub fn movbk_w (state: &mut CPU) -> u64 {
    let dst = state.ds1() as u32 * 0x10 + state.iy() as u32;
    let src = state.effective_address(state.ix());
    state.set_byte(dst as usize + 0, state.get_byte(src as usize + 0));
    state.set_byte(dst as usize + 1, state.get_byte(src as usize + 1));
    if state.dir() {
        state.set_ix(state.ix() - 2);
        state.set_iy(state.iy() - 2);
    } else {
        state.set_ix(state.ix() + 2);
        state.set_iy(state.iy() + 2);
    }
    if (dst % 2 == 0) && (src % 2 == 0) {
        6
    } else if (dst % 2 == 1) && (src % 2 == 1) {
        10
    } else {
        8
    }
}

#[inline]
/// Move word from register
pub fn mov_w_from_reg_to_mem (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let addr = state.memory_address((arg & B_MODE) >> 6, arg & B_MEM);
    let val  = state.register_value_u16((arg & B_REG) >> 3);
    state.write_u16(addr, val);
    if addr % 2 == 0 {
        3
    } else {
        5
    }
}

#[inline]
/// Move word to register
pub fn mov_w_to_reg (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & B_MODE) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u16(arg & B_MEM);
        let dst = state.register_reference_u16((arg & B_REG) >> 3);
        *dst = src;
        2
    } else {
        let value = state.next_u16();
        let memory = arg & B_MEM;
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
    let mode  = (arg & B_MODE) >> 6;
    let value = state.segment_register_value((arg & B_SREG) >> 3);
    if mode == 0b11 {
        let dst = state.register_reference_u16(arg & B_MEM);
        *dst = value;
        2
    } else {
        let addr = state.memory_address(mode, arg & B_MEM);
        state.write_u16(addr, value);
        if addr % 2 == 0 {
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
    let mode = (arg & B_MODE) >> 6;
    if mode == 0b11 {
        let src = state.register_value_u16(arg & B_MEM);
        let dst = state.segment_register_reference((arg & B_SREG) >> 3);
        *dst = src;
        2
    } else {
        let value = state.next_u16();
        let memory = arg & B_MEM;
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
    if state.aw % 2 == 0 { 10 } else { 14 }
}

#[inline]
pub fn mov_ds0_aw (state: &mut CPU) -> u64 {
    unimplemented!()
}

#[inline]
pub fn stm_w (state: &mut CPU) -> u64 {
    let iy   = state.iy();
    let addr = state.ds1_address(iy);
    let aw   = state.aw();
    state.write_u16(addr as u16, aw);
    state.set_iy(if state.dir() {
        iy.overflowing_sub(2).0
    } else {
        iy.overflowing_add(2).0
    });
    if iy % 2 == 0 { 3 } else { 5 }
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
            state.opcode = op;
            while state.cw() != 0 {
                state.clock += execute_instruction(state, op);
                state.set_cw(state.cw() - 1);
            }
        } else if (op == 0xA6) || (op == 0xA7) || // CMPBK
            (op == 0xAE) || (op == 0xAF)          // CMPM
        {
            state.opcode = op;
            unimplemented!("REPZ/REPE {:x}", op);
            // repeat while cw != 0 && z == 0
        } else {
            panic!("invalid instruction after REP")
        }
    }
    2
}
