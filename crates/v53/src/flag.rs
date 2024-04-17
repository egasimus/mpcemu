use crate::*;

macro_rules! define_flag {
    ($(#[$attr:meta])* $getter:ident $setter:ident $bit:literal) => {
        $(#[$attr])*
        impl CPU {
            pub fn $getter (&self) -> bool {
                (self.psw & (1 << $bit)) > 0
            }
            pub fn $setter (&mut self, value: bool) {
                if value {
                    self.psw = self.psw | (1 << $bit)
                } else {
                    self.psw = self.psw & (!(1 << $bit))
                }
            }
        }
    }
}

define_flag! {
    /// Overflow flag. Set when there is a signed overflow, otherwise reset.
    v set_v 11
}

define_flag! {
    /// Direction flag. Set this using SET1 DIR to process data in reverse order,
    /// reset it with CLR1 DIR.
    dir set_dir 10
}

define_flag! {
    /// Interrupt enable flag. When enabled, CPU reacts to interrupts.
    /// Set this with EI and reset it with DI.
    ie set_ie 9
}

define_flag! {
    /// Break/trace flag.
    brk set_brk 8
}

define_flag! {
    /// Sign flag. Equal to MSB of result.
    s set_s 7
}

define_flag! {
    /// Zero flag. Set when result is zero, otherwise reset.
    z set_z 6
}

define_flag! {
    /// Auxiliary carry flag. Set when there is unsigned overflow
    /// for low nibble, otherwise reset.
    ac set_ac 4
}

define_flag! {
    /// Parity flag. Set when there is 0 or even number of 1s in result.
    p set_p 2
}

define_flag! {
    /// Carry flag. Set when there is an unsigned overflow, otherwise reset.
    cy set_cy 0
}

/// Flag utilities.
impl CPU {
    /// Set parity, zero, and sign flags from word result.
    pub fn set_pzs (&mut self, result: u16) {
        self.set_p(determine_parity(result as u8));
        self.set_z(result == 0);
        self.set_s(result >> 15 == 1);
    }
    /// Set parity, zero, sign, carry, and overflow flags from word result.
    pub fn set_pzscyv (&mut self, result: u16, carry: bool, overflow: bool) {
        self.set_p(determine_parity(result as u8));
        self.set_z(result == 0);
        self.set_s(result >> 15 == 1);
        self.set_cy(carry);
        self.set_v(overflow);
    }
}

fn determine_parity (result: u8) -> bool {
    let mut ones = 0;
    for i in 0..8 {
        let s = result >> i;
        if s % 2 == 1 {
            ones += 1;
        }
    }
    ones % 2 == 0
}
