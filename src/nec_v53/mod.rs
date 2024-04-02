/// https://datasheets.chipdb.org/NEC/V20-V30/U11301EJ5V0UMJ1.PDF

mod inst;
mod state;
#[cfg(test)] mod test;

pub use inst::*;
pub use state::*;
