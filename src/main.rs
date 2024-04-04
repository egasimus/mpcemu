mod nec_v53;
fn main () -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = nec_v53::CPU::new();
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    cpu.memory = vec![];
    cpu.memory.extend_from_slice(&bin);
    cpu.memory.extend_from_slice(&bin);
    for i in 0..0x4000 {
        print!("\n {:8X}", i * 0x20);
        for j in 0..32 {
            print!(" {:02x}", cpu.memory[i * 0x20 + j]);
        }
    }
    let mut last_address: usize = cpu.address();
    let mut last_opcode:  u8    = cpu.opcode();
    let mut last_clock:   u64   = cpu.clock;
    println!("\n\nRunning from {:x}:", cpu.address());
    loop {
        cpu.step();
        let clock   = cpu.clock;
        let address = cpu.address();
        let opcode  = cpu.opcode();
        let name    = nec_v53::get_instruction_name(opcode);
        let info    = nec_v53::get_instruction_description(opcode);
        if last_address != address {
            print!("\n{clock:10}  {address:X}  {opcode:02x}  {name:10}  {info}");
        } else {
            print!(".")
        }
        last_address = address;
        last_opcode  = opcode;
        last_clock   = cpu.clock;
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
