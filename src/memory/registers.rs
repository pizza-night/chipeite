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
    type Output = u8;
    fn index(&self, index: Register) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.values[index as usize]
    }
}

impl From<u8> for Register {
    fn from(v: u8) -> Self {
        match v {
            0x0 => Register::V0,
            0x1 => Register::V1,
            0x2 => Register::V2,
            0x3 => Register::V3,
            0x4 => Register::V4,
            0x5 => Register::V5,
            0x6 => Register::V6,
            0x7 => Register::V7,
            0x8 => Register::V8,
            0x9 => Register::V9,
            0xA => Register::VA,
            0xB => Register::VB,
            0xC => Register::VC,
            0xD => Register::VD,
            0xE => Register::VE,
            0xF => Register::VF,
            _ => panic!("Invalid register: {}", v),
        }
    }
}
