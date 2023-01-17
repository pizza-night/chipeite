use std::ops::{Index, IndexMut};

#[repr(u8)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

#[derive(Default)]
pub struct Registers {
    values: [u8; 16],
    pub image: u8,
}

impl Index<Register> for Registers {
    type Output: = u8;
    fn index(&self, index: Register) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.values[index as usize]
    }
}
