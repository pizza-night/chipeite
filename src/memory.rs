mod framebuffer;
pub mod registers;
mod stack;

use self::{framebuffer::FrameBuffer, registers::Registers, stack::Stack};

const FOUR_K: usize = 4 * 1024;

pub struct Memory {
    pub framebuffer: FrameBuffer,
    pub ram: [u8; FOUR_K],
    pub registers: Registers,
    pub stack: Stack,
    pub sound_timer: u8,
    pub delay_timer: u8,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            framebuffer: FrameBuffer::default(),
            ram: [0; FOUR_K],
            registers: Registers::default(),
            stack: Stack::default(),
            sound_timer: 0,
            delay_timer: 0,
        }
    }
}
