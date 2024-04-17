use crate::*;

impl CPU {

    pub fn dump (&self) {
        let start = self.ps.saturating_sub(4);
        let end   = self.ps.saturating_add(4);
        for row in start..=end {
            print!("\n{}{:6X}|", if row == self.ps {">"} else {" "}, row);
            for col in 0..0x10 {
                print!(" {:02x}", self.memory[row as usize * 0x10 + col]);
            }
        }
    }

    pub fn dump_state (&self, pc: u16) {
        print!("\n           AW   BW   CW   DW   DS0  DS1  BP   IX   IY   SS   SP   PS   PC   ");
        print!("\n           {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}:{:04X} {:04X}:{:04X}",
            self.aw(), self.bw(), self.cw(), self.dw(),
            self.ds0(), self.ds1(), self.bp(), self.ix(), self.iy(),
            self.ss(), self.sp(), self.ps(), pc);
        print!("\n           {} {} {} {} {} {} {} {} {}\n",
            if self.v()   { "V  " } else { "   " },
            if self.dir() { "DIR" } else { "   " },
            if self.ie()  { "IE " } else { "   " },
            if self.brk() { "BRK" } else { "   " },
            if self.s()   { "S  " } else { "   " },
            if self.z()   { "Z  " } else { "   " },
            if self.ac()  { "AC " } else { "   " },
            if self.p()   { "P  " } else { "   " },
            if self.cy()  { "CY " } else { "   " },);
    }

    pub fn dump_instruction (&self, addr: u32, name: &str, bytes: &[u8]) {
        print!("\n\n{:10} {addr:05X}  {name:15}  {:02X?}\n",
            self.clock, &bytes);
    }

    pub fn dump_stack (&self, rows: usize) {
        self.dump_segment(self.ss().saturating_sub(1), self.sp(), 4)
    }

    pub fn dump_segment (&self, segment: u16, offset: u16, count: u16) {
        for row in 0..count {
            let start = (((segment as usize * 0x10) + offset as usize) / 0x10 + row as usize) * 0x10;
            print!("\n{:6X}|", start);
            for col in 0..0x10 {
                print!(" {:02x}", self.memory()[start + col]);
            }
        }
    }

    pub fn dump_at (&self, start: usize, per_row: u8, rows: u8) {
        for i in 0..rows {
            let offset = start + i as usize * per_row as usize;
            print!("\n{:6X}|", offset);
            for j in 0..per_row {
                print!(" {:02x}", self.memory()[offset as usize + j as usize]);
            }
        }
    }

}
