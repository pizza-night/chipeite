mod instruction;

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
            1 => self.jump(inst),
            2 => self.call(inst, memory),
            3 => self.skip_if_equal(inst, memory),
            4 => self.skip_if_not_equal(inst, memory),
            5 => self.skip_if_equal_reg(inst, memory),
            6 => self.set_reg(inst, memory),
            7 => self.add_reg(inst, memory),
            _ => unreachable!(),
        }
    }

    pub fn zeroth(&mut self, inst: Instruction<instruction::Three>) {
        let (middle_two, cls_or_ret) = inst.two();
        match middle_two {
            0x0E => match cls_or_ret.one().0 {
                0x0 => todo!("cls"),
                0xE => todo!("ret"),
                _ => unreachable!(),
            },
            _ => todo!("sys"),
        }
    }

    pub fn jump(&mut self, inst: Instruction<instruction::Three>) {
        self.program_counter = inst.three().0;
    }

    pub fn call(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let addr = inst.three().0;
        memory.stack.call(self.program_counter);
        self.program_counter = addr;
    }

    pub fn skip_if_equal(&mut self, inst: Instruction<instruction::Three>, memory: &Memory) {
        let (reg, inst_val) = inst.one();
        if memory.registers[reg.into()] == inst_val.two().0 as _ {
            self.program_counter += 2;
        }
    }

    pub fn skip_if_not_equal(&mut self, inst: Instruction<instruction::Three>, memory: &Memory) {
        let (reg, inst_val) = inst.one();
        if memory.registers[reg.into()] != inst_val.two().0 as _ {
            self.program_counter += 2;
        }
    }

    pub fn skip_if_equal_reg(&mut self, inst: Instruction<instruction::Three>, memory: &Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        if memory.registers[reg1.into()] == memory.registers[reg2.into()] {
            self.program_counter += 2;
        }
    }

    pub fn set_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, val) = inst.one();
        let (val, _) = val.two();
        memory.registers[reg.into()] = val;
    }

    pub fn add_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, val) = inst.one();
        let (val, _) = val.two();
        memory.registers[reg.into()] += val;
    }

    pub fn set_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        memory.registers[reg1.into()] = memory.registers[reg2.into()];
    }

    pub fn or_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        memory.registers[reg1.into()] |= memory.registers[reg2.into()];
    }

    pub fn and_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        memory.registers[reg1.into()] &= memory.registers[reg2.into()];
    }

    pub fn xor_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        memory.registers[reg1.into()] ^= memory.registers[reg2.into()];
    }

    pub fn add_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        let (val, overflow) =
            memory.registers[reg1.into()].overflowing_add(memory.registers[reg2.into()]);
        memory.registers[reg1.into()] = val;
        memory.registers[crate::memory::registers::Register::VF] = overflow as _;
    }

    pub fn sub_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        let (val, overflow) =
            memory.registers[reg1.into()].overflowing_sub(memory.registers[reg2.into()]);
        memory.registers[reg1.into()] = val;
        memory.registers[crate::memory::registers::Register::VF] = !overflow as _;
    }
}

impl Memory {
    fn fetch(&self, pc: u16) -> Instruction<instruction::Four> {
        let pc = pc as usize;
        Instruction::from_bytes([self.ram[pc], self.ram[pc + 1]])
    }
}
