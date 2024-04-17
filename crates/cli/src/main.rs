use mpcemu_v53;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    let mut memory = vec![];
    memory.extend_from_slice(&bin);
    memory.extend_from_slice(&bin);
    let mut cpu = mpcemu_v53::CPU::new(memory);
    let mut first: bool = true;
    let mut last_address: u32 = cpu.program_address();
    println!("\n\nRunning from {:x}:", cpu.program_address());
    loop {
        let address = cpu.program_address();
        cpu.step(first || last_address != address);
        last_address = address;
        first = false;
        if address == 0xFAD99 {
            return Ok(())
        }
    }
}
