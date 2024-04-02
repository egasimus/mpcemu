mod nec_v53;
fn main () -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = nec_v53::State::new();
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    cpu.memory = vec![];
    cpu.memory.extend_from_slice(&bin);
    cpu.memory.extend_from_slice(&bin);
    for i in 0..0x8000 {
        println!(
            "{:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x} {:2x}",
            cpu.memory[i * 0x10 + 0],
            cpu.memory[i * 0x10 + 1],
            cpu.memory[i * 0x10 + 2],
            cpu.memory[i * 0x10 + 3],
            cpu.memory[i * 0x10 + 4],
            cpu.memory[i * 0x10 + 5],
            cpu.memory[i * 0x10 + 6],
            cpu.memory[i * 0x10 + 7],
            cpu.memory[i * 0x10 + 8],
            cpu.memory[i * 0x10 + 9],
            cpu.memory[i * 0x10 + 10],
            cpu.memory[i * 0x10 + 11],
            cpu.memory[i * 0x10 + 12],
            cpu.memory[i * 0x10 + 13],
            cpu.memory[i * 0x10 + 14],
            cpu.memory[i * 0x10 + 15],
        )
    }
    loop {
        let segment = cpu.ps as u32 * 0x10;
        let address = cpu.address();
        let opcode  = cpu.memory[address];
        //println!("0x{:x} + 0x{:x} = 0x{:x}", segment, cpu.pc, segment + cpu.pc as u32);
        println!("{address:x} {opcode:x} {}", nec_v53::get_instruction_name(opcode));
        cpu.step();
    }
    Ok(())
}
