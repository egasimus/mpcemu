mod nec_v53;
fn main () -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    let mut memory = vec![];
    memory.extend_from_slice(&bin);
    memory.extend_from_slice(&bin);
    let mut cpu = nec_v53::CPU::new(memory);
    for i in 0..0x4000 {
        print!("\n {:8X}", i * 0x20);
        for j in 0..32 {
            print!(" {:02x}", cpu.memory()[i * 0x20 + j]);
        }
    }
    let mut first: bool = true;
    let mut last_address: usize = cpu.address();
    let mut last_opcode:  u8    = 0;
    let mut last_clock:   u64   = cpu.clock;
    println!("\n\nRunning from {:x}:", cpu.address());
    loop {
        let clock   = cpu.clock;
        let address = cpu.address();
        cpu.step();
        let opcode  = cpu.opcode();
        let name    = nec_v53::get_instruction_name(opcode);
        let info    = nec_v53::get_instruction_description(opcode);
        if first || last_address != address {
            print!("\n{last_clock:10}  {address:05X}  {opcode:02x}  {name:10}  {info}");
        } else {
            print!(".")
        }
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
