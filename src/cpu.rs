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
            8 => self.eighth(inst, memory),
            9 => self.skip_if_not_equal_reg(inst, memory),
            0xA => self.set_i(inst, memory),
            0xB => self.jump_reg(inst, memory),
            0xC => self.rand(inst, memory),
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

    pub fn eighth(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (_, op_code) = inst.two();
        let op_code = op_code.one().0;
        match op_code {
            0x0 => self.set_reg_reg(inst, memory),
            0x1 => self.or_reg_reg(inst, memory),
            0x2 => self.and_reg_reg(inst, memory),
            0x3 => self.xor_reg_reg(inst, memory),
            0x4 => self.add_reg_reg(inst, memory),
            0x5 => self.sub_reg_reg(inst, memory),
            0x6 => self.shr_reg_reg(inst, memory),
            0x7 => self.subn_reg_reg(inst, memory),
            0xE => self.shl_reg_reg(inst, memory),
            _ => unreachable!(),
        }
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

    pub fn shr_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, _) = inst.one();
        let val = memory.registers[reg1.into()];
        memory.registers[crate::memory::registers::Register::VF] = val & 1;
        memory.registers[reg1.into()] = val.wrapping_shr(1);
    }

    pub fn subn_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        let (val, overflow) =
            memory.registers[reg2.into()].overflowing_sub(memory.registers[reg1.into()]);
        memory.registers[reg1.into()] = val;
        memory.registers[crate::memory::registers::Register::VF] = !overflow as _;
    }

    pub fn shl_reg_reg(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg1, _) = inst.one();
        let val = memory.registers[reg1.into()];
        memory.registers[crate::memory::registers::Register::VF] = val >> 7;
        memory.registers[reg1.into()] = val.wrapping_shl(1);
    }

    // 9
    pub fn skip_if_not_equal_reg(
        &mut self,
        inst: Instruction<instruction::Three>,
        memory: &Memory,
    ) {
        let (reg1, reg2) = inst.one();
        let (reg2, _) = reg2.one();
        if memory.registers[reg1.into()] != memory.registers[reg2.into()] {
            self.program_counter += 2;
        }
    }

    // A
    pub fn set_i(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (val, _) = inst.three();
        memory.registers.image = val;
    }

    // B
    pub fn jump_reg(&mut self, inst: Instruction<instruction::Three>, memory: &Memory) {
        let (val, _) = inst.three();
        self.program_counter =
            memory.registers[crate::memory::registers::Register::V0] as u16 + val;
    }

    // C
    pub fn rand(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, val) = inst.one();
        let (val, _) = val.two();
        memory.registers[reg.into()] = rand::random::<u8>() & val;
    }

    //D
    pub fn draw(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        todo!()
        //    let (reg1, reg2) = inst.one();
        //    let (reg2, val) = reg2.one();
        //    let (val, _) = val.two();
        //    let x = memory.registers[reg1.into()] as usize;
        //    let y = memory.registers[reg2.into()] as usize;
        //    let mut collision = false;
        //    for i in 0..val {
        //        let sprite = memory.memory[memory.registers.image as usize + i];
        //        for j in 0..8 {
        //            let pixel = (sprite >> (7 - j)) & 1;
        //            if pixel == 1 {
        //                let index = (x + j + (y + i) * 64) % 2048;
        //                if memory.display[index] == 1 {
        //                    collision = true;
        //                }
        //                memory.display[index] ^= 1;
        //            }
        //        }
        //    }
        //    memory.registers[crate::memory::registers::Register::VF] = collision as _;
    }
}

impl Memory {
    fn fetch(&self, pc: u16) -> Instruction<instruction::Four> {
        let pc = pc as usize;
        Instruction::from_bytes([self.ram[pc], self.ram[pc + 1]])
    }
}
