use mpcemu_v53::CPU as V53;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    let mut memory = vec![];
    memory.extend_from_slice(&bin);
    memory.extend_from_slice(&bin);
    let mut cpu = V53::new(memory);
    cpu.on_output(0x00E0, Box::new(|cpu: &V53| {
        let value = cpu.ports()[0x00E0] as char;
        if value.is_ascii() {
            println!("0x00E0 -> {value}");
        } else {
            println!("0x00E0 -> (unprintable)");
        }
    }));
    let mut first: bool = true;
    let mut last_address: u32 = cpu.program_address();
    println!("\n\nRunning from {:x}:", cpu.program_address());
    loop {
        let address = cpu.program_address();
        cpu.step(false && (first || last_address != address));
        last_address = address;
        first = false;
        // 0xF986C out 0E0h, al -> write to screen
        //if address == 0xFAD79
            //return Ok(())
        //}
    }
}
