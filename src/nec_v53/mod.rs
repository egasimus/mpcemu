/// https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF

mod reg;
mod flag;
mod inst;
mod mem;
#[cfg(test)] mod test;

pub use self::mem::Segment;
pub use self::{inst::*, reg::*};

pub struct CPU {
    pub clock:    u64,
    pub memory:   Vec<u8>,
    pub ports:    [u8;65536],
    pub internal: [u8;256],
    /// General purpose register A, whole word
    /// Default for:
    /// - Word multiplication/division
    /// - Word input/output
    /// - Data exchange
    pub aw:      u16,
    /// General purpose register B, whole word
    /// Default for:
    /// - Data exchange (table reference)
    pub bw:      u16,
    /// General purpose register C, whole word
    /// Default for:
    /// - Loop control branch
    /// - Repeat prefix
    pub cw:      u16,
    /// General purpose register D, whole word
    /// Default for:
    /// - Word multiplication/division
    /// - Indirect addressing input/output
    pub dw:      u16,
    pub ps:      u16,
    pub ss:      u16,
    pub ds0:     u16,
    pub ds1:     u16,
    pub sp:      u16,
    pub bp:      u16,
    pub pc:      u16,
    pub psw:     u16,
    pub ix:      u16,
    pub iy:      u16,
    pub segment: Option<Segment>,
}

impl CPU {

    pub fn new () -> Self {
        Self {
            clock:    0x0000,
            memory:   vec![0x00;65536],
            ports:    [0x00;65536],
            internal: [0x00;256],
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
            psw:      0b1111000000000010,
            ix:       0x0000,
            iy:       0x0000,
            segment:  None,
        }
    }

    /// Read and execute the next instruction in the program
    pub fn step (&mut self) {
        let opcode = self.next_u8();
        self.clock += execute_instruction(self, opcode);
        // Reset segment override, except if it was just set:
        if !((opcode == 0x26) || (opcode == 0x2E) || (opcode == 0x36) || (opcode == 0x3E)) {
            self.segment = None
        }
    }
}
