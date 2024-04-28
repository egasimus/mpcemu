use mpcemu_v53::CPU as V53;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    let mut memory = vec![];
    memory.extend_from_slice(&bin);
    memory.extend_from_slice(&bin);
    let mut cpu = V53::new(memory);

    cpu.on_output(0x00E0, Box::new(|cpu: &V53| {
        let value = cpu.ports()[0x00E0];
        if (value as char).is_ascii() {
            let value = value as char;
            println!("0x00E0 -> '{value}'");
        } else {
            println!("0x00E0 -> 0x{value:02X}");
        }
    }));

    println!("\n\nRunning from {:x}:", cpu.program_address());
    loop {
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        cpu.step(true);
        // 0xF986C out 0E0h, al -> write to screen
        //if address == 0xFAD79
            //return Ok(())
        //}
    }
}
