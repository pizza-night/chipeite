mod instruction;
mod instruction_set;

use crate::memory::Memory;

use self::instruction::Instruction;

pub struct Cpu {
    program_counter: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self { program_counter: 0 }
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        let inst = memory.fetch(self.program_counter);
        let (id, inst) = inst.one();
        match id {
            0 => self.zeroth(inst),
            _ => unreachable!()
        }
    }

    pub fn zeroth(&mut self, inst: Instruction<instruction::Three>) {
        let (middle_two, cls_or_ret) = inst.two();
        match middle_two {
            0x0E => match cls_or_ret.one().0 {
                0x0 => todo!("cls"),
                0xE => todo!("ret"),
                _ => unreachable!(),
            }
            _ => todo!("sys"),
        }
    }
}

impl Memory {
    fn fetch(&self, pc: u16) -> Instruction<instruction::Four> {
        let pc = pc as usize;
        Instruction::from_bytes([self.ram[pc], self.ram[pc + 1]])
    }
}
