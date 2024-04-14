/// <https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF>

mod bit;
mod reg;
mod flag;
mod mem;
mod math;
mod shift;
#[cfg(test)] mod test;

use self::{
    bit::*,
    reg::*,
    flag::*,
    mem::*,
    math::*,
    shift::*,
};

pub struct CPU {
    memory:   [u8;0x100000],
    extended: [u8;0xA0000],
    ports:    [u8;0x10000],
    internal: [u8;0x100],

    aw:  u16,
    bw:  u16,
    cw:  u16,
    dw:  u16,

    ps:  u16,
    ss:  u16,
    ds0: u16,
    ds1: u16,

    sp:  u16,
    bp:  u16,
    pc:  u16,
    psw: u16,

    ix:  u16,
    iy:  u16,

    pub segment: Option<Segment>,
    opcode: u8,
    pub clock: u64,
}

impl CPU {

    pub fn new (image: Vec<u8>) -> Self {
        let mut memory = [0x00;0x100000];
        if image.len() > memory.len() {
            panic!("Memory image too big (0x{:X}/0x{:X} bytes)", image.len(), memory.len());
        }
        for i in 0..image.len() {
            memory[i] = image[i];
        }
        Self {
            memory,
            extended: [0x00;0xA0000],
            ports:    [0x00;0x10000],
            internal: [0x00;0x100],
            aw:       0x0000,
            bw:       0x0000,
            cw:       0x0000,
            dw:       0x0000,
            ps:       0xffff,
            ss:       0x0000,
            ds0:      0x0000,
            ds1:      0x0000,
            sp:       0x0000,
            bp:       0x0000,
            pc:       0x0000,
            psw:      W15 | W14 | W13 | W12 | W2,
            ix:       0x0000,
            iy:       0x0000,
            segment:  None,
            opcode:   0xF1,
            clock:    0x0000,
        }
    }

    /// Read and execute the next instruction in the program
    pub fn step (&mut self) {
        let opcode = self.next_u8();
        self.opcode = opcode;
        self.clock += execute_instruction(self, opcode);
        // Reset segment override, except if it was just set:
        if !((opcode == 0x26) || (opcode == 0x2E) || (opcode == 0x36) || (opcode == 0x3E)) {
            self.segment = None
        }
    }

    /// Get the opcode that is currently being executed
    pub fn opcode (&self) -> u8 {
        self.opcode
    }

    pub fn jump_i8 (&mut self, displace: i8) {
        self.pc = ((self.pc as i16) + (displace as i16)) as u16;
    }

    pub fn jump_i16 (&mut self, displace: i16) {
        self.pc = ((self.pc as i16) + displace) as u16;
    }

    pub fn register_value_u8 (&self, reg: u8) -> u8 {
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

    pub fn register_value_u16 (&self, reg: u8) -> u16 {
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

    pub fn register_reference_u16 (&mut self, reg: u8) -> &mut u16 {
        match reg {
            0b000 => &mut self.aw,
            0b001 => &mut self.cw,
            0b010 => &mut self.dw,
            0b011 => &mut self.bw,
            0b100 => &mut self.sp,
            0b101 => &mut self.bp,
            0b110 => &mut self.ix,
            0b111 => &mut self.iy,
            _ => unreachable!(),
        }
    }

    pub fn segment_register_value (&self, sreg: u8) -> u16 {
        match sreg {
            0b00 => self.ds1,
            0b01 => self.ps,
            0b10 => self.ss,
            0b11 => self.ds0,
            _ => unreachable!(),
        }
    }

    pub fn segment_register_reference (&mut self, sreg: u8) -> &mut u16 {
        match sreg {
            0b00 => &mut self.ds1,
            0b01 => &mut self.ps,
            0b10 => &mut self.ss,
            0b11 => &mut self.ds0,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn memory_address (&mut self, mode: u8, mem: u8) -> u16 {
        match mode {
            0b00 => match mem {
                0b000 => self.bw() + self.ix(),
                0b001 => self.bw() + self.iy(),
                0b010 => self.bp() + self.ix(),
                0b011 => self.bp() + self.iy(),
                0b100 => self.ix(),
                0b101 => self.iy(),
                0b110 => self.next_u16(),
                0b111 => self.bw(),
                _ => panic!("invalid memory inner mode {:b}", mem)
            },
            0b01 => {
                let displace = self.next_u8() as u16;
                match mem {
                    0b000 => self.bw() + self.ix() + displace,
                    0b001 => self.bw() + self.iy() + displace,
                    0b010 => self.bp() + self.ix() + displace,
                    0b011 => self.bp() + self.iy() + displace,
                    0b100 => self.ix() + displace,
                    0b101 => self.iy() + displace,
                    0b110 => self.bp() + displace,
                    0b111 => self.bw() + displace,
                    _ => panic!("invalid memory inner mode {:b}", mem)
                }
            },
            0b10 => {
                let displace = self.next_u16();
                match mem {
                    0b000 => self.bw() + self.ix() + displace,
                    0b001 => self.bw() + self.iy() + displace,
                    0b010 => self.bp() + self.ix() + displace,
                    0b011 => self.bp() + self.iy() + displace,
                    0b100 => self.ix() + displace,
                    0b101 => self.iy() + displace,
                    0b110 => self.bp() + displace,
                    0b111 => self.bw() + displace,
                    _ => panic!("invalid memory inner mode {:b}", mem)
                }
            },
            _ => panic!("invalid memory outer mode {:b}", mode)
        }
    }

    pub fn memory_dump (&self, start: u16, per_row: u8, rows: u8) {
        for i in 0..rows {
            let offset = start + i as u16 * per_row as u16;
            print!("\n{:6X}|", offset);
            for j in 0..per_row {
                print!(" {:02x}", self.memory()[offset as usize + j as usize]);
            }
        }
    }

}

fn get_mode_reg_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, reg, mem]
}

mpcemu_core::impl_instruction_set! {
    CPU,

    0x00 => add_b_f_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
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
    },

    0x01 => add_w_f_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
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
    },

    0x02 => add_b_t_rm (cpu) {
        unimplemented!()
    },

    0x03 => add_w_t_rm (cpu) {
        unimplemented!()
    },

    0x04 => add_b_ia (cpu) {
        unimplemented!()
    },

    0x05 => add_w_ia (cpu) {
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
    },

    0x06 => push_ds1 (cpu),

    0x07 => pop_ds1 (cpu),

    0x08 => or (cpu) {
        unimplemented!("Byte bitwise OR to memory from register")
    },

    0x09 => or (cpu) {
        unimplemented!("Word bitwise OR to memory from register")
    },

    0x0A => or (cpu) {
        unimplemented!("Byte bitwise OR to register from memory")
    },

    0x0B => or_w_t_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        (
            format!("Word bitwise OR to register from memory"),
            vec![0x0B, arg],
            Box::new(|cpu: &mut CPU|{
                if mode == 0b11 {
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.register_reference_u16(reg);
                    let result = *dst | src;
                    *dst = result;
                    cpu.set_pzs(result);
                    2
                } else {
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u16(addr);
                    let dst  = cpu.register_reference_u16(reg);
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
    },

    0x0C => or (cpu) {
        unimplemented!("Bitwise OR b ia")
    },

    0x0D => or (cpu) {
        unimplemented!("Bitwise OR w ia")
    },

    0x0E => push_ps,

    0x0F => group3_instruction,

    0x10 => unimplemented("ADDC"),

    0x11 => unimplemented("ADDC"),

    0x12 => unimplemented("ADDC"),

    0x13 => unimplemented("ADDC"),

    0x14 => unimplemented("ADDC"),

    0x15 => unimplemented("ADDC"),

    0x16 => push_ss,

    0x17 => pop_ss,

    0x18 => unimplemented("SUBC"),

    0x19 => unimplemented("SUBC"),

    0x1A => unimplemented("SUBC"),

    0x1B => unimplemented("SUBC"),

    0x1C => unimplemented("SUBC"),

    0x1D => unimplemented("SUBC"),

    0x1E => push_ds0,

    0x1F => pop_ds0,

    0x20 => unimplemented("AND"),

    0x21 => unimplemented("AND"),

    0x22 => unimplemented("AND"),

    0x23 => unimplemented("AND"),

    0x24 => unimplemented("AND"),

    0x25 => unimplemented("AND"),

    0x26 => ds1,

    0x27 => unimplemented("ADJ4A"),

    0x28 => unimplemented("SUB b f rm"),

    0x29 => unimplemented("SUB w f rm"),

    0x2A => sub_b_t_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        if mode == 0b11 {
            let src = cpu.register_value_u8(mem);
            let dst = cpu.register_value_u8(reg);
            let (result, unsigned_overflow) = dst.overflowing_sub(src);
            let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
            cpu.set_register_u8(reg, result);
            cpu.set_pzs(result as u16);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            2
        } else {
            let addr = cpu.memory_address(mode, mem);
            let src  = cpu.read_u8(addr);
            let dst  = cpu.register_value_u8(reg);
            let (result, unsigned_overflow) = dst.overflowing_sub(src);
            let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
            cpu.set_register_u8(reg, result);
            cpu.set_pzs(result as u16);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            if addr % 2 == 0 { 6 } else { 8 }
        }
    },

    0x2B => sub_w_t_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        if mode == 0b11 {
            let src = cpu.register_value_u8(mem);
            let dst = cpu.register_value_u8(reg);
            let (result, unsigned_overflow) = dst.overflowing_sub(src);
            let (_, signed_overflow) = (dst as i16).overflowing_sub(src as i8);
            cpu.set_register_u8(reg, result);
            cpu.set_pzs(result as u16);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            2
        } else {
            let addr = cpu.memory_address(mode, mem);
            let src  = cpu.read_u8(addr);
            let dst  = cpu.register_value_u8(reg);
            let (result, unsigned_overflow) = dst.overflowing_sub(src);
            let (_, signed_overflow) = (dst as i16).overflowing_sub(src as i8);
            cpu.set_register_u8(reg, result);
            cpu.set_pzs(result as u16);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            if addr % 2 == 0 { 6 } else { 8 }
        }
    },

    0x2C => unimplemented("SUB b, ia"),

    0x2D => unimplemented("SUB w, ia"),

    0x2E => ps,

    0x2F => unimplemented("ADJ4S"),

    0x30 => unimplemented("XOR"),

    0x31 => unimplemented("XOR"),

    0x32 => unimplemented("XOR"),

    0x33 => xor_w_to_reg (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        if mode == 0b11 {
            let src = cpu.register_value_u16(mem);
            let dst = cpu.register_reference_u16(reg);
            let result = *dst ^ src;
            *dst = result;
            cpu.set_pzs(result);
            2
        } else {
            let addr = cpu.memory_address(mode, mem);
            let src  = cpu.read_u16(addr);
            let dst  = cpu.register_reference_u16(reg);
            let result = *dst ^ src;
            *dst = result;
            cpu.set_pzs(result);
            if addr % 2 == 0 { 6 } else { 8 }
        }
    },

    0x34 => unimplemented("XOR"),

    0x35 => unimplemented("XOR"),

    0x36 => ss,

    0x37 => unimplemented("ADJBA"),

    0x38 => cmp_b_f_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        if mode == 0b11 {
            let src = state.register_value_u8(reg);
            let dst = state.register_value_u8(mem);
            let (result, unsigned_overflow) = dst.overflowing_sub(src);
            let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
            state.set_pzs(result as u16);
            state.set_cy(unsigned_overflow);
            state.set_v(signed_overflow);
            2
        } else {
            let src  = state.register_value_u8(reg);
            let addr = state.memory_address(mode, mem);
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
    },

    0x39 => unimplemented("Compare memory with word"),

    0x3A => unimplemented("Compare byte with memory"),

    0x3B => cmp_w_t_rm (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        if mode == 0b11 {
            let src = cpu.register_value_u16(mem);
            let dst = cpu.register_reference_u16(reg);
            let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
            let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
            cpu.set_pzs(result);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            2
        } else {
            let addr = cpu.memory_address(mode, mem);
            let src  = cpu.read_u16(addr);
            let dst  = cpu.register_reference_u16(reg);
            let (result, unsigned_overflow) = (*dst).overflowing_sub(src);
            let (_, signed_overflow) = (*dst as i16).overflowing_sub(src as i16);
            cpu.set_pzs(result);
            cpu.set_cy(unsigned_overflow);
            cpu.set_v(signed_overflow);
            if addr % 2 == 0 {
                6
            } else {
                8
            }
        }
    },

    0x3C => unimplemented("CMP b, ia"),

    0x3D => unimplemented("CMP w, ia"),

    0x3E => ds0,

    0x3F => unimplemented("ADJBS"),

    0x40 => inc_aw,
    0x41 => inc_cw,
    0x42 => inc_dw,
    0x43 => inc_bw,

    0x44 => inc_sp,
    0x45 => inc_bp,
    0x46 => inc_ix,
    0x47 => inc_iy,

    0x48 => dec_aw,
    0x49 => dec_cw,
    0x4A => dec_dw,
    0x4B => dec_bw,

    0x4C => dec_sp,
    0x4D => dec_bp,
    0x4E => dec_ix,
    0x4F => dec_iy,

    0x50 => push_aw,
    0x51 => push_cw,
    0x52 => push_dw,
    0x53 => push_bw,

    0x54 => push_sp,
    0x55 => push_bp,
    0x56 => push_ix,
    0x57 => push_iy,

    0x58 => pop_aw,
    0x59 => pop_cw,
    0x5A => pop_dw,
    0x5B => pop_bw,

    0x5C => pop_sp,
    0x5D => pop_bp,
    0x5E => pop_ix,
    0x5F => pop_iy,

    0x60 => unimplemented("PUSH R"),

    0x61 => unimplemented("POP R"),

    0x62 => unimplemented("CHKIND"),

    0x63 => unimplemented("UNDEF"),

    0x64 => unimplemented("REPNC"),

    0x65 => unimplemented("REPC"),

    0x66 => unimplemented("FPO2"),

    0x67 => unimplemented("FPO2"),

    0x68 => unimplemented("PUSH"),

    0x69 => unimplemented("MUL"),

    0x6A => unimplemented("PUSH"),

    0x6B => unimplemented("MUL"),

    0x6C => unimplemented("INM"),

    0x6D => unimplemented("INM"),

    0x6E => outm_b (cpu) {
        let data = state.read_u8(state.ix);
        state.output_u8(state.dw, data);
        if state.dir() {
            state.ix = state.ix - 1;
        } else {
            state.ix = state.ix + 1;
        }
        let rep = 1; // TODO
        8 * rep - 2
    },

    0x6F => outm_w (cpu) {
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
    },

    0x70 => unimplemented("BV"),

    0x71 => unimplemented("BNV"),

    0x72 => bc (cpu) {
        let displace = state.next_i8();
        if state.cy() { state.jump_i8(displace); 6 } else { 3 }
    },

    0x73 => bnc (cpu) {
        let displace = state.next_i8();
        if !state.cy() { state.jump_i8(displace); 6 } else { 3 }
    },

    0x74 => be (cpu) {
        let displace = state.next_i8();
        if state.z() { state.jump_i8(displace); 6 } else { 3 }
    },

    0x75 => bne (cpu) {
        let displace = state.next_i8();
        if !state.z() { state.jump_i8(displace); 6 } else { 3 }
    },

    0x76 => unimplemented("BNH"),

    0x77 => unimplemented("BH"),

    0x78 => unimplemented("BN"),

    0x79 => unimplemented("BP"),

    0x7A => unimplemented("BPE"),

    0x7B => unimplemented("BPO"),

    0x7C => unimplemented("BLT"),

    0x7D => unimplemented("BGE"),

    0x7E => unimplemented("BLE"),

    0x7F => unimplemented("BGT"),

    0x80 => imm_b,

    0x81 => imm_w,

    0x82 => imm_b_s,

    0x83 => imm_w_s,

    0x84 => unimplemented!("TEST"),

    0x85 => unimplemented!("TEST"),

    0x86 => unimplemented!("XCH"),

    0x87 => unimplemented!("XCH"),

    0x88 => unimplemented!("MOV"),

    0x89 => mov_w_from_reg_to_mem (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        let addr = state.memory_address(mode, mem)
        let val = state.register_value_u16(reg);
        state.write_u16(addr, val);
        if addr % 2 == 0 { 3 } else { 5 }
    },

    0x8A => unimplemented("MOV"),

    0x8B => mov_w_to_reg (cpu) {
        let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
        if mode == 0b11 {
            let src = state.register_value_u16(mem);
            let dst = state.register_reference_u16(reg);
            *dst = src;
            2
        } else {
            let value = state.next_u16();
            if mode == 0b01 {
                match mem {
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
                match mem {
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
                match mem {
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
    },

    0x8C => mov_w_from_sreg (cpu) {
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
            if addr % 2 == 0 { 3 } else { 5 }
        }
    },

    0x8D => unimplemented!("LDEA"),

    0x8E => mov_w_to_sreg (cpu) {
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
    },

    0x8F => unimplemented("POP rm"),

    0x90 => nop,
    0x91 => unimplemented("XCH CW"),
    0x92 => unimplemented("XCH DW"),
    0x93 => unimplemented("XCH BW"),

    0x94 => unimplemented("XCH SP"),
    0x95 => unimplemented("XCH BP"),
    0x96 => unimplemented("XCH IX"),
    0x97 => unimplemented("XCH IY"),

    0x98 => unimplemented("CVTBW"),

    0x99 => unimplemented("CVTBL"),

    0x9A => unimplemented("CALL"),

    0x9B => unimplemented("POLL"),

    0x9C => push_psw,

    0x9D => pop_psw,

    0x9E => unimplemented("MOV PSW, AH"),

    0x9F => unimplemented("MOV AH, PSW"),

    0xA0 => unimplemented("MOV al m"),

    0xA1 => unimplemented("MOV aw m"),

    0xA2 => unimplemented("MOV m al"),

    0xA3 => unimplemented("MOV m aw"),

    0xA4 => unimplemented("MOVBK b"),

    0xA5 => movbk_w (cpu) {
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
    },

    0xA6 => unimplemented("CMPBK"),

    0xA7 => unimplemented("CMPBK"),

    0xA8 => unimplemented("TEST"),

    0xA9 => unimplemented("TEST"),

    0xAA => stm_b (cpu) {
        let iy = state.iy();
        state.write_u8(state.ds1_address(iy) as u16, state.al());
        state.set_iy(if state.dir() {
            iy.overflowing_sub(1).0
        } else {
            iy.overflowing_add(1).0
        });
        if iy % 2 == 0 { 3 } else { 5 }
    },

    0xAB => stm_w (cpu) {
        let iy = state.iy();
        state.write_u16(state.ds1_address(iy) as u16, state.aw());
        state.set_iy(if state.dir() {
            iy.overflowing_sub(2).0
        } else {
            iy.overflowing_add(2).0
        });
        if iy % 2 == 0 { 3 } else { 5 }
    },

    0xAC => ldm_b (cpu) {
        let data = state.read_u8(state.ix);
        state.set_al(data);
        if state.dir() {
            state.ix = state.ix - 1;
        } else {
            state.ix = state.ix + 1;
        }
        5
    },

    0xAD => ldm_w (cpu) {
        let data = state.read_u16(state.ix);
        state.aw = data;
        if state.dir() {
            state.ix = state.ix - 2;
        } else {
            state.ix = state.ix + 2;
        }
        if state.ix % 2 == 1 { 7 } else { 5 }
    },

    0xAE => unimplemented("CMPM"),

    0xAF => unimplemented("CMPM"),

    0xB0 => mov_al_i,
    0xB1 => mov_cl_i,
    0xB2 => mov_dl_i,
    0xB3 => mov_bl_i,

    0xB4 => mov_ah_i,
    0xB5 => mov_ch_i,
    0xB6 => mov_dh_i,
    0xB7 => mov_bh_i,

    0xB8 => mov_aw_i,
    0xB9 => mov_cw_i,
    0xBA => mov_dw_i,
    0xBB => mov_bw_i,

    0xBC => mov_sp_i,
    0xBD => mov_bp_i,
    0xBE => mov_ix_i,
    0xBF => mov_iy_i,

    0xC0 => unimplemented("SHIFT"),
    0xC1 => unimplemented("SHIFT"),
    0xC2 => unimplemented("RET"),
    0xC3 => unimplemented("REF"),
    0xC4 => mov_ds1_aw (cpu) {
        state.ds1 = state.aw;
        if state.aw % 2 == 0 { 10 } else { 14 }
    },
    0xC5 => unimplemented("MOV DS0, AW"),
    0xC6 => mov_mb_imm (cpu) {
        let arg  = state.next_u8();
        let mode = (arg & B_MODE) >> 6;
        let code = (arg & B_REG)  >> 3;
        if code != 0b000 {
            panic!();
        }
        let mem  = (arg & B_MEM)  >> 0;
        let addr = state.memory_address(mode, mem);
        let imm  = state.next_u8();
        state.write_u8(addr, imm);
        3
    },
    0xC7 => unimplemented!("mov mw imm"),
    0xC8 => unimplemented!("PREPARE"),
    0xC9 => unimplemented!("DISPOSE"),
    0xCA => unimplemented!("RET"),
    0xCB => unimplemented!("RET"),
    0xCC => unimplemented!("BRK"),
    0xCD => unimplemented!("BRK"),
    0xCE => unimplemented!("BRKV"),
    0xCF => unimplemented!("RETI"),

    0xD0 => unimplemented!("SHIFT b"),
    0xD1 => shift_w (cpu) {
        let arg = state.next_u8();
        let code = (arg & B_REG) >> 3;
        let source = get_source_word(state, arg);
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
    },
    0xD2 => unimplemented("SHIFT b, port"),
    0xD3 => unimplemented("SHIFT b, port"),
    0xD4 => unimplemented("CVTBD"),
    0xD5 => unimplemented("CVTDB"),
    0xD6 => unimplemented("UNDEF"),
    0xD7 => unimplemented("TRANS"),
    0xD8 => unimplemented("FPO1"),
    0xD9 => unimplemented("FPO1"),
    0xDA => unimplemented("FPO1"),
    0xDB => unimplemented("FPO1"),
    0xDC => unimplemented("FPO1"),
    0xDD => unimplemented("FPO1"),
    0xDE => unimplemented("FPO1"),
    0xDF => unimplemented("FPO1"),

    0xE0 => unimplemented("DBNZE"),

    0xE1 => unimplemented("DBNZE"),

    0xE2 => dbnz (cpu) {
        let displace = state.next_i8();
        state.cw = state.cw.overflowing_sub(1).0;
        if state.cw > 0 { state.jump_i8(displace); 6 } else { 3 }
    },

    0xE3 => bcwz (cpu) {
        let displace = state.next_i8();
        if state.cw() == 0 { state.jump_i8(displace); 6 } else { 3 }
    },

    0xE4 => in_b (cpu) {
        let addr = state.next_u16();
        let data = state.input_u8(addr);
        state.set_al(data);
        5
    },

    0xE5 => in_w (cpu) {
        let addr = state.next_u16();
        let data = state.input_u16(addr);
        state.aw = data;
        7
    },

    0xE6 => out_b (cpu) {
        let addr = state.next_u16();
        let data = state.al();
        state.output_u8(addr, data);
        3
    },

    0xE7 => out_w (cpu) {
        let addr = state.next_u16();
        let data = state.aw;
        state.output_u16(addr, data);
        5
    },

    0xE8 => call_d (cpu) {
        let displace = state.next_i16();
        state.push_u16(state.pc);
        state.jump_i16(displace);
        if state.pc % 1 == 0 { 7 } else { 9 }
    },

    0xE9 => br_near (cpu) {
        let displace = state.next_i16();
        state.jump_i16(displace);
        7
    },

    0xEA => br_far (cpu) {
        let offset  = state.next_u16();
        let segment = state.next_u16();
        state.pc = offset;
        state.ps = segment;
        7
    },

    0xEB => br_short (cpu) {
        let displace = state.next_i8();
        state.jump_i8(displace);
        7
    },

    0xEC => in_b_v (cpu) {
        let addr = state.dw;
        let data = state.input_u8(addr);
        state.set_al(data);
        5
    },

    0xED => in_w_v (cpu) {
        let addr = state.dw;
        let data = state.input_u16(addr);
        state.aw = data;
        7
    },

    0xEE => out_b_v (cpu) {
        let addr = state.dw;
        let data = state.al();
        state.output_u8(addr, data);
        3
    },

    0xEF => out_w_v (cpu) {
        let addr = state.dw;
        let data = state.aw;
        state.output_u16(addr, data);
        5
    },

    0xF0 => unimplemented!("BUSLOCK"),

    0xF1 => unimplemented!("UNDEFINED"),

    0xF2 => unimplemented!("REPNE"),

    0xF3 => rep (cpu) {
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
    },

    0xF4 => unimplemented("HALT"),

    0xF5 => unimplemented("NOT1"),

    0xF6 => group1_b,

    0xF7 => group1_w,

    0xF8 => clr1_cy,

    0xF9 => set1_cy,

    0xFA => di (cpu) {
        state.set_ie(false);
        2
    },

    0xFB => ei,

    0xFC => clr1_dir,

    0xFD => set1_dir,

    0xFE => group2_b,

    0xFF => group2_w,

}

#[inline]
fn nop (state: &mut CPU) -> u64 {
    1
}

#[inline]
fn unimplemented (state: &mut CPU) -> u64 {
    unimplemented!("opcode {:x}", state.opcode())
}

#[inline]
fn group2_b (state: &mut CPU) -> u64 {
    unimplemented!();
}

#[inline]
fn group2_w (state: &mut CPU) -> u64 {
    let arg  = state.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let code = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    match code {
        0b000 => {
            unimplemented!("inc");
        },
        0b001 => {
            unimplemented!("dec");
        },
        0b010 => {
            unimplemented!("call regptr16/memptr16");
        },
        0b011 => {
            let addr = state.memory_address(mode, mem) as i32;
            let pc = state.read_u16(addr as u16 + 0);
            let ps = state.read_u16(addr as u16 + 2);
            state.set_sp(state.sp() - 2);
            state.write_u16(state.sp(), state.ps());
            state.set_ps(ps);
            state.set_sp(state.sp() - 2);
            state.write_u16(state.sp(), state.pc());
            state.set_pc(pc);
            if addr % 2 == 0 { 15 } else { 23 }
        },
        0b100 => {
            unimplemented!("br");
        },
        0b101 => {
            unimplemented!("br");
        },
        0b110 => {
            unimplemented!("push");
        },
        0b111 => {
            panic!("undefined instruction 0b111")
        },
        _ => {
            unreachable!("imm code {code:b}");
        }
    }
}

#[inline]
fn group3_instruction (state: &mut CPU) -> u64 {
    let opcode = state.next_u8();
    group3::execute_instruction(state, opcode)
}

mod group3 {
    use mpcemu_core::define_instruction_set;
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
    // temp1 ← (imm8 × 4 + 1, imm8 × 4);
    // temp2 ← (imm8 × 4 + 3, imm8 × 4 + 2);
    // XA ← 1;
    // PC ← temp1;
    // PS ← temp2.
    fn brkxa (state: &mut CPU) -> u64 {
        let addr = state.next_u8() as usize;
        //panic!("{addr} {:x?}", &state.memory[addr*4..addr*4+4]);
        state.pc = u16::from_le_bytes([
            state.get_byte(addr as usize * 4 + 0),
            state.get_byte(addr as usize * 4 + 1),
        ]);
        state.ps = u16::from_le_bytes([
            state.get_byte(addr as usize * 4 + 2),
            state.get_byte(addr as usize * 4 + 3),
        ]);
        state.set_xa(true);
        //println!("\n==========BRKXA {:x} {:x} {:x} {:x}", addr, state.pc, state.ps, state.program_address());
        // TODO: set XA (internal I/O address: FF80H)
        12
    }

    #[inline]
    /// temp1 ← (imm8 × 4 + 1, imm8 × 4);
    /// temp2 ← (imm8 × 4 + 3, imm8 × 4 + 2);
    /// XA ← 0;
    /// PC ← temp1;
    /// PS ← temp2.
    fn retxa (state: &mut CPU) -> u64 {
        let addr = state.next_u8();
        state.pc = u16::from_le_bytes([
            state.get_byte(addr as usize * 4 + 0),
            state.get_byte(addr as usize * 4 + 1),
        ]);
        state.ps = u16::from_le_bytes([
            state.get_byte(addr as usize * 4 + 2),
            state.get_byte(addr as usize * 4 + 3),
        ]);
        state.set_xa(false);
        // TODO: reset XA
        12
    }
}
