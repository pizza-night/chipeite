pub struct Stack {
    sp: usize,
    bytes: [u16; 64 / 2],
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            sp: 0,
            bytes: [0; 64 / 2],
        }
    }
}

impl Stack {
    pub fn call(&mut self, current_pc: u16) {
        self.bytes[self.sp] = current_pc;
        self.sp += 1;
    }

    pub fn ret(&mut self) -> u16 {
        self.sp = self
            .sp
            .checked_sub(1)
            .expect("illegal instruction, tried to return but stack is empty");
        self.bytes[self.sp]
    }
}
