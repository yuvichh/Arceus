#[derive(Debug)]
struct Cpu {
    regs: [u32;32],
    pc: u32,
    mem: Vec<u8>,
}

enum Instruction {
    Add  { rd: usize, rs1: usize, rs2: usize },
    Addi { rd: usize, rs1: usize, imm: i32 },
}

impl Cpu {
    fn fetch() {}
    fn execute(&mut self,inst: Instruction) {
        match inst {
            Instruction::Add{rd,rs1,rs2} => {
                self.regs[rd] = self.regs[rs1] + .wrapping_add(self.regs[rs2]);
            },
            Instruction::Addi{ rd, rs1, imm} => {
                self.regs[rd] = self.regs[rs1].wrapping_add(imm as u32);
            }
           }
        }
    }
}

fn main () {
    
    let mut cpu = Cpu {
        regs: [0; 32],
        pc: 0,
        mem: vec![0u8;4096],
    };
    cpu.regs[0] = 5;
    for i in 0..=31 {
     println!("{:?}",cpu.regs[i]);
    }

    println!("Hello World !");
}


