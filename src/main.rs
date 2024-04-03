mod nec_v53;
fn main () -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = nec_v53::CPU::new();
    let bin = std::fs::read("./data/mpc3000-v3.12.bin")?;
    cpu.memory = vec![];
    cpu.memory.extend_from_slice(&bin);
    cpu.memory.extend_from_slice(&bin);
    for i in 0..0x4000 {
        print!("\n {:8x}", i * 0x20);
        for j in 0..32 {
            print!(" {:2x}", cpu.memory[i * 0x20 + j]);
        }
    }
    let mut last_address: usize = 0;
    let mut last_opcode:  u8    = 0;
    loop {
        //let segment = cpu.ps() as u32 * 0x10;
        let address = cpu.address();
        let opcode  = cpu.memory[address];
        //println!("0x{:x} + 0x{:x} = 0x{:x}", segment, cpu.pc, segment + cpu.pc as u32);
        if last_address != address {
            print!("\n{address:x} {opcode:x} {}", nec_v53::get_instruction_name(opcode));
        } else {
            print!(".")
        }
        cpu.step();
        last_address = address;
        last_opcode  = opcode;
    }
    Ok(())
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
