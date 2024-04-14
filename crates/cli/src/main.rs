use mpcemu_v53;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    let mut memory = vec![];
    memory.extend_from_slice(&bin);
    memory.extend_from_slice(&bin);
    let mut cpu = mpcemu_v53::CPU::new(memory);
    let mut first: bool = true;
    let mut last_address: usize = cpu.program_address();
    let mut last_clock:   u64   = cpu.clock;
    println!("\n\nRunning from {:x}:", cpu.program_address());
    loop {
        let address = cpu.program_address();
        cpu.step();
        let opcode  = cpu.opcode();
        let name    = mpcemu_v53::get_instruction_name(opcode);
        let info    = mpcemu_v53::get_instruction_description(opcode);
        if first || last_address != address {
            print!("\n\n{last_clock:10} {address:05X}  {opcode:02X}  {name:10}  {info}");
            print!("\n           AW={:04X} BW={:04X} CW={:04X} DW={:04X} PS={:04X} SS={:04X} DS0={:04X} DS1={:04X} IX={:04X} IY={:04X}",
                cpu.aw(), cpu.bw(), cpu.cw(), cpu.dw(),
                cpu.ps(), cpu.ss(), cpu.ds0(), cpu.ds1(),
                cpu.ix(), cpu.iy());
            print!("\n           V={} DIR={} IE={} BRK={} S={} Z={} AC={} P={} CY={}",
                cpu.v() as u8, cpu.dir() as u8, cpu.ie() as u8, cpu.brk() as u8,
                cpu.s() as u8, cpu.z() as u8, cpu.ac() as u8, cpu.p() as u8,
                cpu.cy() as u8);
        } else {
            //print!(".")
        }
        last_address = address;
        last_clock   = cpu.clock;
        first = false;
    }
}
