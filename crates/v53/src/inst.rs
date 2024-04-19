use crate::*;

pub fn v53_instruction (cpu: &mut CPU, op: u8) -> (
    String,
    Vec<u8>,
    Box<dyn Fn(&mut CPU)->u64>
) {
    match op {

        0x00 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg_dst = reg;
                let reg_src = mem;
                (
                    format!("ADD {}, {}", register_name_u8(reg_dst), register_name_u8(reg_src)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU| {
                        let src = cpu.register_value_u8(reg_src);
                        let dst = cpu.register_value_u8(reg_dst);
                        let (result, carry) = dst.overflowing_add(src);
                        let (_, overflow) = (dst as i8).overflowing_add(src as i8);
                        cpu.set_register_u8(reg_dst, result);
                        cpu.set_pzscyv(result as u16, carry, overflow);
                        2
                    })
                )
            } else {
                (
                    format!("ADD mem, {}", register_name_u8(reg)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU| {
                        let src  = cpu.register_value_u8(reg);
                        let addr = cpu.memory_address(mode, mem);
                        let dst  = cpu.read_u8(addr);
                        let (result, carry) = dst.overflowing_add(src);
                        let (_, overflow) = (dst as i8).overflowing_add(src as i8);
                        cpu.write_u8(addr, result);
                        cpu.set_pzscyv(result as u16, carry, overflow);
                        if addr % 2 == 0 { 7 } else { 11 }
                    })
                )
            }
        },

        0x01 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("ADDW mem, reg"), vec![op, arg], Box::new(move |cpu: &mut CPU| {
                let src  = cpu.register_value_u16(reg);
                let addr = cpu.memory_address(mode, mem);
                let dst  = cpu.read_u16(addr);
                let (result, carry) = dst.overflowing_add(src);
                let (_, overflow) = (dst as i16).overflowing_add(src as i16);
                cpu.write_u16(addr, result);
                cpu.set_pzscyv(result, carry, overflow);
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
                let (result, carry) = cpu.aw().overflowing_add(word);
                let (_, overflow) = (cpu.aw() as i16).overflowing_add(word as i16);
                cpu.set_aw(result);
                cpu.set_pzscyv(result, carry, overflow);
                2
            }))
        },

        0x06 => (format!("PUSH DS1"), vec![op], Box::new(push_ds1)),
        0x07 => (format!("POP DS1"), vec![op], Box::new(move |cpu: &mut CPU| {
            let value = cpu.pop_u16();
            cpu.set_ds1(value);
            if cpu.pc() % 2 == 1 { 7 } else { 5 }
        })),

        0x08 => unimplemented!("Byte bitwise OR to memory from register"),
        0x09 => unimplemented!("Word bitwise OR to memory from register"),

        0x0A => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                (format!("OR"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u8(mem);
                    let dst = cpu.register_value_u8(reg);
                    let result = dst | src;
                    cpu.set_register_u8(reg, result);
                    cpu.set_pzs(result as u16);
                    2
                }))
            } else {
                (format!("OR"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u8(addr);
                    let dst  = cpu.register_value_u8(reg);
                    let result = dst | src;
                    cpu.set_register_u8(reg, result);
                    cpu.set_pzs(result as u16);
                    6
                }))
            }
        },

        0x0B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                (format!("OR"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.register_reference_u16(reg);
                    let result = *dst | src;
                    *dst = result;
                    cpu.set_pzs(result);
                    2
                }))
            } else {
                (format!("OR"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u16(addr);
                    let dst  = cpu.register_reference_u16(reg);
                    let result = *dst | src;
                    *dst = result;
                    cpu.set_pzs(result);
                    if addr % 2 == 0 { 6 } else { 8 }
                }))
            }
        },

        0x0C => unimplemented!("Bitwise OR b ia"),
        0x0D => unimplemented!("Bitwise OR w ia"),

        0x0E => (format!("PUSH PS"), vec![op], Box::new(push_ps)),

        0x0F => {
            let arg = cpu.next_u8();
            match arg {

                0xE0 => (format!("BRKXA"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.next_u8() as u32;
                    //panic!("{addr} {:x?}", &cpu.memory[addr*4..addr*4+4]);
                    cpu.pc = u16::from_le_bytes([
                        cpu.get_byte(addr * 4 + 0),
                        cpu.get_byte(addr * 4 + 1),
                    ]);
                    cpu.ps = u16::from_le_bytes([
                        cpu.get_byte(addr * 4 + 2),
                        cpu.get_byte(addr * 4 + 3),
                    ]);
                    cpu.set_xa(true);
                    //println!("\n==========BRKXA {:x} {:x} {:x} {:x}", addr, cpu.pc, cpu.ps, cpu.program_address());
                    // TODO: set XA (internal I/O address: FF80H)
                    12
                })),

                0xF0 => (format!("RETXA"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.next_u8() as u32;
                    cpu.pc = u16::from_le_bytes([
                        cpu.get_byte(addr * 4 + 0),
                        cpu.get_byte(addr * 4 + 1),
                    ]);
                    cpu.ps = u16::from_le_bytes([
                        cpu.get_byte(addr * 4 + 2),
                        cpu.get_byte(addr * 4 + 3),
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
        0x17 => (format!("POP SS"),  vec![op], Box::new(move |cpu: &mut CPU| {
            let value = cpu.pop_u16();
            cpu.set_ss(value);
            if cpu.pc() % 2 == 1 { 7 } else { 5 }
        })),

        0x18 => unimplemented!("SUBC mem, reg"),
        0x19 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("SUBW"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let addr = cpu.memory_address(mode, mem);
                let dst  = cpu.read_u16(addr);
                let src  = cpu.register_value_u16(reg);
                let cy   = cpu.cy() as u16;
                let (result, carry) = dst.overflowing_sub(src + cy);
                let (_, overflow) = (dst as i16).overflowing_sub(src as i16 + cy as i16);
                cpu.write_u16(addr, result);
                cpu.set_pzscyv(result as u16, carry, overflow);
                if addr % 2 == 1 { 11 } else { 7 }
            }))
        },
        0x1A => unimplemented!("SUBC reg, mem"),
        0x1B => unimplemented!("SUBCW reg, mem"),
        0x1C => unimplemented!("SUBC acc, imm"),
        0x1D => unimplemented!("SUBCW acc, imm"),

        0x1E => (format!("PUSH DS0"), vec![op], Box::new(push_ds0)),
        0x1F => (format!("POP DS0"),  vec![op], Box::new(move |cpu: &mut CPU| {
            let value = cpu.pop_u16();
            cpu.set_ds0(value);
            if cpu.pc() % 2 == 1 { 7 } else { 5 }
        })),

        0x20 => unimplemented!("AND mem, reg"),
        0x21 => unimplemented!("ANDW mem, reg"),
        0x22 => unimplemented!("AND reg, mem"),
        0x23 => unimplemented!("ANDW reg, mem"),
        0x24 => {
            let byte = cpu.next_u8();
            (format!("AND AL, {byte}"), vec![op, byte], Box::new(move |cpu: &mut CPU|{
                let result = cpu.al() & byte;
                cpu.set_al(result);
                cpu.set_pzs(result as u16);
                2
            }))
        },
        0x25 => unimplemented!("ANDW acc, imm"),

        0x26 => (format!("DS1:"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.segment = Some(Segment::DS1);
            2
        })),

        0x27 => unimplemented!("ADJ4A"),

        0x28 => unimplemented!("SUB b f rm"),

        0x29 => unimplemented!("SUB w f rm"),

        0x2A => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg1 = register_name_u8(reg);
                let reg2 = register_name_u8(mem);
                (format!("SUB {reg1}, {reg2}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u8(mem);
                    let dst = cpu.register_value_u8(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_register_u8(reg, result);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    2
                }))
            } else {
                let name = register_name_u8(reg);
                (format!("SUB {name}, mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u8(addr);
                    let dst  = cpu.register_value_u8(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_register_u8(reg, result);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    if addr % 2 == 0 { 6 } else { 8 }
                }))
            }
        },

        0x2B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg1 = register_name_u16(reg);
                let reg2 = register_name_u16(mem);
                (format!("SUB {reg1}, {reg2}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.register_value_u16(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i16).overflowing_sub(src as i16);
                    cpu.set_register_u16(reg, result);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    2
                }))
            } else {
                let name = register_name_u16(reg);
                (format!("SUB {name}, mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u16(addr);
                    let dst  = cpu.register_value_u16(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i16).overflowing_sub(src as i16);
                    cpu.set_register_u16(reg, result);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    if addr % 2 == 0 { 6 } else { 8 }
                }))
            }
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
        0x32 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg_dst = reg;
                let reg_src = mem;
                (
                    format!("XOR {}, {}", register_name_u8(reg_dst), register_name_u8(reg_src)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU|{
                        let src = cpu.register_value_u8(mem);
                        let dst = cpu.register_value_u8(reg);
                        let result = dst ^ src;
                        cpu.set_register_u8(reg, result);
                        cpu.set_pzs(result as u16);
                        2
                    })
                )
            } else {
                (
                    format!("XOR {}, mem", register_name_u8(reg)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let src  = cpu.read_u8(addr);
                        let dst = cpu.register_value_u8(reg);
                        let result = dst ^ src;
                        cpu.set_register_u8(reg, result);
                        cpu.set_pzs(result as u16);
                        if addr % 2 == 0 { 6 } else { 8 }
                    })
                )
            }
        },

        0x33 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg_dst = reg;
                let reg_src = mem;
                (
                    format!("XOR {}, {}", register_name_u16(reg_dst), register_name_u16(reg_src)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU|{
                        let src = cpu.register_value_u16(mem);
                        let dst = cpu.register_reference_u16(reg);
                        let result = *dst ^ src;
                        *dst = result;
                        cpu.set_pzs(result);
                        2
                    })
                )
            } else {
                (
                    format!("XOR {}, mem", register_name_u16(reg)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let src  = cpu.read_u16(addr);
                        let dst  = cpu.register_reference_u16(reg);
                        let result = *dst ^ src;
                        *dst = result;
                        cpu.set_pzs(result);
                        if addr % 2 == 0 { 6 } else { 8 }
                    })
                )
            }
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
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    2
                } else {
                    let src  = cpu.register_value_u8(reg);
                    let addr = cpu.memory_address(mode, mem);
                    let dst  = cpu.read_u8(addr);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    if addr % 2 == 0 {
                        6
                    } else {
                        8
                    }
                }
            }))
        },

        0x39 => unimplemented!("Compare memory with word"),

        0x3A => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg1 = register_name_u8(reg);
                let reg2 = register_name_u8(mem);
                (format!("CMP {reg1}, {reg2}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u8(mem);
                    let dst = cpu.register_value_u8(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    2
                }))
            } else {
                let name = register_name_u8(reg);
                (format!("CMP mem, {name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u8(addr);
                    let dst  = cpu.register_value_u8(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    if addr % 2 == 0 {
                        6
                    } else {
                        8
                    }
                }))
            }
        },

        0x3B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg1 = register_name_u16(reg);
                let reg2 = register_name_u16(mem);
                (format!("CMPW {reg1}, {reg2}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.register_reference_u16(reg);
                    let (result, carry) = (*dst).overflowing_sub(src);
                    let (_, overflow) = (*dst as i16).overflowing_sub(src as i16);
                    cpu.set_pzscyv(result, carry, overflow);
                    2
                }))
            } else {
                let name = register_name_u16(reg);
                (format!("CMPW mem, {name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let src  = cpu.read_u16(addr);
                    let dst  = cpu.register_value_u16(reg);
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i16).overflowing_sub(src as i16);
                    cpu.set_pzscyv(result, carry, overflow);
                    if addr % 2 == 0 {
                        6
                    } else {
                        8
                    }
                }))
            }
        },

        0x3C => {
            let byte = cpu.next_u8();
            (format!("CMP w ia"), vec![op, byte], Box::new(move |cpu: &mut CPU|{
                let (result, carry) = cpu.al().overflowing_sub(byte);
                let (_, overflow) = (cpu.al() as i8).overflowing_sub(byte as i8);
                cpu.set_pzscyv(result as u16, carry, overflow);
                2
            }))
        },

        0x3D => {
            let word = cpu.next_u16();
            let [lo, hi] = word.to_le_bytes();
            (format!("CMP w ia"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                let (result, carry) = cpu.aw().overflowing_sub(word);
                let (_, overflow) = (cpu.aw() as i16).overflowing_sub(word as i16);
                cpu.set_pzscyv(result, carry, overflow);
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

        0x58 => (format!("POP AW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_aw(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),
        0x59 => (format!("POP CW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_cw(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),
        0x5A => (format!("POP DW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_dw(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),
        0x5B => (format!("POP BW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_bw(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),

        0x5C => (format!("POP SP"), vec![op], Box::new(move |cpu: &mut CPU|{
            // preserve old stack position to determine cycle count
            let addr = cpu.sp();
            let value = cpu.pop_u16();
            cpu.set_sp(value);
            if addr % 2 == 1 { 7 } else { 5 }
        })),
        0x5D => (format!("POP BP"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_bp(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),
        0x5E => (format!("POP IX"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_ix(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),
        0x5F => (format!("POP IY"), vec![op], Box::new(move |cpu: &mut CPU|{
            let value = cpu.pop_u16();
            cpu.set_iy(value);
            if cpu.sp() % 2 == 1 { 7 } else { 5 }
        })),

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
            let data = cpu.read_u8(cpu.ix() as u32);
            cpu.output_u8(cpu.dw(), data);
            if cpu.dir() {
                cpu.ix = cpu.ix - 1;
            } else {
                cpu.ix = cpu.ix + 1;
            }
            let rep = 1; // TODO
            8 * rep - 2
        })),

        0x6F => (format!("OUTMW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let data = cpu.read_u16(cpu.ix() as u32);
            cpu.output_u16(cpu.dw(), data);
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

        0x70 => {
            let arg = cpu.next_i8();
            (format!("BV {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU|{
                if cpu.v() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x71 => {
            let arg = cpu.next_i8();
            (format!("BNV {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU|{
                if !cpu.v() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x72 => {
            let arg = cpu.next_i8();
            (format!("BC {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU|{
                if cpu.cy() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x73 => {
            let arg = cpu.next_i8();
            (format!("BNC {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                if !cpu.cy() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x74 => {
            let arg = cpu.next_i8();
            (format!("BE {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                if cpu.z() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x75 => {
            let arg = cpu.next_i8();
            (format!("BNE {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                if !cpu.z() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x76 => {
            let arg = cpu.next_i8();
            (format!("BNH {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                if cpu.z() || cpu.cy() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x77 => {
            let arg = cpu.next_i8();
            (format!("BNH {arg}"), vec![op, arg as u8], Box::new(move |cpu: &mut CPU| {
                if !(cpu.z() || cpu.cy()) { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x78 => {
            let arg = cpu.next_i8();
            (format!("BN {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if cpu.s() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x79 => {
            let arg = cpu.next_i8();
            (format!("BP {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if !cpu.s() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x7A => {
            let arg = cpu.next_i8();
            (format!("BPE {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if cpu.p() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x7B => {
            let arg = cpu.next_i8();
            (format!("BPE {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if !cpu.p() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x7C => {
            let arg = cpu.next_i8();
            (format!("BLT {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if cpu.s() ^ cpu.z() { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x7D => {
            let arg = cpu.next_i8();
            (format!("BGE {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if !(cpu.s() ^ cpu.v()) { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x7E => {
            let arg = cpu.next_i8();
            (format!("BLT {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if cpu.z() || (cpu.s() ^ cpu.v()) { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x7F => {
            let arg = cpu.next_i8();
            (format!("BLT {arg}"), vec![op, arg as u8], Box::new(move|cpu: &mut CPU|{
                if !(cpu.z() || (cpu.s() ^ cpu.v())) { cpu.jump_i8(arg); 6 } else { 3 }
            }))
        },

        0x80 => { // TODO: ensure no sign extension
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, _) => unimplemented!("ADD"),
                (0b001, _) => unimplemented!("OR"),
                (0b010, _) => unimplemented!("ADDC"),
                (0b011, _) => unimplemented!("SUB"),
                (0b100, _) => unimplemented!("AND"),
                (0b101, _) => unimplemented!("SUB"),
                (0b110, _) => unimplemented!("XOR"),
                (0b111, 0b11) => {
                    let name = register_name_u8(mem);
                    let src  = cpu.next_u8();
                    (format!("CMP {name}, {src}"), vec![op, arg, src], Box::new(move|cpu: &mut CPU|{
                        let dst = cpu.register_value_u8(mem);
                        let (result, carry) = dst.overflowing_sub(src);
                        let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                        cpu.set_pzscyv(result as u16, carry, overflow);
                        2
                    }))
                },
                (0b111, _) => (format!("CMP"), vec![op, arg], Box::new(move|cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    let dst = cpu.read_u8(addr);
                    let src = cpu.next_u8();
                    let (result, carry) = dst.overflowing_sub(src);
                    let (_, overflow) = (dst as i8).overflowing_sub(src as i8);
                    cpu.set_pzscyv(result as u16, carry, overflow);
                    if addr % 2 == 0 { 6 } else { 8 }
                })),
                _ => unreachable!()
            }
        },

        0x81 => { // TODO: ensure no sign extension
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, 0b11) => {
                    let src = cpu.next_u16() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (
                        format!("ADDW {}, {src:04X}", register_name_u16(mem)),
                        vec![op, arg, lo, hi],
                        Box::new(move |cpu: &mut CPU|{
                            let dst = cpu.register_value_u16(mem) as i16;
                            let (result, carry) = (dst as u16).overflowing_add(src as u16);
                            let (_, overflow) = dst.overflowing_add(src);
                            cpu.set_register_u16(mem, result);
                            cpu.set_pzscyv(result, carry, overflow);
                            2
                        })
                    )
                },
                (0b001, _) => unimplemented!("ORW"),
                (0b010, _) => unimplemented!("ADDCW"),
                (0b011, _) => unimplemented!("SUBW"),
                (0b100, _) => unimplemented!("ANDW"),
                (0b101, _) => unimplemented!("SUBW"),
                (0b110, _) => unimplemented!("XORW"),
                (0b111, _) => unimplemented!("CMPW"),
                _ => unreachable!()
            }
        },

        0x82 => { // TODO: ensure sign extension
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, _) => unimplemented!("ADD"),
                (0b001, _) => unimplemented!("OR"),
                (0b010, _) => unimplemented!("ADDC"),
                (0b011, _) => unimplemented!("SUB"),
                (0b100, _) => unimplemented!("AND"),
                (0b101, _) => unimplemented!("SUB"),
                (0b110, _) => unimplemented!("XOR"),
                (0b111, _) => unimplemented!("CMP"),
                _ => unreachable!()
            }
        },

        0x83 => { // TODO: ensure sign extension
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, 0b11) => {
                    let src = cpu.next_u8() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (format!("ADDW {}, {src:04X}", register_name_u16(mem)), vec![op, arg, lo, hi], Box::new(move |cpu: &mut CPU|{
                        let dst = cpu.register_value_u16(mem) as i16;
                        let (result, carry) = (dst as u16).overflowing_add(src as u16);
                        let (_, overflow) = dst.overflowing_add(src);
                        cpu.set_register_u16(mem, result);
                        cpu.set_pzscyv(result, carry, overflow);
                        2
                    }))
                },
                (0b000, _)    => {
                    let src = cpu.next_u8() as i16;
                    let [lo, hi] = src.to_le_bytes();
                    (format!("ADDW {}, mem", register_name_u16(mem)), vec![op, arg, lo, hi], Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let dst = cpu.read_u16(addr);
                        let (result, carry) = (dst as u16).overflowing_add(src as u16);
                        let (_, overflow) = dst.overflowing_add(src as u16);
                        cpu.set_register_u16(mem, result);
                        cpu.set_pzscyv(result, carry, overflow);
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
                (0b111, 0b11) => {
                    let src = cpu.next_u8();
                    (format!("CMPW {}, {src:04X}", register_name_u16(mem)), vec![op, arg, src],
                        Box::new(move |cpu: &mut CPU|{
                            let dst = cpu.register_value_u16(mem) as i16;
                            let (result, carry) = (dst as u16).overflowing_sub(src as u16);
                            let (_, overflow) = dst.overflowing_sub(src as i16);
                            cpu.set_pzscyv(result, carry, overflow);
                            2
                        }))
                },
                (0b111, _) => {
                    let src = cpu.next_u8();
                    (format!("CMPW {}, mem", register_name_u16(mem)), vec![op, arg, src],
                        Box::new(move |cpu: &mut CPU|{
                            let addr = cpu.memory_address(mode, mem);
                            let dst = cpu.read_u16(addr);
                            let (result, carry) = (dst as u16).overflowing_sub(src as u16);
                            let (_, overflow) = dst.overflowing_sub(src as u16);
                            cpu.set_pzscyv(result, carry, overflow);
                            if addr % 2 == 0 { 6 } else { 8 }
                        }))
                },
                _ => unreachable!()
            }
        },

        0x84 => unimplemented!("TEST"),

        0x85 => unimplemented!("TEST"),

        0x86 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("XCH"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let register_value = cpu.register_value_u8(reg);
                let addr           = cpu.memory_address(mode, mem);
                let memory_value   = cpu.read_u8(addr);
                cpu.set_register_u8(reg, memory_value);
                cpu.write_u8(addr, register_value);
                if addr % 2 == 1 { 12 } else { 8 }
            }))
        }

        0x87 => unimplemented!("XCH"),

        0x88 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            let name = register_name_u8(reg);
            (format!("MOV mem, {name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let addr = cpu.memory_address(mode, mem);
                let val = cpu.register_value_u8(reg);
                cpu.write_u8(addr, val);
                if addr % 2 == 0 { 3 } else { 5 }
            }))
        },

        0x89 => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            let name = register_name_u16(reg);
            (format!("MOVW mem, {name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let addr = cpu.memory_address(mode, mem);
                let val = cpu.register_value_u16(reg);
                cpu.write_u16(addr, val);
                if addr % 2 == 0 { 3 } else { 5 }
            }))
        },

        0x8A => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg1 = register_name_u8(reg);
                let reg2 = register_name_u8(mem);
                (format!("MOV {reg1}, {reg2}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    cpu.set_register_u8(reg, cpu.register_value_u8(mem));
                    2
                }))
            } else {
                let name = register_name_u16(reg);
                (format!("MOV {name}, mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let address = cpu.memory_address(mode, mem);
                    let value = cpu.read_u8(address);
                    cpu.set_register_u8(reg, value);
                    5
                }))
            }
        },

        0x8B => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            if mode == 0b11 {
                let reg1 = register_name_u16(reg);
                let reg2 = register_name_u16(mem);
                (format!("MOVW {reg1}, {reg2}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    cpu.set_register_u16(reg, cpu.register_value_u16(mem));
                    2
                }))
            } else {
                let name = register_name_u16(reg);
                (format!("MOVW {name}, mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let address = cpu.memory_address(mode, mem);
                    let value = cpu.read_u16(address);
                    cpu.set_register_u16(reg, value);
                    if address % 2 == 1 { 7 } else { 5 }
                }))
            }
        },

        0x8C => {
            let [arg, mode, sreg, mem] = get_mode_sreg_mem(cpu);
            let name = segment_register_name(sreg);
            (format!("MOV mem, {name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
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

        0x8D => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            let dst = register_name_u16(reg);
            let (src, mut bytes, compute_address) = cpu.parse_effective_address(mode, mem);
            bytes.insert(0, arg);
            bytes.insert(0, op);
            (format!("LDEA {dst}, {src}"), bytes, Box::new(move |cpu: &mut CPU|{
                let address = compute_address(cpu);
                cpu.set_register_u16(reg, address);
                2
            }))
        },

        0x8E => {
            let [arg, mode, sreg, mem] = get_mode_sreg_mem(cpu);
            let name = segment_register_name(sreg);
            if mode == 0b11 {
                let src = register_name_u16(mem);
                (format!("MOVW {name}, {src}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let src = cpu.register_value_u16(mem);
                    let dst = cpu.segment_register_reference(sreg);
                    *dst = src;
                    2
                }))
            } else {
                let (src, mut bytes, compute_address) = cpu.parse_effective_address(mode, mem);
                bytes.insert(0, arg);
                bytes.insert(0, op);
                unimplemented!();
            }
        },

        0x8F => {
            let [arg, mode, reg, mem] = get_mode_reg_mem(cpu);
            (format!("POP"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let addr = cpu.memory_address(mode, mem);
                let data = cpu.pop_u16();
                cpu.write_u16(addr, data);
                if addr % 2 == 1 { 9 } else { 5 }
            }))
        },

        0x90 => (format!("NOP"), vec![op], Box::new(move |cpu: &mut CPU|{
            let cw = cpu.cw();
            let aw = cpu.aw();
            cpu.set_cw(aw);
            cpu.set_aw(cw);
            1
        })),

        0x91 => (format!("XCH DW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let cw = cpu.cw();
            let aw = cpu.aw();
            cpu.set_cw(aw);
            cpu.set_aw(cw);
            3
        })),

        0x92 => (format!("XCH DW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let dw = cpu.dw();
            let aw = cpu.aw();
            cpu.set_dw(aw);
            cpu.set_aw(dw);
            3
        })),

        0x93 => (format!("XCH BW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let bw = cpu.bw();
            let aw = cpu.aw();
            cpu.set_bw(aw);
            cpu.set_aw(bw);
            3
        })),

        0x94 => (format!("XCH SP"), vec![op], Box::new(move |cpu: &mut CPU|{
            let sp = cpu.sp();
            let aw = cpu.aw();
            cpu.set_sp(aw);
            cpu.set_aw(sp);
            3
        })),

        0x95 => (format!("XCH BP"), vec![op], Box::new(move |cpu: &mut CPU|{
            let bp = cpu.bp();
            let aw = cpu.aw();
            cpu.set_bp(aw);
            cpu.set_aw(bp);
            3
        })),

        0x96 => (format!("XCH IX"), vec![op], Box::new(move |cpu: &mut CPU|{
            let ix = cpu.ix();
            let aw = cpu.aw();
            cpu.set_ix(aw);
            cpu.set_aw(ix);
            3
        })),

        0x97 => (format!("XCH IY"), vec![op], Box::new(move |cpu: &mut CPU|{
            let iy = cpu.iy();
            let aw = cpu.aw();
            cpu.set_iy(aw);
            cpu.set_aw(iy);
            3
        })),

        0x98 => (format!("CVTBW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let msb = cpu.al() & B7;
            if msb > 0 {
                cpu.set_ah(0b11111111)
            } else {
                cpu.set_ah(0b00000000)
            }
            2
        })),

        0x99 => unimplemented!("CVTBL"),
        0x9A => unimplemented!("CALL"),
        0x9B => (format!("POLL"), vec![op], Box::new(move |cpu: &mut CPU|{
            // TODO: coprocessor
            2
        })),

        0x9C => (format!("PUSH PSW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let data = cpu.psw();
            cpu.push_u16(data);
            if cpu.sp() % 2 == 0 { 5 } else { 9 }
        })),

        0x9D => (format!("POP PSW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let value = cpu.pop_u16();
            cpu.set_psw(value);
            if cpu.pc() % 2 == 1 { 7 } else { 5 }
        })),

        0x9E => (format!("MOV PSW, AH"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_psw(((cpu.ah() & 0b11010111) | 0b00000010) as u16);
            2
        })),

        0x9F => (format!("MOV AH, PSW"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_ah(((cpu.psw() & 0b11010111) | 0b00000010) as u8);
            2
        })),

        0xA0 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("MOV AL, {addr:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                let value = cpu.read_u8(addr as u32);
                cpu.set_al(value);
                5
            }))
        },

        0xA1 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("MOV AL, {addr:04X}"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                let value = cpu.read_u16(addr as u32);
                cpu.set_aw(value);
                if addr % 2 == 1 { 7 } else { 5 }
            }))
        },

        0xA2 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("MOV {addr:04X}, AL"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                cpu.write_u8(addr as u32, cpu.al());
                3
            }))
        },

        0xA3 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("MOV {addr:04X}, AW"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU|{
                cpu.write_u16(addr as u32, cpu.aw());
                if addr % 2 == 1 { 5 } else { 3 }
            }))
        },

        0xA4 => unimplemented!("MOVBK b"),

        0xA5 => (format!("MOVBKW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let dst = cpu.ds1() as u32 * 0x10 + cpu.iy() as u32;
            let src = cpu.effective_address(cpu.ix() as u32);
            println!("{:04X} {src:04X}", cpu.cw());
            cpu.set_byte(dst + 0, cpu.get_byte(src + 0));
            cpu.set_byte(dst + 1, cpu.get_byte(src + 1));
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
            let segment = cpu.segment;
            cpu.segment = Some(Segment::DS1);
            cpu.write_u8(iy as u32, cpu.al());
            cpu.segment = segment;
            cpu.set_iy(if cpu.dir() {
                iy.overflowing_sub(1).0
            } else {
                iy.overflowing_add(1).0
            });
            if iy % 2 == 0 { 3 } else { 5 }
        })),

        0xAB => (format!("STMW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let iy = cpu.iy();
            let segment = cpu.segment;
            cpu.segment = Some(Segment::DS1);
            cpu.write_u16(iy as u32, cpu.aw());
            cpu.segment = segment;
            cpu.set_iy(if cpu.dir() {
                iy.overflowing_sub(2).0
            } else {
                iy.overflowing_add(2).0
            });
            if iy % 2 == 0 { 3 } else { 5 }
        })),

        0xAC => (format!("LDM"), vec![op], Box::new(move |cpu: &mut CPU| {
            let data = cpu.read_u8(cpu.ix() as u32);
            cpu.set_al(data);
            if cpu.dir() {
                cpu.ix = cpu.ix - 1;
            } else {
                cpu.ix = cpu.ix + 1;
            }
            5
        })),

        0xAD => (format!("LDMW"), vec![op], Box::new(move |cpu: &mut CPU| {
            let data = cpu.read_u16(cpu.ix() as u32);
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

        0xBC => {
            let arg = cpu.next_u16();
            let [lo, hi] = arg.to_le_bytes();
            (format!("MOV SP, {arg:04X}"), vec![op, lo, hi], Box::new(move|cpu: &mut CPU|{
                cpu.set_sp(arg);
                2
            }))
        },
        0xBD => {
            let arg = cpu.next_u16();
            let [lo, hi] = arg.to_le_bytes();
            (format!("MOV BP, {arg:04X}"), vec![op, lo, hi], Box::new(move|cpu: &mut CPU|{
                cpu.set_bp(arg);
                2
            }))
        },
        0xBE => {
            let arg = cpu.next_u16();
            let [lo, hi] = arg.to_le_bytes();
            (format!("MOV IX, {arg:04X}"), vec![op, lo, hi], Box::new(move|cpu: &mut CPU|{
                cpu.set_ix(arg);
                2
            }))
        },
        0xBF => {
            let arg = cpu.next_u16();
            let [lo, hi] = arg.to_le_bytes();
            (format!("MOV IY, {arg:04X}"), vec![op, lo, hi], Box::new(move|cpu: &mut CPU|{
                cpu.set_iy(arg);
                2
            }))
        },

        0xC0 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, _) => {
                    unimplemented!("rol");
                },
                (0b001, _) => {
                    unimplemented!("ror");
                },
                (0b010, _) => {
                    unimplemented!("rolc");
                },
                (0b011, _) => {
                    unimplemented!("rorc");
                },
                (0b100, 0b11) => {
                    let imm = cpu.next_u8();
                    let name = register_name_u8(mem);
                    (
                        format!("SHL {name}, {imm:02X}"),
                        vec![op, arg, imm],
                        Box::new(move |cpu: &mut CPU|{
                            let source    = cpu.register_value_u8(mem);
                            let msb       = source & B7;
                            let shifted   = source << 1;
                            let msb_after = shifted >> 7;
                            cpu.set_register_u8(mem, shifted);
                            cpu.set_cy(msb > 0);
                            cpu.set_v(msb != msb_after);
                            2 + imm as u64
                        })
                    )
                },
                (0b100, _) => {
                    unimplemented!("shl mem");
                },
                (0b101, _) => {
                    unimplemented!("shr");
                },
                (0b110, _) => {
                    panic!("invalid shift code 0b110");
                },
                (0b111, _) => {
                    unimplemented!("shra");
                },
                _ => {
                    unreachable!("shift code {code:b}");
                }
            }
        },

        0xC1 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match code {
                0b000 => {
                    unimplemented!("rol");
                },
                0b001 => {
                    unimplemented!("ror");
                },
                0b010 => {
                    unimplemented!("rolc");
                },
                0b011 => {
                    unimplemented!("rorc");
                },
                0b100 => {
                    unimplemented!("shl");
                },
                0b101 => {
                    unimplemented!("shr");
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

        0xC2 => {
            let pop = cpu.next_u16();
            let [lo, hi] = pop.to_le_bytes();
            unimplemented!("RET {pop}")
        },

        0xC3 => (format!("RET"), vec![op], Box::new(move |cpu: &mut CPU| {
            let pc = cpu.pop_u16();
            cpu.set_pc(pc);
            if pc % 2 == 1 { 12 } else { 10 }
        })),

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

        0xCD => {
            let arg = cpu.next_u8();
            (format!("BRK {arg:02X}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                let ta = cpu.read_u16(arg as u32 * 4);
                let tc = cpu.read_u16(arg as u32 * 4 + 2);
                cpu.push_u16(cpu.psw());
                cpu.set_ie(false);
                cpu.set_brk(false);
                cpu.push_u16(cpu.ps());
                cpu.set_ps(tc);
                cpu.push_u16(cpu.pc());
                cpu.set_pc(ta);
                if cpu.pc() % 2 == 1 { 24 } else { 18 }
            }))
        },

        0xCE => unimplemented!("BRKV"),

        0xCF => (format!("RETI"), vec![op], Box::new(move |cpu: &mut CPU|{
            let pc = cpu.pop_u16();
            let ps = cpu.pop_u16();
            let psw = cpu.pop_u16();
            cpu.set_pc(pc);
            cpu.set_ps(ps);
            cpu.set_psw(psw);
            if cpu.pc() % 2 == 1 { 19 } else { 13 }
        })),

        0xD0 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match code {
                0b000 => {
                    unimplemented!("rol");
                },
                0b001 => {
                    unimplemented!("ror");
                },
                0b010 => {
                    unimplemented!("rolc");
                },
                0b011 => {
                    unimplemented!("rorc");
                },
                0b100 => {
                    unimplemented!("shl");
                },
                0b101 => {
                    unimplemented!("shr");
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

        0xD1 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, _) => unimplemented!("rol"),
                (0b001, _) => unimplemented!("ror"),
                (0b010, _) => {
                    (format!("ROLC"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                        let source = get_source_word(cpu, arg);
                        let cy  = cpu.cy() as u16;
                        let msb = (source & W15) >> 15;
                        let nsb = (source & W14) >> 14;
                        let rotated = source << 1 | cy;
                        set_source_word(cpu, arg, rotated);
                        cpu.set_cy(msb > 0);
                        cpu.set_v(msb != nsb);
                        2
                    }))
                },
                (0b011, _) => unimplemented!("rorc"),
                (0b100, _) => {
                    (format!("SHL"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                        let source = get_source_word(cpu, arg);
                        let msb       = source & W15;
                        let shifted   = source << 1;
                        let msb_after = shifted >> 15;
                        set_source_word(cpu, arg, shifted);
                        cpu.set_cy(msb > 0);
                        cpu.set_v(msb != msb_after);
                        2
                    }))
                },
                (0b101, _) => {
                    (format!("SHR"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                        let source = get_source_word(cpu, arg);
                        let msb       = source & W15;
                        let shifted   = source >> 1;
                        let msb_after = shifted >> 15;
                        set_source_word(cpu, arg, shifted);
                        cpu.set_cy(msb > 0);
                        cpu.set_v(msb != msb_after);
                        2
                    }))
                },
                (0b110, _) => panic!("invalid shift code 0b110"),
                (0b111, _) => unimplemented!("shra"),
                _ => unreachable!("shift code {code:b}"),
            }
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
            let addr = cpu.next_u8();
            (format!("IN {addr:04X}"), vec![op, addr], Box::new(move |cpu: &mut CPU| {
                let data = cpu.input_u8(addr as u32);
                cpu.set_al(data);
                5
            }))
        },

        0xE5 => {
            let addr = cpu.next_u16();
            let [lo, hi] = addr.to_le_bytes();
            (format!("INW"), vec![op, lo, hi], Box::new(move |cpu: &mut CPU| {
                let data = cpu.input_u16(addr as u32);
                cpu.set_aw(data);
                7
            }))
        },

        0xE6 => {
            let addr = cpu.next_u8();
            (format!("OUT"), vec![op, addr], Box::new(move |cpu: &mut CPU| {
                let data = cpu.al();
                cpu.output_u8(addr as u16, data);
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
            (format!("BR {segment:04X}:{offset:04X}"), vec![op, olo, ohi, slo, shi],
                Box::new(move |cpu: &mut CPU| {
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
            let data = cpu.input_u8(addr as u32);
            cpu.set_al(data);
            5
        })),

        0xED => (format!("INW"), vec![op], Box::new(move |cpu: &mut CPU|{
            let addr = cpu.dw;
            let data = cpu.input_u16(addr as u32);
            cpu.set_aw(data);
            7
        })),

        0xEE => (format!("OUT DW, AL"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.output_u8(cpu.dw(), cpu.al());
            3
        })),

        0xEF => (format!("OUTW DW, AW"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.output_u16(cpu.dw(), cpu.aw());
            5
        })),

        0xF0 => unimplemented!("BUSLOCK"),
        0xF1 => unimplemented!("UNDEFINED"),
        0xF2 => unimplemented!("REPNE"),

        0xF3 => {
            let arg = cpu.next_u8();
            let name = match arg {
                0xA4 => "MOVBK",
                0xA5 => "MOVBKW",
                0xAC => "LDM",
                0xAD => "LDMW",
                0xAA => "STM",
                0xAB => "STMW",
                0x6E => "OUTM",
                0x6F => "OUTMW",
                0x6C => "INM",
                0x6D => "INMW",
                0xA6 => "CMPBK",
                0xA7 => "CMPBKW",
                0xAE => "CMPM",
                0xAF => "CMPMW",
                _ => panic!("invalid instruction after REP")
            };
            (format!("REP {name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                if cpu.cw() == 0 {
                    cpu.set_pc(cpu.pc() + 1);
                } else {
                    let op = arg;
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
            }))
        },

        0xF4 => unimplemented!("HALT"),
        0xF5 => unimplemented!("NOT1"),

        0xF6 => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, 0b11) => {
                    let imm = cpu.next_u8();
                    let name = register_name_u8(mem);
                    (
                        format!("TEST {name}, {imm:02X}"),
                        vec![op, arg, imm],
                        Box::new(move |cpu: &mut CPU|{
                            unimplemented!()
                        })
                    )
                },
                (0b000, _) => {
                    let disp = cpu.next_u16();
                    let [lo, hi] = disp.to_le_bytes();
                    let imm = cpu.next_u8();
                    (
                        format!("TEST mem, {imm:02X}"),
                        vec![op, arg, imm],
                        Box::new(move |cpu: &mut CPU|{
                            let address = match mem {
                                0b110 => disp,
                                _ => unimplemented!("mode={mode:b} mem={mem:b}"),
                            } as u32;
                            let data = cpu.read_u8(address);
                            let result = data & imm;
                            cpu.set_pzs(result as u16);
                            cpu.set_cy(false);
                            cpu.set_v(false);
                            2
                        })
                    )
                },
                (0b001, _) => panic!("undefined group1 instruction"),
                (0b010, _) => unimplemented!("not rm"),
                (0b011, _) => unimplemented!("neg rm"),
                (0b100, _) => unimplemented!("mulu rm"),
                (0b101, _) => unimplemented!("mul rm"),
                (0b110, _) => {
                    unimplemented!("divu rm")
/*
#define DIVUB                                               \
	uresult = Wreg(AW);                                 \
	uresult2 = uresult % tmp;                               \
	if ((uresult /= tmp) > 0xff) {                          \
		nec_interrupt(NEC_DIVIDE_VECTOR, BRK); break;                            \
	} else {                                                \
		Breg(AL) = uresult;                             \
		Breg(AH) = uresult2;                            \
	}
*/

                },
                (0b111, 0b11) => (format!("DIV"), vec![op, arg], Box::new(move|cpu: &mut CPU|{
/*
#define DIVB                                                \
	result = (int16_t)Wreg(AW);                           \
	result2 = result % (int16_t)((int8_t)tmp);                  \
	if ((result /= (int16_t)((int8_t)tmp)) > 0xff) {            \
		nec_interrupt(NEC_DIVIDE_VECTOR, BRK); break;                            \
	} else {                                                \
		Breg(AL) = result;                              \
		Breg(AH) = result2;                             \
	}
*/
                    let t = cpu.aw() as i16;
                    let dst = cpu.register_value_u8((arg & B_REG) >> 3) as i16;
                    if (((t / dst) > 0) && ((t / dst) <= 0x7F)) ||
                       (((t / dst) < 0) && ((t / dst) > (0 - 0x7F - 1)))
                    {
                        cpu.set_ah((t % dst) as u8);
                        cpu.set_al((t / dst) as u8);
                    }
                    //cpu.push_u16(cpu.psw());
                    //cpu.set_ie(false);
                    //cpu.set_brk(false);
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
                    //cpu.push_u16(cpu.psw());
                    //cpu.set_ie(false);
                    //cpu.set_brk(false);
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
                (0b110, _) => {
/*
#define DIVUW                                               \
	uresult = (((uint32_t)Wreg(DW)) << 16) | Wreg(AW);\
	uresult2 = uresult % tmp;                               \
	if ((uresult /= tmp) > 0xffff) {                        \
		nec_interrupt(NEC_DIVIDE_VECTOR, BRK); break;                            \
	} else {                                                \
		Wreg(AW)=uresult;                               \
		Wreg(DW)=uresult2;                              \
	}
*/
/*
#define DIVW                                                \
	result = ((uint32_t)Wreg(DW) << 16) + Wreg(AW);   \
	result2 = result % (int32_t)((int16_t)tmp);                 \
	if ((result /= (int32_t)((int16_t)tmp)) > 0xffff) {         \
		nec_interrupt(NEC_DIVIDE_VECTOR, BRK); break;                            \
	} else {                                                \
		Wreg(AW)=result;                                \
		Wreg(DW)=result2;                               \
	}
*/
                    unimplemented!("divuw rm")
                },

                (0b111, 0b11) => (
                    format!("DIVW {}", register_name_u16(mem)),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU|{
                        let divisor   = cpu.register_value_u16(mem) as u32;
                        let dividend  = ((cpu.dw() as u32) << 16) + cpu.aw() as u32;
                        let remainder = dividend % divisor;
                        let result    = dividend / divisor; 
                        if result > 0xffff {
                            panic!("divide error")
                        } else {
                            cpu.set_aw(result.try_into().expect("u32 didn't fit in u16"));
                            cpu.set_dw(remainder.try_into().expect("u32 didn't fit in u16"));
                        }
                        24
                    })
                ),

                (0b111, _) => (
                    format!("DIVW mem"),
                    vec![op, arg],
                    Box::new(move |cpu: &mut CPU|{
                        let address  = cpu.memory_address(mode, mem);
                        let divisor  = cpu.read_u16(address) as u32;
                        let dividend  = ((cpu.dw() as u32) << 16) + cpu.aw() as u32;
                        let remainder = dividend % divisor;
                        let result    = dividend / divisor; 
                        if result > 0xffff {
                            panic!("divide error")
                        } else {
                            cpu.set_aw(result.try_into().expect("u32 didn't fit in u16"));
                            cpu.set_dw(remainder.try_into().expect("u32 didn't fit in u16"));
                        }
                        24
                    })
                ),

                _ => panic!("invalid group1 instruction {code:b}"),
            }
        },

        0xF8 => (format!("CLR1 CY"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_cy(false);
            2
        })),

        0xF9 => (format!("SET1 CY"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_cy(true);
            2
        })),

        0xFA => (format!("DI"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_ie(false);
            2
        })),

        0xFB => (format!("EI"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_ie(true);
            2
        })),

        0xFC => (format!("CLR1 DIR"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_dir(false);
            2
        })),

        0xFD => (format!("SET1 DIR"), vec![op], Box::new(move |cpu: &mut CPU|{
            cpu.set_dir(true);
            2
        })),

        0xFE => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match (code, mode) {
                (0b000, 0b11) => {
                    let reg_name = register_name_u8(mem);
                    (format!("INC {reg_name}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                        let value = cpu.register_value_u8(mem);
                        let (result, carry) = value.overflowing_add(1);
                        let (_, overflow) = (value as i8).overflowing_add(1);
                        //println!("\n\n==================={value} -> {result} {carry} {overflow}\n\n");
                        cpu.set_register_u8(mem, result);
                        cpu.set_pzscyv(result as u16, carry, overflow);
                        2
                    }))
                },
                (0b000, _) => {
                    (format!("INC mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let value = cpu.read_u8(addr);
                        let (result, carry) = value.overflowing_add(1);
                        let (_, overflow) = (value as i8).overflowing_add(1);
                        cpu.write_u8(addr, result);
                        cpu.set_pzscyv(result as u16, carry, overflow);
                        if addr % 2 == 1 { 11 } else { 7 }
                    }))
                },
                (0b001, _) => {
                    unimplemented!("dec");
                },
                (0b010, _) => {
                    unimplemented!()
                },
                (0b011, _) => {
                    unimplemented!()
                },
                (0b100, _) => {
                    unimplemented!("br");
                },
                (0b101, _) => {
                    unimplemented!("br");
                },
                (0b110, _) => {
                    unimplemented!("push");
                },
                (0b111, _) => {
                    panic!("undefined instruction 0b111")
                },
                _ => {
                    unreachable!("imm code {code:b}");
                }
            }
        },

        0xFF => {
            let [arg, mode, code, mem] = get_mode_code_mem(cpu);
            match code {
                0b000 => {
                    (format!("INCW mem"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                        let addr = cpu.memory_address(mode, mem);
                        let value = cpu.read_u16(addr);
                        let (result, carry) = value.overflowing_add(1);
                        let (_, overflow) = (value as i16).overflowing_add(1);
                        cpu.write_u16(addr, result);
                        cpu.set_pzscyv(result as u16, carry, overflow);
                        if addr % 2 == 1 { 11 } else { 7 }
                    }))
                },
                0b001 => {
                    unimplemented!("dec");
                },
                0b010 => (format!("CALL16 {arg}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);
                    cpu.push_u16(cpu.pc());
                    let pc = cpu.read_u16(addr);
                    cpu.set_pc(pc);
                    if addr % 2 == 0 { 15 } else { 23 }
                })),
                0b011 => (format!("CALL32 {arg}"), vec![op, arg], Box::new(move |cpu: &mut CPU|{
                    let addr = cpu.memory_address(mode, mem);

                    cpu.push_u16(cpu.ps());
                    let ps = cpu.read_u16(addr + 2);
                    cpu.set_ps(ps);

                    cpu.push_u16(cpu.pc());
                    let pc = cpu.read_u16(addr + 0);
                    cpu.set_pc(pc);

                    if addr % 2 == 0 { 15 } else { 23 }
                })),
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
        },

    }
}
