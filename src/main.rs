mod nec_v53;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    let mut memory = vec![];
    memory.extend_from_slice(&bin);
    memory.extend_from_slice(&bin);
    let mut cpu = nec_v53::CPU::new(memory);
    //for i in 0..0x4000 {
        //print!("\n{:6X}|", i * 0x20);
        //for j in 0..16 {
            //print!(" {:02x}", cpu.memory()[i * 0x20 + j]);
        //}
        //print!(" |");
        //for j in 16..32 {
            //print!(" {:02x}", cpu.memory()[i * 0x20 + j]);
        //}
    //}
    let mut first: bool = true;
    let mut last_address: usize = cpu.program_address();
    let mut last_opcode:  u8    = 0;
    let mut last_clock:   u64   = cpu.clock;
    println!("\n\nRunning from {:x}:", cpu.program_address());
    loop {
        let clock   = cpu.clock;
        let address = cpu.program_address();
        cpu.step();
        let opcode  = cpu.opcode();
        let name    = nec_v53::get_instruction_name(opcode);
        let info    = nec_v53::get_instruction_description(opcode);
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
        //if clock > 1800000 {
            //println!();
            //for i in 0xF198..=0xF19A {
                //print!("\n{:6X}|", i * 0x10);
                //for j in 0..16 {
                    //print!(" {:02x}", cpu.memory()[i * 0x10 + j]);
                //}
                ////print!(" |");
                ////for j in 16..32 {
                    ////print!(" {:02x}", cpu.memory()[i * 0x20 + j]);
                ////}
            //}
            //println!();
        //}
        last_address = address;
        last_opcode  = opcode;
        last_clock   = cpu.clock;
        first = false;
    }
}

#[macro_export] macro_rules! define_instruction_set (

    ($([$code:literal, $inst:literal, $info:literal, $impl:ident],)+$(,)?) => {

        #[allow(unused)]
        pub fn get_instruction_name (code: u8) -> &'static str {
            match code {
                $($code => $inst),+,
                _ => panic!("undefined instruction {}", code),
            }
        }

        #[allow(unused)]
        pub fn get_instruction_description (code: u8) -> &'static str {
            match code {
                $($code => $info),+,
                _ => panic!("undefined instruction {}", code),
            }
        }

        #[allow(unused)]
        pub fn execute_instruction (state: &mut CPU, code: u8) -> u64 {
            match code {
                $($code => $impl(state)),+,
                _ => panic!("undefined instruction {}", code),
            }
        }

    }

);
