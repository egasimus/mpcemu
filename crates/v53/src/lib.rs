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

mpcemu_core::define_instruction_set! {
    [0x00, "ADD",      "Add byte to memory from register",        add_b_f_rm],
    [0x01, "ADD",      "Add word to memory from register",        add_w_f_rm],
    [0x02, "ADD",      "Add byte to register from memory",        add_b_t_rm],
    [0x03, "ADD",      "Add word to register from memory",        add_w_t_rm],
    [0x04, "ADD",      "Add byte to accumulator from constant",   add_b_ia],
    [0x05, "ADD",      "Add word to accumulator from constant",   add_w_ia],
    [0x06, "PUSH DS1", "Push value of DS1 register to stack",     push_ds1],
    [0x07, "POP DS1",  "Pop value of DS1 register from stack",    pop_ds1],
    [0x08, "OR",       "Byte bitwise OR to memory from register", unimplemented],
    [0x09, "OR",       "Word bitwise OR to memory from register", unimplemented],
    [0x0A, "OR",       "Byte bitwise OR to register from memory", unimplemented],
    [0x0B, "OR",       "Word bitwise OR to register from memory", or_w_t_rm],
    [0x0C, "OR",       "Bitwise OR b ia",                         unimplemented],
    [0x0D, "OR",       "Bitwise OR w ia",                         unimplemented],
    [0x0E, "PUSH PS",  "Bitwise OR",                              unimplemented],
    [0x0F, "GROUP3",   "See Group 3",                             group3_instruction],

    [0x10, "ADDC",     "",                                     unimplemented],
    [0x11, "ADDC",     "",                                     unimplemented],
    [0x12, "ADDC",     "",                                     unimplemented],
    [0x13, "ADDC",     "",                                     unimplemented],
    [0x14, "ADDC",     "",                                     unimplemented],
    [0x15, "ADDC",     "",                                     unimplemented],
    [0x16, "PUSH SS",  "Push value of SS register to stack",   push_ss],
    [0x17, "POP SS",   "Pop value of SS register from stack",  pop_ss],
    [0x18, "SUBC",     "",                                     unimplemented],
    [0x19, "SUBC",     "",                                     unimplemented],
    [0x1A, "SUBC",     "",                                     unimplemented],
    [0x1B, "SUBC",     "",                                     unimplemented],
    [0x1C, "SUBC",     "",                                     unimplemented],
    [0x1D, "SUBC",     "",                                     unimplemented],
    [0x1E, "PUSH DS0", "Push value of DS0 register to stack",  push_ds0],
    [0x1F, "POP DS0",  "Pop value of DS0 register from stack", pop_ds0],

    [0x20, "AND",   "",                                        unimplemented],
    [0x21, "AND",   "",                                        unimplemented],
    [0x22, "AND",   "",                                        unimplemented],
    [0x23, "AND",   "",                                        unimplemented],
    [0x24, "AND",   "",                                        unimplemented],
    [0x25, "AND",   "",                                        unimplemented],
    [0x26, "DS1:",  "Set segment override to data segment 1",  ds1],
    [0x27, "ADJ4A", "",                                        unimplemented],
    [0x28, "SUB",   "b f rm",                                  unimplemented],
    [0x29, "SUB",   "w f rm",                                  unimplemented],
    [0x2A, "SUB",   "Subtract byte into memory",               sub_b_t_rm],
    [0x2B, "SUB",   "Subtract word into memory",               sub_w_t_rm],
    [0x2C, "SUB",   "b ia",                                    unimplemented],
    [0x2D, "SUB",   "w ia",                                    unimplemented],
    [0x2E, "PS:",   "Set segment override to program segment", ps],
    [0x2F, "ADJ4S", "",                                        unimplemented],

    [0x30, "XOR",   "",                                       unimplemented],
    [0x31, "XOR",   "",                                       unimplemented],
    [0x32, "XOR",   "",                                       unimplemented],
    [0x33, "XOR",   "Word XOR into register",                 xor_w_to_reg],
    [0x34, "XOR",   "",                                       unimplemented],
    [0x35, "XOR",   "",                                       unimplemented],
    [0x36, "SS:",   "Set segment override to stack segment",  ss],
    [0x37, "ADJBA", "",                                       unimplemented],
    [0x38, "CMP",   "Compare memory with byte",               cmp_b_f_rm],
    [0x39, "CMP",   "Compare memory with word",               unimplemented],
    [0x3A, "CMP",   "Compare byte with memory",               unimplemented],
    [0x3B, "CMP",   "Compare word with memory",               cmp_w_t_rm],
    [0x3C, "CMP",   "b, ia",                                  unimplemented],
    [0x3D, "CMP",   "w, ia",                                  cmp_aw_imm],
    [0x3E, "DS0:",  "Set segment override to data segment 0", ds0],
    [0x3F, "ADJBS", "",                                       unimplemented],

    [0x40, "INC AW", "Increment AW by 1", inc_aw],
    [0x41, "INC CW", "Increment CW by 1", inc_cw],
    [0x42, "INC DW", "Increment DW by 1", inc_dw],
    [0x43, "INC BW", "Increment BW by 1", inc_bw],
    [0x44, "INC SP", "Increment SP by 1", inc_sp],
    [0x45, "INC BP", "Increment BP by 1", inc_bp],
    [0x46, "INC IX", "Increment IX by 1", inc_ix],
    [0x47, "INC IY", "Increment IY by 1", inc_iy],
    [0x48, "DEC AW", "Decrement AW by 1", dec_aw],
    [0x49, "DEC CW", "Decrement CW by 1", dec_cw],
    [0x4A, "DEC DW", "Decrement DW by 1", dec_dw],
    [0x4B, "DEC BW", "Decrement BW by 1", dec_bw],
    [0x4C, "DEC SP", "Decrement SP by 1", dec_sp],
    [0x4D, "DEC BP", "Decrement BP by 1", dec_bp],
    [0x4E, "DEC IX", "Decrement IX by 1", dec_ix],
    [0x4F, "DEC IY", "Decrement IY by 1", dec_iy],

    [0x50, "PUSH AW", "Push value of AW register to stack",  push_aw],
    [0x51, "PUSH CW", "Push value of CW register to stack",  push_cw],
    [0x52, "PUSH DW", "Push value of DW register to stack",  push_dw],
    [0x53, "PUSH BW", "Push value of BW register to stack",  push_bw],
    [0x54, "PUSH SP", "Push value of SP register to stack",  push_sp],
    [0x55, "PUSH BP", "Push value of BP register to stack",  push_bp],
    [0x56, "PUSH IX", "Push value of IX register to stack",  push_ix],
    [0x57, "PUSH IY", "Push value of IY register to stack",  push_iy],
    [0x58, "POP AW",  "Pop value of AW register from stack", pop_aw],
    [0x59, "POP CW",  "Pop value of CW register from stack", pop_cw],
    [0x5A, "POP DW",  "Pop value of DW register from stack", pop_dw],
    [0x5B, "POP BW",  "Pop value of BW register from stack", pop_bw],
    [0x5C, "POP SP",  "Pop value of SP register from stack", pop_sp],
    [0x5D, "POP BP",  "Pop value of BP register from stack", pop_bp],
    [0x5E, "POP IX",  "Pop value of IX register from stack", pop_ix],
    [0x5F, "POP IY",  "Pop value of IY register from stack", pop_iy],

    [0x60, "PUSH R", "",                              unimplemented],
    [0x61, "POP R",  "",                              unimplemented],
    [0x62, "CHKIND", "",                              unimplemented],
    [0x63, "UNDEF",  "",                              unimplemented],
    [0x64, "REPNC",  "",                              unimplemented],
    [0x65, "REPC",   "",                              unimplemented],
    [0x66, "FPO2",   "",                              unimplemented],
    [0x67, "FPO2",   "",                              unimplemented],
    [0x68, "PUSH",   "",                              unimplemented],
    [0x69, "MUL",    "",                              unimplemented],
    [0x6A, "PUSH",   "",                              unimplemented],
    [0x6B, "MUL",    "",                              unimplemented],
    [0x6C, "INM",    "",                              unimplemented],
    [0x6D, "INM",    "",                              unimplemented],
    [0x6E, "OUTM",   "Output byte from memory at IX", outm_b],
    [0x6F, "OUTM",   "Output word from memory at IX", outm_w],

    [0x70, "BV",  "",                           unimplemented],
    [0x71, "BNV", "",                           unimplemented],
    [0x72, "BC",  "Branch if CY flag is 1",     unimplemented],
    [0x73, "BNC", "Branch if CY flag is 0",     bnc],
    [0x74, "BE",  "Branch if Z flag is 1",      be],
    [0x75, "BNE", "Branch if Z flag is 0",      bne],
    [0x76, "BNH", "",                           unimplemented],
    [0x77, "BH",  "",                           unimplemented],
    [0x78, "BN",  "",                           unimplemented],
    [0x79, "BP",  "",                           unimplemented],
    [0x7A, "BPE", "",                           unimplemented],
    [0x7B, "BPO", "",                           unimplemented],
    [0x7C, "BLT", "Branch if lesser",           unimplemented],
    [0x7D, "BGE", "Branch if greater or equal", unimplemented],
    [0x7E, "BLE", "Branch if lesser or equal",  unimplemented],
    [0x7F, "BGT", "Branch if greater",          unimplemented],

    [0x80, "IMM",  "Unsigned byte constant arithmetic",         imm_b],
    [0x81, "IMM",  "Unsigned word constant arithmetic",         imm_w],
    [0x82, "IMM",  "Sign-extended byte constant arithmetic",    imm_b_s],
    [0x83, "IMM",  "Sign-extended word constant arithmetic",    imm_w_s],
    [0x84, "TEST", "",                                          unimplemented],
    [0x85, "TEST", "",                                          unimplemented],
    [0x86, "XCH",  "",                                          unimplemented],
    [0x87, "XCH",  "",                                          unimplemented],
    [0x88, "MOV",  "Move byte to memory from register",         unimplemented],
    [0x89, "MOV",  "Move word to memory from register",         mov_w_from_reg_to_mem],
    [0x8A, "MOV",  "Move byte to register from memory",         unimplemented],
    [0x8B, "MOV",  "Move word to register from memory",         mov_w_to_reg],
    [0x8C, "MOV",  "Move word to memory from segment register", mov_w_from_sreg],
    [0x8D, "LDEA", "",                                          unimplemented],
    [0x8E, "MOV",  "Move word to segment register from memory", mov_w_to_sreg],
    [0x8F, "POP",  "rm",                                        unimplemented],

    [0x90, "NOP",         "Do nothing",                           nop],
    [0x91, "XCH CW",      "Switch values of CW and AW",           unimplemented],
    [0x92, "XCH DW",      "Switch values of DW and AW",           unimplemented],
    [0x93, "XCH BW",      "Switch values of BW and AW",           unimplemented],
    [0x94, "XCH SP",      "Switch values of SP and AW",           unimplemented],
    [0x95, "XCH BP",      "Switch values of BP and AW",           unimplemented],
    [0x96, "XCH IX",      "Switch values of IX and AW",           unimplemented],
    [0x97, "XCH IY",      "Switch values of IY and AW",           unimplemented],
    [0x98, "CVTBW",       "",                                     unimplemented],
    [0x99, "CVTBL",       "",                                     unimplemented],
    [0x9A, "CALL",        "Call a subroutine",                    unimplemented],
    [0x9B, "POLL",        "",                                     unimplemented],
    [0x9C, "PUSH PSW",    "Push value of PSW register to stack",  push_psw],
    [0x9D, "POP PSW",     "Pop value of PSW register from stack", pop_psw],
    [0x9E, "MOV PSW, AH", "",                                     unimplemented],
    [0x9F, "MOV AH, PSW", "",                                     unimplemented],

    [0xA0, "MOV AL", "Move byte into AL from memory",               mov_al_m],
    [0xA1, "MOV AW", "Move word into AW from memory",               mov_aw_m],
    [0xA2, "MOV",    "Move byte into memory from AL",               mov_m_al],
    [0xA3, "MOV",    "Move word into memory from AW",               mov_m_aw],
    [0xA4, "MOVBK",  "Move byte from memory at IX to memory at IY", unimplemented],
    [0xA5, "MOVBK",  "Move word from memory at IX to memory at IY", movbk_w],
    [0xA6, "CMPBK",  "",                                            unimplemented],
    [0xA7, "CMPBK",  "",                                            unimplemented],
    [0xA8, "TEST",   "",                                            unimplemented],
    [0xA9, "TEST",   "",                                            unimplemented],
    [0xAA, "STM",    "Store multiple bytes",                        stm_b],
    [0xAB, "STM",    "Store multiple words",                        stm_w],
    [0xAC, "LDM",    "b",                                           ldm_b],
    [0xAD, "LDM",    "w",                                           ldm_w],
    [0xAE, "CMPM",   "",                                            unimplemented],
    [0xAF, "CMPM",   "",                                            unimplemented],
    
    [0xB0, "MOV AL", "Move byte constant into AL", mov_al_i],
    [0xB1, "MOV CL", "Move byte constant into CL", mov_cl_i],
    [0xB2, "MOV DL", "Move byte constant into DL", mov_dl_i],
    [0xB3, "MOV BL", "Move byte constant into BL", mov_bl_i],
    [0xB4, "MOV AH", "Move byte constant into AH", mov_ah_i],
    [0xB5, "MOV CH", "Move byte constant into CH", mov_ch_i],
    [0xB6, "MOV BH", "Move byte constant into BH", mov_bh_i],
    [0xB7, "MOV DH", "Move byte constant into DH", mov_dh_i],
    [0xB8, "MOV AW", "Move word constant into AW", mov_aw_i],
    [0xB9, "MOV CW", "Move word constant into CW", mov_cw_i],
    [0xBA, "MOV DW", "Move word constant into DW", mov_dw_i],
    [0xBB, "MOV BW", "Move word constant into BW", mov_bw_i],
    [0xBC, "MOV SP", "Move word constant into SP", mov_sp_i],
    [0xBD, "MOV BP", "Move word constant into BP", mov_bp_i],
    [0xBE, "MOV IX", "Move word constant into IX", mov_ix_i],
    [0xBF, "MOV IY", "Move word constant into IY", mov_iy_i],

    [0xC0, "SHIFT",   "",                             unimplemented],
    [0xC1, "SHIFT",   "",                             unimplemented],
    [0xC2, "RET",     "",                             unimplemented],
    [0xC3, "RET",     "",                             unimplemented],
    [0xC4, "MOV",     "Move word to DS1 from AW",     mov_ds1_aw],
    [0xC5, "MOV",     "Move word to DS0 from AW",     mov_ds0_aw],
    [0xC6, "MOV",     "Move byte constant to memory", mov_mb_imm],
    [0xC7, "MOV",     "Move word constant to memory", mov_mw_imm],
    [0xC8, "PREPARE", "",                             unimplemented],
    [0xC9, "DISPOSE", "Delete a stack frame",         unimplemented],
    [0xCA, "RET",     "",                             unimplemented],
    [0xCB, "RET",     "",                             unimplemented],
    [0xCC, "BRK",     "",                             unimplemented],
    [0xCD, "BRK",     "",                             unimplemented],
    [0xCE, "BRKV",    "",                             unimplemented],
    [0xCF, "RETI",    "Return from interrupt, restoring PC, PS, and PSW", unimplemented],

    [0xD0, "SHIFT", "Byte shift",         unimplemented],
    [0xD1, "SHIFT", "Word shift",         shift_w],
    [0xD2, "SHIFT", "Byte shift to port", unimplemented],
    [0xD3, "SHIFT", "Word shift to port", unimplemented],
    [0xD4, "CVTBD", "",                   unimplemented],
    [0xD5, "CVTDB", "",                   unimplemented],
    [0xD6, "UNDEF", "",                   unimplemented],
    [0xD7, "TRANS", "",                   unimplemented],
    [0xD8, "FPO1",  "",                   nop],
    [0xD9, "FPO1",  "",                   nop],
    [0xDA, "FPO1",  "",                   nop],
    [0xDB, "FPO1",  "",                   nop],
    [0xDC, "FPO1",  "",                   nop],
    [0xDD, "FPO1",  "",                   nop],
    [0xDE, "FPO1",  "",                   nop],
    [0xDF, "FPO1",  "",                   nop],

    [0xE0, "DBNZE", "",                                    unimplemented],
    [0xE1, "DBNZE", "",                                    unimplemented],
    [0xE2, "DBNZ",  "Decrement CW and branch if not zero", dbnz],
    [0xE3, "BCWZ",  "Branch if CW is zero",                bcwz],
    [0xE4, "IN",    "b",                                   in_b],
    [0xE5, "IN",    "w",                                   in_w],
    [0xE6, "OUT",   "b",                                   out_b],
    [0xE7, "OUT",   "w",                                   out_w],
    [0xE8, "CALL",  "Call direct address",                 call_d],
    [0xE9, "BR",    "Branch near",                         br_near],
    [0xEA, "BR",    "Branch far",                          br_far],
    [0xEB, "BR",    "Branch short",                        br_short],
    [0xEC, "IN",    "b, v",                                in_b_v],
    [0xED, "IN",    "w, v",                                in_w_v],
    [0xEE, "OUT",   "b, v",                                out_b_v],
    [0xEF, "OUT",   "w, v",                                out_w_v],

    [0xF0, "BUSLOCK", "",                                             unimplemented],
    [0xF1, "UNDEF",   "",                                             unimplemented],
    [0xF2, "REPNE",   "",                                             unimplemented],
    [0xF3, "REP",     "Repeat next instruction until CW = 0",         rep],
    [0xF4, "HALT",    "",                                             unimplemented],
    [0xF5, "NOT1",    "",                                             unimplemented],
    [0xF6, "GROUP1",  "",                                             group1_b],
    [0xF7, "GROUP1",  "",                                             group1_w],
    [0xF8, "CLR1",    "Clear carry flag",                             clr1_cy],
    [0xF9, "SET1",    "Set carry flag",                               set1_cy],
    [0xFA, "DI",      "Reset IE flag and disable maskable interrupt", di],
    [0xFB, "EI",      "Set IE flag and enable maskable interrupt",    unimplemented],
    [0xFC, "CLR1",    "Clear direction flag",                         clr1_dir],
    [0xFD, "SET1",    "Set direction flag",                           set1_dir],
    [0xFE, "GROUP2",  "",                                             group2_b],
    [0xFF, "GROUP2",  "",                                             group2_w],
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
fn call_d (state: &mut CPU) -> u64 {
    let displace = state.next_i16();
    state.push_u16(state.pc);
    state.jump_i16(displace);
    match state.pc % 2 {
        0 => 7,
        1 => 9,
        _ => unreachable!()
    }
}

#[inline]
/// PC ← PC + disp
fn br_near (state: &mut CPU) -> u64 {
    let displace = state.next_i16();
    state.jump_i16(displace);
    7
}

#[inline]
/// PS ← seg
/// PC ← offset
fn br_far (state: &mut CPU) -> u64 {
    let offset  = state.next_u16();
    let segment = state.next_u16();
    state.pc = offset;
    state.ps = segment;
    7
}

#[inline]
fn br_short (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    state.jump_i8(displace);
    7
}

#[inline]
/// IE ← 0
fn di (state: &mut CPU) -> u64 {
    state.set_ie(false);
    2
}

#[inline]
/// CW ← CW – 1
/// Where CW ≠ 0: PC ← PC + ext-disp8
fn dbnz (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    state.cw = state.cw.overflowing_sub(1).0;
    if state.cw > 0 {
        state.jump_i8(displace);
        6
    } else {
        3
    }
}

#[inline]
/// Branch if CW is zero.
fn bcwz (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    if state.cw() == 0 {
        state.jump_i8(displace);
        6
    } else {
        3
    }
}

#[inline]
fn be (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    if state.z() {
        state.jump_i8(displace);
        6
    } else {
        3
    }
}

#[inline]
fn bne (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    if !state.z() {
        state.jump_i8(displace);
        6
    } else {
        3
    }
}

#[inline]
fn bc (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    if state.cy() {
        state.jump_i8(displace);
        6
    } else {
        3
    }
}

#[inline]
fn bnc (state: &mut CPU) -> u64 {
    let displace = state.next_i8();
    if !state.cy() {
        state.jump_i8(displace);
        6
    } else {
        3
    }
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
