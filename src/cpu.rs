use std::{io, path::Path};

use crate::framebuffer::FrameBuffer;

#[derive(Default)]
pub struct Cpu {
    framebuffer: FrameBuffer,
}

impl Cpu {
    pub fn new<P: AsRef<Path>>(_rom_file: P) -> io::Result<Self> {
        Ok(Self::default())
    }
}
