/// <https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF>

mod reg;
mod flag;
mod mem;
mod math;
mod inst;
#[cfg(test)] mod test;

pub use self::{
    reg::*,
    flag::*,
    mem::*,
    math::*,
    inst::*,
};

pub struct CPU {
    pub clock:    u64,
    pub memory:   Vec<u8>,
    pub ports:    [u8;65536],
    pub internal: [u8;256],
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
            opcode:   0xF1
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

    pub fn opcode (&self) -> u8 {
        self.opcode
    }

}
