use crate::*;

pub fn v53_instruction (cpu: &mut CPU, op: u8) -> (
    String,
    Vec<u8>,
    Box<dyn Fn(&mut CPU)->u64>
) {
    match op {

        0x00 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("ADD mem, reg"), vec![op, arg], Box::new(move |cpu: &mut CPU| {
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
            }))
        },

        0x01 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("ADDW mem, reg"), vec![op, arg], Box::new(move |cpu: &mut CPU| {
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
            }))
        },

        0x02 => unimplemented!("ADD b, t, rm"),
        0x03 => unimplemented!("ADD w, t, rm"),
        0x04 => unimplemented!("ADD b, ia"),

        0x05 => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("ADD AW, {word}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                let (result, unsigned_overflow) = cpu.aw().overflowing_add(word);
                let (_, signed_overflow) = (cpu.aw() as i16).overflowing_add(word as i16);
                cpu.set_aw(result);
                cpu.set_pzs(result);
                cpu.set_cy(unsigned_overflow);
                cpu.set_v(signed_overflow);
                2
            }))
        },

        0x06 => (format!("PUSH DS1"), vec![op], Box::new(push_ds1)),
        0x07 => (format!("POP DS1"), vec![op], Box::new(pop_ds1)),

        0x08 => unimplemented!("Byte bitwise OR to memory from register"),
        0x09 => unimplemented!("Word bitwise OR to memory from register"),
        0x0A => unimplemented!("Byte bitwise OR to register from memory"),

        0x0B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("OR"), vec![0x0B, arg], Box::new(move |cpu: &mut CPU|{
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
            }))
        },

        0x0C => unimplemented!("Bitwise OR b ia"),
        0x0D => unimplemented!("Bitwise OR w ia"),

        0x0E => (format!("PUSH PS"), vec![op], Box::new(push_ps)),

        0x0F => {
            let arg = cpu.next_u8();
            match arg {

                0xE0 => (format!("BRKXA"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.next_u8() as usize;
                    //panic!("{addr} {:x?}", &cpu.memory[addr*4..addr*4+4]);
                    cpu.pc = u16::from_le_bytes([
                        cpu.get_byte(addr as usize * 4 + 0),
                        cpu.get_byte(addr as usize * 4 + 1),
                    ]);
                    cpu.ps = u16::from_le_bytes([
                        cpu.get_byte(addr as usize * 4 + 2),
                        cpu.get_byte(addr as usize * 4 + 3),
                    ]);
                    cpu.set_xa(true);
                    //println!("\n==========BRKXA {:x} {:x} {:x} {:x}", addr, cpu.pc, cpu.ps, cpu.program_address());
                    // TODO: set XA (internal I/O address: FF80H)
                    12
                })),

                0xF0 => (format!("RETXA"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.next_u8();
                    cpu.pc = u16::from_le_bytes([
                        cpu.get_byte(addr as usize * 4 + 0),
                        cpu.get_byte(addr as usize * 4 + 1),
                    ]);
                    cpu.ps = u16::from_le_bytes([
                        cpu.get_byte(addr as usize * 4 + 2),
                        cpu.get_byte(addr as usize * 4 + 3),
                    ]);
                    cpu.set_xa(false);
                    // TODO: reset XA
                    12
                })),

                _ => unimplemented!("unimplemented Group 3 instruction {arg}")
            }
        },

        0x10 => unimplemented!("ADDC"),
        0x11 => unimplemented!("ADDC"),
        0x12 => unimplemented!("ADDC"),
        0x13 => unimplemented!("ADDC"),
        0x14 => unimplemented!("ADDC"),
        0x15 => unimplemented!("ADDC"),

        0x16 => (format!("PUSH SS"), vec![op], Box::new(push_ss)),
        0x17 => (format!("POP SS"),  vec![op], Box::new(pop_ss)),

        0x18 => unimplemented!("SUBC"),
        0x19 => unimplemented!("SUBC"),
        0x1A => unimplemented!("SUBC"),
        0x1B => unimplemented!("SUBC"),
        0x1C => unimplemented!("SUBC"),
        0x1D => unimplemented!("SUBC"),

        0x1E => (format!("PUSH DS0"), vec![op], Box::new(push_ds0)),
        0x1F => (format!("POP DS0"), vec![op], Box::new(pop_ds0)),

        0x20 => unimplemented!("AND"),
        0x21 => unimplemented!("AND"),
        0x22 => unimplemented!("AND"),
        0x23 => unimplemented!("AND"),
        0x24 => unimplemented!("AND"),
        0x25 => unimplemented!("AND"),

        0x26 => (format!("DS1:"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.segment = Some(Segment::DS1);
            2
        })),

        0x27 => unimplemented!("ADJ4A"),

        0x28 => unimplemented!("SUB b f rm"),

        0x29 => unimplemented!("SUB w f rm"),

        0x2A => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("SUB"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
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
            }))
        },

        0x2B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("SUB"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                if mode == 0b11 {
                    let src = cpu.register_value_u8(mem);
                    let dst = cpu.register_value_u8(reg);
                    let (result, unsigned_overflow) = dst.overflowing_sub(src);
                    let (_, signed_overflow) = (dst as i16).overflowing_sub(src as i16);
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
                    let (_, signed_overflow) = (dst as i16).overflowing_sub(src as i16);
                    cpu.set_register_u8(reg, result);
                    cpu.set_pzs(result as u16);
                    cpu.set_cy(unsigned_overflow);
                    cpu.set_v(signed_overflow);
                    if addr % 2 == 0 { 6 } else { 8 }
                }
            }))
        },

        0x2C => unimplemented!("SUB b, ia"),
        0x2D => unimplemented!("SUB w, ia"),

        0x2E => (format!("PS:"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.segment = Some(Segment::PS);
            2
        })),

        0x2F => unimplemented!("ADJ4S"),
        0x30 => unimplemented!("XOR"),
        0x31 => unimplemented!("XOR"),
        0x32 => unimplemented!("XOR"),

        0x33 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("XOR"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
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
            }))
        },

        0x34 => unimplemented!("XOR"),
        0x35 => unimplemented!("XOR"),

        0x36 => (format!("SS:"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.segment = Some(Segment::PS);
            2
        })),

        0x37 => unimplemented!("ADJBA"),

        0x38 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("CMP"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                if mode == 0b11 {
                    let src = cpu.register_value_u8(reg);
                    let dst = cpu.register_value_u8(mem);
                    let (result, unsigned_overflow) = dst.overflowing_sub(src);
                    let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzs(result as u16);
                    cpu.set_cy(unsigned_overflow);
                    cpu.set_v(signed_overflow);
                    2
                } else {
                    let src  = cpu.register_value_u8(reg);
                    let addr = cpu.memory_address(mode, mem);
                    let dst  = cpu.read_u8(addr);
                    let (result, unsigned_overflow) = dst.overflowing_sub(src);
                    let (_, signed_overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzs(result as u16);
                    cpu.set_cy(unsigned_overflow);
                    cpu.set_v(signed_overflow);
                    if addr % 2 == 0 {
                        6
                    } else {
                        8
                    }
                }
            }))
        },

        0x39 => unimplemented!("Compare memory with word"),
        0x3A => unimplemented!("Compare byte with memory"),

        0x3B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("CMP"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
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
            }))
        },

        0x3C => unimplemented!("CMP b, ia"),
        0x3D => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("CMP w ia"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                let (result, unsigned_overflow) = cpu.aw.overflowing_sub(word);
                let (_, signed_overflow) = (cpu.aw as i16).overflowing_sub(word as i16);
                cpu.set_pzs(result);
                cpu.set_cy(unsigned_overflow);
                cpu.set_v(signed_overflow);
                2
            }))
        },

        0x3E => (format!("DS0:"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.segment = Some(Segment::DS0);
            2
        })),

        0x3F => unimplemented!("ADJBS"),

        0x40 => (format!("INC AW"), vec![op], Box::new(inc_aw)),
        0x41 => (format!("INC CW"), vec![op], Box::new(inc_cw)),
        0x42 => (format!("INC DW"), vec![op], Box::new(inc_dw)),
        0x43 => (format!("INC BW"), vec![op], Box::new(inc_bw)),

        0x44 => (format!("INC SP"), vec![op], Box::new(inc_sp)),
        0x45 => (format!("INC BP"), vec![op], Box::new(inc_bp)),
        0x46 => (format!("INC IX"), vec![op], Box::new(inc_ix)),
        0x47 => (format!("INC IY"), vec![op], Box::new(inc_iy)),

        0x48 => (format!("DEC AW"), vec![op], Box::new(dec_aw)),
        0x49 => (format!("DEC CW"), vec![op], Box::new(dec_cw)),
        0x4A => (format!("DEC DW"), vec![op], Box::new(dec_dw)),
        0x4B => (format!("DEC BW"), vec![op], Box::new(dec_bw)),

        0x4C => (format!("DEC SP"), vec![op], Box::new(dec_sp)),
        0x4D => (format!("DEC BP"), vec![op], Box::new(dec_bp)),
        0x4E => (format!("DEC IX"), vec![op], Box::new(dec_ix)),
        0x4F => (format!("DEC IY"), vec![op], Box::new(dec_iy)),

        0x50 => (format!("PUSH AW"), vec![op], Box::new(push_aw)),
        0x51 => (format!("PUSH CW"), vec![op], Box::new(push_cw)),
        0x52 => (format!("PUSH DW"), vec![op], Box::new(push_dw)),
        0x53 => (format!("PUSH BW"), vec![op], Box::new(push_bw)),

        0x54 => (format!("PUSH SP"), vec![op], Box::new(push_sp)),
        0x55 => (format!("PUSH BP"), vec![op], Box::new(push_bp)),
        0x56 => (format!("PUSH IX"), vec![op], Box::new(push_ix)),
        0x57 => (format!("PUSH IY"), vec![op], Box::new(push_iy)),

        0x58 => (format!("PUSH AW"), vec![op], Box::new(pop_aw)),
        0x59 => (format!("PUSH CW"), vec![op], Box::new(pop_cw)),
        0x5A => (format!("PUSH DW"), vec![op], Box::new(pop_dw)),
        0x5B => (format!("PUSH BW"), vec![op], Box::new(pop_bw)),

        0x5C => (format!("POP SP"), vec![op], Box::new(pop_sp)),
        0x5D => (format!("POP BP"), vec![op], Box::new(pop_bp)),
        0x5E => (format!("POP IX"), vec![op], Box::new(pop_ix)),
        0x5F => (format!("POP IY"), vec![op], Box::new(pop_iy)),

        0x60 => unimplemented!("PUSH R"),
        0x61 => unimplemented!("POP R"),
        0x62 => unimplemented!("CHKIND"),
        0x63 => unimplemented!("UNDEF"),
        0x64 => unimplemented!("REPNC"),
        0x65 => unimplemented!("REPC"),
        0x66 => unimplemented!("FPO2"),
        0x67 => unimplemented!("FPO2"),
        0x68 => unimplemented!("PUSH"),
        0x69 => unimplemented!("MUL"),
        0x6A => unimplemented!("PUSH"),
        0x6B => unimplemented!("MUL"),
        0x6C => unimplemented!("INM"),
        0x6D => unimplemented!("INM"),

        0x6E => (format!("OUTM"), vec![op], Box::new(move |cpu: &mut CPU|{
            let data = cpu.read_u8(cpu.ix);
            cpu.output_u8(cpu.dw, data);
            if cpu.dir() {
                cpu.ix = cpu.ix - 1;
            } else {
                cpu.ix = cpu.ix + 1;
            }
            let rep = 1; // TODO
            8 * rep - 2
        })),

        0x6F => (format!("OUTMW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let data = cpu.read_u16(cpu.ix);
            cpu.output_u16(cpu.dw, data);
            if cpu.dir() {
                cpu.ix = cpu.ix - 2;
            } else {
                cpu.ix = cpu.ix + 2;
            }
            let rep = 1; // TODO
            if (cpu.dw % 2 == 1) && (cpu.ix % 2 == 1) {
                14 * rep - 2
            } else if cpu.dw % 2 == 1 {
                12 * rep - 2
            } else if cpu.ix % 2 == 1 {
                10 * rep - 2
            } else {
                8 * rep - 2
            }
        })),

        0x70 => unimplemented!("BV"),
        0x71 => unimplemented!("BNV"),

        0x72 => {
            let arg = cpu.next_i8();
            (
                format!("BC"),
                vec![op, arg as u8],
                Box::new(move |cpu: &mut CPU|{
                    if cpu.cy() { cpu.jump_i8(arg); 6 } else { 3 }
                })
            )
        },

        0x73 => {
            let arg = cpu.next_i8();
            (
                format!("BNC"),
                vec![op, arg as u8],
                Box::new(move |cpu: &mut CPU| {
                    if !cpu.cy() { cpu.jump_i8(arg); 6 } else { 3 }
                })
            )
        },

        0x74 => {
            let arg = cpu.next_i8();
            (
                format!("BE"),
                vec![op, arg as u8],
                Box::new(move |cpu: &mut CPU| {
                    if cpu.z() { cpu.jump_i8(arg); 6 } else { 3 }
                })
            )
        },

        0x75 => {
            let arg = cpu.next_i8();
            (
                format!("BNE"),
                vec![op, arg as u8],
                Box::new(move |cpu: &mut CPU| {
                    if !cpu.z() { cpu.jump_i8(arg); 6 } else { 3 }
                })
            )
        },

        0x76 => unimplemented!("BNH"),
        0x77 => unimplemented!("BH"),
        0x78 => unimplemented!("BN"),
        0x79 => unimplemented!("BP"),
        0x7A => unimplemented!("BPE"),
        0x7B => unimplemented!("BPO"),
        0x7C => unimplemented!("BLT"),
        0x7D => unimplemented!("BGE"),
        0x7E => unimplemented!("BLE"),
        0x7F => unimplemented!("BGT"),

        0x80 => (format!("IMM"), vec![op], Box::new(imm_b)),

        0x81 => (format!("IMM"), vec![op], Box::new(imm_w)),

        0x82 => (format!("IMM"), vec![op], Box::new(imm_b_s)),

        0x83 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, 0b11) => {
                    let src = cpu.next_u16() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (format!("ADDW"), vec![op, arg, lo, hi], Box::new(move |cpu: &mut CPU|{
                        let dst = cpu.register_value_u16(mem) as i16;
                        let (result, unsigned_overflow) = (dst as u16).overflowing_add(src as u16);
                        let (_, signed_overflow) = dst.overflowing_add(src);
                        cpu.set_register_u16(mem, result);
                        cpu.set_pzs(result);
                        cpu.set_cy(unsigned_overflow);
                        cpu.set_v(signed_overflow);
                        2
                    }))
                },
                (0b000, _)    => {
                    let src = cpu.next_u16() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (format!("ADDW"), vec![op, arg, lo, hi], Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let dst = cpu.read_u16(addr);
                        let (result, unsigned_overflow) = (dst as u16).overflowing_add(src as u16);
                        let (_, signed_overflow) = dst.overflowing_add(src as u16);
                        cpu.set_register_u16(mem, result);
                        cpu.set_pzs(result);
                        cpu.set_cy(unsigned_overflow);
                        cpu.set_v(signed_overflow);
                        if addr % 2 == 0 { 6 } else { 8 }
                    }))
                },
                (0b001, _) => (format!("ORW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    unimplemented!()
                })),
                (0b010, _) => (format!("ADDCW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    unimplemented!()
                })),
                (0b011, _) => (format!("SUBW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    unimplemented!()
                })),
                (0b100, _) => (format!("ANDW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    unimplemented!()
                })),
                (0b101, _) => (format!("SUBW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    unimplemented!()
                })),
                (0b110, _) => (format!("XORW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    unimplemented!()
                })),
                // FIXME: fetch 1 more byte
                (0b111, 0b11) => {
                    let src = cpu.next_u16() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (format!("CMPW"), vec![op, arg, lo, hi], Box::new(move |cpu: &mut CPU|{
                        let dst = cpu.register_value_u16(mem) as i16;
                        let (result, unsigned_overflow) = (dst as u16).overflowing_sub(src as u16);
                        let (_, signed_overflow) = dst.overflowing_sub(src);
                        cpu.set_pzs(result);
                        cpu.set_cy(unsigned_overflow);
                        cpu.set_v(signed_overflow);
                        2
                    }))
                },
                // FIXME: fetch 1 more byte
                (0b111, _) => {
                    let src = cpu.next_u16() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (format!("CMPW"), vec![op, arg, lo, hi], Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let dst = cpu.read_u16(addr);
                        let (result, unsigned_overflow) = (dst as u16).overflowing_sub(src as u16);
                        let (_, signed_overflow) = dst.overflowing_sub(src as u16);
                        cpu.set_pzs(result);
                        cpu.set_cy(unsigned_overflow);
                        cpu.set_v(signed_overflow);
                        if addr % 2 == 0 { 6 } else { 8 }
                    }))
                },
                _ => unreachable!()
            }
        },

        0x84 => unimplemented!("TEST"),

        0x85 => unimplemented!("TEST"),

        0x86 => unimplemented!("XCH"),

        0x87 => unimplemented!("XCH"),

        0x88 => unimplemented!("MOV"),

        0x89 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("MOVW mem, reg"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let addr = cpu.memory_address(mode, mem);
                let val = cpu.register_value_u16(reg);
                cpu.write_u16(addr, val);
                if addr % 2 == 0 { 3 } else { 5 }
            }))
        },

        0x8A => unimplemented!("MOV"),

        0x8B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("MOVW reg, mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                if mode == 0b11 {
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.register_reference_u16(reg);
                    *dst = src;
                    2
                } else {
                    unimplemented!();
                }
            }))
        },

        0x8C => {
            let [arg, mode, sreg, mem] = get_mode_sreg_mem(cpu);
            (format!("MOV mem, sreg"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let value = cpu.segment_register_value(sreg);
                if mode == 0b11 {
                    let dst = cpu.register_reference_u16(mem);
                    *dst = value;
                    2
                } else {
                    let addr = cpu.memory_address(mode, mem);
                    cpu.write_u16(addr, value);
                    if addr % 2 == 0 { 3 } else { 5 }
                }
            }))
        },

        0x8D => unimplemented!("LDEA"),

        0x8E => {
            let [arg, mode, sreg, mem] = get_mode_sreg_mem(cpu);
            (format!("MOVW sreg, mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                if mode == 0b11 {
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.segment_register_reference(sreg);
                    *dst = src;
                    2
                } else {
                    let _value = cpu.next_u16();
                    unimplemented!();
                }
            }))
        },

        0x8F => unimplemented!("POP rm"),

        0x90 => (format!("NOP"), vec![op], Box::new(nop)),

        0x91 => unimplemented!("XCH CW"),
        0x92 => unimplemented!("XCH DW"),
        0x93 => unimplemented!("XCH BW"),
        0x94 => unimplemented!("XCH SP"),
        0x95 => unimplemented!("XCH BP"),
        0x96 => unimplemented!("XCH IX"),
        0x97 => unimplemented!("XCH IY"),
        0x98 => unimplemented!("CVTBW"),
        0x99 => unimplemented!("CVTBL"),
        0x9A => unimplemented!("CALL"),
        0x9B => unimplemented!("POLL"),

        0x9C => (format!("PUSH PSW"), vec![op], Box::new(push_psw)),

        0x9D => (format!("POP PSW"), vec![op], Box::new(pop_psw)),

        0x9E => unimplemented!("MOV PSW, AH"),

        0x9F => unimplemented!("MOV AH, PSW"),

        0xA0 => unimplemented!("MOV al m"),

        0xA1 => unimplemented!("MOV aw m"),

        0xA2 => unimplemented!("MOV m al"),

        0xA3 => unimplemented!("MOV m aw"),

        0xA4 => unimplemented!("MOVBK b"),

        0xA5 => (format!("MOVBKW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let dst = cpu.ds1() as u32 * 0x10 + cpu.iy() as u32;
            let src = cpu.effective_address(cpu.ix());
            cpu.set_byte(dst as usize + 0, cpu.get_byte(src as usize + 0));
            cpu.set_byte(dst as usize + 1, cpu.get_byte(src as usize + 1));
            if cpu.dir() {
                cpu.set_ix(cpu.ix() - 2);
                cpu.set_iy(cpu.iy() - 2);
            } else {
                cpu.set_ix(cpu.ix() + 2);
                cpu.set_iy(cpu.iy() + 2);
            }
            if (dst % 2 == 0) && (src % 2 == 0) {
                6
            } else if (dst % 2 == 1) && (src % 2 == 1) {
                10
            } else {
                8
            }
        })),

        0xA6 => unimplemented!("CMPBK"),
        0xA7 => unimplemented!("CMPBK"),
        0xA8 => unimplemented!("TEST"),
        0xA9 => unimplemented!("TEST"),

        0xAA => (format!("STM"), vec![op], Box::new(move |cpu: &mut CPU| {
            let iy = cpu.iy();
            cpu.write_u8(cpu.ds1_address(iy) as u16, cpu.al());
            cpu.set_iy(if cpu.dir() {
                iy.overflowing_sub(1).0
            } else {
                iy.overflowing_add(1).0
            });
            if iy % 2 == 0 { 3 } else { 5 }
        })),

        0xAB => (format!("STMW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let iy = cpu.iy();
            cpu.write_u16(cpu.ds1_address(iy) as u16, cpu.aw());
            cpu.set_iy(if cpu.dir() {
                iy.overflowing_sub(2).0
            } else {
                iy.overflowing_add(2).0
            });
            if iy % 2 == 0 { 3 } else { 5 }
        })),

        0xAC => (format!("LDM"), vec![op], Box::new(move |cpu: &mut CPU| {
            let data = cpu.read_u8(cpu.ix);
            cpu.set_al(data);
            if cpu.dir() {
                cpu.ix = cpu.ix - 1;
            } else {
                cpu.ix = cpu.ix + 1;
            }
            5
        })),

        0xAD => (format!("LDMW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let data = cpu.read_u16(cpu.ix);
            cpu.aw = data;
            if cpu.dir() {
                cpu.ix = cpu.ix - 2;
            } else {
                cpu.ix = cpu.ix + 2;
            }
            if cpu.ix % 2 == 1 { 7 } else { 5 }
        })),

        0xAE => unimplemented!("CMPM"),
        0xAF => unimplemented!("CMPM"),

        0xB0 => (format!("MOV AL"), vec![op], Box::new(mov_al_i)),
        0xB1 => (format!("MOV CL"), vec![op], Box::new(mov_cl_i)),
        0xB2 => (format!("MOV DL"), vec![op], Box::new(mov_dl_i)),
        0xB3 => (format!("MOV BL"), vec![op], Box::new(mov_bl_i)),

        0xB4 => (format!("MOV AH"), vec![op], Box::new(mov_ah_i)),
        0xB5 => (format!("MOV CH"), vec![op], Box::new(mov_ch_i)),
        0xB6 => (format!("MOV DH"), vec![op], Box::new(mov_dh_i)),
        0xB7 => (format!("MOV BH"), vec![op], Box::new(mov_bh_i)),

        0xB8 => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("MOV AW, {word:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                cpu.set_aw(word);
                2
            }))
        },
        0xB9 => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("MOV CW, {word:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                cpu.set_cw(word);
                2
            }))
        },
        0xBA => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("MOV DW, {word:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                cpu.set_dw(word);
                2
            }))
        },
        0xBB => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("MOV BW, {word:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                cpu.set_bw(word);
                2
            }))
        },

        0xBC => (format!("MOV SP"), vec![op], Box::new(mov_sp_i)),
        0xBD => (format!("MOV BP"), vec![op], Box::new(mov_bp_i)),
        0xBE => (format!("MOV IX"), vec![op], Box::new(mov_ix_i)),
        0xBF => (format!("MOV IY"), vec![op], Box::new(mov_iy_i)),

        0xC0 => unimplemented!("SHIFT"),
        0xC1 => unimplemented!("SHIFT"),
        0xC2 => unimplemented!("RET"),
        0xC3 => unimplemented!("REF"),

        0xC4 => (format!("MOV DS1, AW"), vec![op], Box::new(move |cpu: &mut CPU| {
            cpu.ds1 = cpu.aw;
            if cpu.aw % 2 == 0 { 10 } else { 14 }
        })),

        0xC5 => (format!("MOV DS0, AW"), vec![op], Box::new(move |cpu: &mut CPU| {
            cpu.ds0 = cpu.aw;
            if cpu.aw % 2 == 0 { 10 } else { 14 }
        })),

        0xC6 => {
            let arg  = cpu.next_u8();
            (format!("MOV"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let mode = (arg & B_MODE) >> 6;
                let code = (arg & B_REG)  >> 3;
                if code != 0b000 {
                    panic!();
                }
                let mem  = (arg & B_MEM)  >> 0;
                let addr = cpu.memory_address(mode, mem);
                let imm  = cpu.next_u8();
                cpu.write_u8(addr, imm);
                3
            }))
        },

        0xC7 => unimplemented!("MOV mw imm"),
        0xC8 => unimplemented!("PREPARE"),
        0xC9 => unimplemented!("DISPOSE"),
        0xCA => unimplemented!("RET"),
        0xCB => unimplemented!("RET"),
        0xCC => unimplemented!("BRK"),
        0xCD => unimplemented!("BRK"),
        0xCE => unimplemented!("BRKV"),
        0xCF => unimplemented!("RETI"),

        0xD0 => unimplemented!("SHIFT b"),

        0xD1 => {
            let arg = cpu.next_u8();
            (format!("SHIFTW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let code = (arg & B_REG) >> 3;
                let source = get_source_word(cpu, arg);
                match code {
                    0b000 => {
                        unimplemented!("rol");
                    },
                    0b001 => {
                        unimplemented!("ror");
                    },
                    0b010 => {
                        let cy  = cpu.cy() as u16;
                        let msb = (source & W15) >> 15;
                        let nsb = (source & W14) >> 14;
                        let rotated = source << 1 | cy;
                        set_source_word(cpu, arg, rotated);
                        cpu.set_cy(msb > 0);
                        cpu.set_v(msb != nsb);
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
                        set_source_word(cpu, arg, shifted);
                        cpu.set_cy(lsb > 0);
                        cpu.set_v(msb_before != msb_after);
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
            }))
        },

        0xD2 => unimplemented!("SHIFT b, port"),
        0xD3 => unimplemented!("SHIFT w, port"),
        0xD4 => unimplemented!("CVTBD"),
        0xD5 => unimplemented!("CVTDB"),
        0xD6 => unimplemented!("UNDEF"),
        0xD7 => unimplemented!("TRANS"),
        0xD8 => unimplemented!("FPO1"),
        0xD9 => unimplemented!("FPO1"),
        0xDA => unimplemented!("FPO1"),
        0xDB => unimplemented!("FPO1"),
        0xDC => unimplemented!("FPO1"),
        0xDD => unimplemented!("FPO1"),
        0xDE => unimplemented!("FPO1"),
        0xDF => unimplemented!("FPO1"),

        0xE0 => unimplemented!("DBNZE"),

        0xE1 => unimplemented!("DBNZE"),

        0xE2 => {
            let arg = cpu.next_i8();
            (format!("DBNZ {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                cpu.cw = cpu.cw.overflowing_sub(1).0;
                if cpu.cw > 0 { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0xE3 => {
            let arg = cpu.next_i8();
            (format!("BCWZ {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                if cpu.cw() == 0 { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0xE4 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("IN {addr:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                let data = cpu.input_u8(addr);
                cpu.set_al(data);
                5
            }))
        },

        0xE5 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("INW"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                let data = cpu.input_u16(addr);
                cpu.set_aw(data);
                7
            }))
        },

        0xE6 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("OUT"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                let data = cpu.al();
                cpu.output_u8(addr, data);
                3
            }))
        },

        0xE7 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("OUTW"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                cpu.output_u16(addr, cpu.aw);
                5
            }))
        },

        0xE8 => {
            let displace = cpu.next_i16();
            let [lo, hi] = displace.to_le_bytes();
            (format!("CALLD"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                cpu.push_u16(cpu.pc);
                cpu.jump_i16(displace);
                if cpu.pc % 1 == 0 { 7 } else { 9 }
            }))
        },

        0xE9 => {
            let displace = cpu.next_i16();
            let [lo, hi] = displace.to_le_bytes();
            (format!("BR"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                cpu.jump_i16(displace);
                7
            }))
        },

        0xEA => {
            let offset     = cpu.next_u16();
            let [olo, ohi] = offset.to_le_bytes();
            let segment    = cpu.next_u16();
            let [slo, shi] = segment.to_le_bytes();
            (format!("BR {segment:04X}:{offset:04X}"), vec![op, olo, ohi, slo, shi], Box::new(move |cpu: &mut CPU| {
                cpu.set_pc(offset);
                cpu.set_ps(segment);
                7
            }))
        },

        0xEB => {
            let displace = cpu.next_i8();
            (format!("BR"), vec![op, displace as u8], Box::new(move |cpu: &mut CPU|{
                cpu.jump_i8(displace);
                7
            }))
        },

        0xEC => (format!("IN"), vec![op], Box::new(move |cpu: &mut CPU|{
            let addr = cpu.dw;
            let data = cpu.input_u8(addr);
            cpu.set_al(data);
            5
        })),

        0xED => (format!("INW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let addr = cpu.dw;
            let data = cpu.input_u16(addr);
            cpu.set_aw(data);
            7
        })),

        0xEE => (format!("OUT DW, AL"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.output_u8(cpu.dw, cpu.al());
            3
        })),

        0xEF => (format!("OUTW DW, AW"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.output_u16(cpu.dw, cpu.aw());
            5
        })),

        0xF0 => unimplemented!("BUSLOCK"),
        0xF1 => unimplemented!("UNDEFINED"),
        0xF2 => unimplemented!("REPNE"),

        0xF3 => (format!("REP"), vec![op], Box::new(move |cpu: &mut CPU|{
            if cpu.cw() == 0 {
                cpu.set_pc(cpu.pc() + 1);
            } else {
                let op = cpu.peek_u8();
                if (op == 0xA4) || (op == 0xA5) ||        // MOVBK
                   (op == 0xAC) || (op == 0xAD) ||        // LDM
                   (op == 0xAA) || (op == 0xAB) ||        // STM
                   (op == 0x6E) || (op == 0x6F) ||        // OUTM
                   (op == 0x6C) || (op == 0x6D)           // INM
                {
                    // repeat while cw != 0
                    cpu.opcode = op;
                    while cpu.cw() != 0 {
                        let (_, _, instruction) = v53_instruction(cpu, op);
                        let ticks = instruction(cpu);
                        cpu.clock += ticks;
                        cpu.set_cw(cpu.cw() - 1);
                    }
                } else if (op == 0xA6) || (op == 0xA7) || // CMPBK
                    (op == 0xAE) || (op == 0xAF)          // CMPM
                {
                    cpu.opcode = op;
                    unimplemented!("REPZ/REPE {:x}", op);
                    // repeat while cw != 0 && z == 0
                } else {
                    panic!("invalid instruction after REP")
                }
            }
            2
        })),

        0xF4 => unimplemented!("HALT"),
        0xF5 => unimplemented!("NOT1"),

        0xF6 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, _) => unimplemented!("test rm"),
                (0b001, _) => panic!("undefined group1 instruction"),
                (0b010, _) => unimplemented!("not rm"),
                (0b011, _) => unimplemented!("neg rm"),
                (0b100, _) => unimplemented!("mulu rm"),
                (0b101, _) => unimplemented!("mul rm"),
                (0b110, _) => unimplemented!("divu rm"),
                (0b111, 0b11) => (format!("DIV"), vec![op, arg], Box::new(move|cpu: &mut CPU|{
                    let t = cpu.aw() as i16;
                    let dst = cpu.register_value_u8((arg & B_REG) >> 3) as i16;
                    if (((t / dst) > 0) && ((t / dst) <= 0x7F)) ||
                       (((t / dst) < 0) && ((t / dst) > (0 - 0x7F - 1)))
                    {
                        cpu.set_ah((t % dst) as u8);
                        cpu.set_al((t / dst) as u8);
                    }
                    cpu.push_u16(cpu.psw());
                    cpu.set_ie(false);
                    cpu.set_brk(false);
                    //cpu.push_u16(cpu.ps());
                    //cpu.set_ps(u16::from_le_bytes([0x2, 0x3]));
                    //cpu.push_u16(cpu.pc());
                    //cpu.set_pc(u16::from_le_bytes([0x0, 0x1]));
                    17
                })),
                (0b111, _) => (format!("DIV"), vec![op, arg], Box::new(move|cpu: &mut CPU|{
                    let t    = cpu.aw() as i16;
                    let addr = cpu.memory_address(mode, mem);
                    let dst  = sign_extend_16(cpu.read_u8(addr) as u16, 8);
                    println!("\n\naddr={addr:x} dst={dst:b} t={t:b}\n");
                    cpu.dump();
                    if (((t / dst) > 0) && ((t / dst) <= 0x7F)) ||
                       (((t / dst) < 0) && ((t / dst) > (0 - 0x7F - 1)))
                    {
                        cpu.set_ah((t % dst) as u8);
                        cpu.set_al((t / dst) as u8);
                    }
                    cpu.push_u16(cpu.psw());
                    cpu.set_ie(false);
                    cpu.set_brk(false);
                    20
                })),
                _ => panic!("invalid group1 instruction {code:b}"),
            }
        },

        0xF7 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, _) => unimplemented!("testw rm"),
                (0b001, _) => panic!("undefined group1 instruction"),
                (0b010, _) => unimplemented!("notw rm"),
                (0b011, _) => unimplemented!("negw rm"),
                (0b100, _) => unimplemented!("muluw rm"),
                (0b101, _) => unimplemented!("mulw rm"),
                (0b110, _) => unimplemented!("divuw rm"),
                (0b111, _) => unimplemented!("divw rm"),
                _ => panic!("invalid group1 instruction {code:b}"),
            }
        },

        0xF8 => (format!("CLR1 CY"), vec![op], Box::new(clr1_cy)),
        0xF9 => (format!("SET1 CY"), vec![op], Box::new(set1_cy)),

        0xFA => (format!("DI"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_ie(false);
            2
        })),

        0xFB => (format!("EI"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_ie(true);
            2
        })),

        0xFC => (format!("CLR1 DIR"), vec![op], Box::new(clr1_dir)),
        0xFD => (format!("SET1 DIR"), vec![op], Box::new(set1_dir)),

        0xFE => unimplemented!("group2_b"),

        0xFF => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            (format!("GROUP2"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
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
                        let addr = cpu.memory_address(mode, mem) as i32;
                        let pc = cpu.read_u16(addr as u16 + 0);
                        let ps = cpu.read_u16(addr as u16 + 2);
                        cpu.set_sp(cpu.sp() - 2);
                        cpu.write_u16(cpu.sp(), cpu.ps());
                        cpu.set_ps(ps);
                        cpu.set_sp(cpu.sp() - 2);
                        cpu.write_u16(cpu.sp(), cpu.pc());
                        cpu.set_pc(pc);
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
            }))
        },
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
pub fn get_mode_reg_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let reg  = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, reg, mem]
}

#[inline]
pub fn get_mode_sreg_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let sreg = (arg & B_SREG) >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, sreg, mem]
}

#[inline]
pub fn get_mode_code_mem (cpu: &mut CPU) -> [u8;4] {
    let arg  = cpu.next_u8();
    let mode = (arg & B_MODE) >> 6;
    let code = (arg & B_REG)  >> 3;
    let mem  = (arg & B_MEM)  >> 0;
    [arg, mode, code, mem]
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

#[inline]
fn nop (state: &mut CPU) -> u64 {
    1
}

#[inline]
fn unimplemented (state: &mut CPU) -> u64 {
    unimplemented!("opcode {:x}", state.opcode())
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
