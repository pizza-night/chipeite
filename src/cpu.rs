mod instruction;
mod timer;

use crate::memory::Memory;

use self::{instruction::Instruction, timer::Timers};

pub struct Cpu {
    program_counter: u16,
    timers: Timers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: 0x200,
            timers: Timers::new(),
        }
    }

    pub fn execute(&mut self, memory: &mut Memory) {
        let inst = memory.fetch(self.program_counter);
        #[cfg(debug_assertions)]
        {
            println!("executing {inst:?}")
        }
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
            0xD => self.draw(inst, memory),
            0xE => self.eth(inst, memory),
            _ => unreachable!(),
        }
        self.program_counter += 2;
        self.timers.count_down();
        if self.timers.sound() == 0 {
            memory.video.stop_beep();
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

    pub fn eth(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (_, op_code) = inst.one();
        let op_code = op_code.two().0;
        match op_code {
            0x9E => self.skip_if_key_pressed(inst, memory),
            0xA1 => self.skip_if_key_not_pressed(inst, memory),
            _ => unreachable!(),
        }
    }

    // Ex9E
    pub fn skip_if_key_pressed(&mut self, inst: Instruction<instruction::Three>, memory: &Memory) {
        let (reg, _) = inst.one();
        let key = memory.registers[reg.into()];
        if memory.key_state.get(key.into()) {
            self.program_counter += 2;
        }
    }

    // ExA1
    pub fn skip_if_key_not_pressed(
        &mut self,
        inst: Instruction<instruction::Three>,
        memory: &Memory,
    ) {
        let (reg, _) = inst.one();
        let key = memory.registers[reg.into()];
        if !memory.key_state.get(key.into()) {
            self.program_counter += 2;
        }
    }

    pub fn fth(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (_, op_code) = inst.one();
        let op_code = op_code.two().0;
        match op_code {
            0x07 => self.get_delay_timer(inst, memory),
            0x0A => self.wait_for_key(inst, memory),
            0x15 => self.set_delay_timer(inst, memory),
            0x18 => self.set_sound_timer(inst, memory),
            0x1E => self.add_to_i(inst, memory),
            0x29 => self.set_i_to_sprite(inst, memory),
            0x33 => self.store_bcd(inst, memory),
            0x55 => self.store_registers(inst, memory),
            0x65 => self.load_registers(inst, memory),
            _ => unreachable!(),
        }
    }

    // Fx07
    pub fn get_delay_timer(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        //memory.registers[reg.into()] = memory.delay_timer;
        todo!()
    }

    // Fx0A
    pub fn wait_for_key(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        if let Some(key) = memory.video.wait_for_key() {
            memory.registers[reg.into()] = key as u8;
        } else {
            panic!("No key pressed");
        }
    }

    // Fx15
    pub fn set_delay_timer(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        //memory.delay_timer = memory.registers[reg.into()];
        todo!()
    }

    // Fx18
    pub fn set_sound_timer(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        //memory.sound_timer = memory.registers[reg.into()];
        todo!()
    }

    // Fx1E
    pub fn add_to_i(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        let val = memory
            .registers
            .image
            .checked_add(memory.registers[reg.into()] as u16)
            .unwrap();
        memory.registers.image = val;
    }

    // Fx29
    pub fn set_i_to_sprite(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        let val = memory.registers[reg.into()] as u16;
        memory.registers.image = val * 5;
    }

    // Fx33
    pub fn store_bcd(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        let val = memory.registers[reg.into()];
        memory.ram[memory.registers.image as usize] = val / 100;
        memory.ram[memory.registers.image as usize + 1] = (val / 10) % 10;
        memory.ram[memory.registers.image as usize + 2] = val % 10;
    }

    // Fx55
    pub fn store_registers(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        for i in 0u8..=reg {
            memory.ram[(memory.registers.image + i as u16) as usize] = memory.registers[i.into()];
        }
        memory.registers.image += reg as u16 + 1;
    }

    // Fx65
    pub fn load_registers(&mut self, inst: Instruction<instruction::Three>, memory: &mut Memory) {
        let (reg, _) = inst.one();
        for i in 0u8..=reg {
            memory.registers[i.into()] = memory.ram[(memory.registers.image + i as u16) as usize];
        }
        memory.registers.image += reg as u16 + 1;
    }
}

impl Memory {
    fn fetch(&self, pc: u16) -> Instruction<instruction::Four> {
        let pc = pc as usize;
        Instruction::from_bytes([self.ram[pc], self.ram[pc + 1]])
    }
}
