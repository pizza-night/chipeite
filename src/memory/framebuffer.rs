#[derive(Default)]
pub struct FrameBuffer {}

pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub state: bool,
}

impl FrameBuffer {
    pub fn iter(&self) -> impl Iterator<Item = Pixel> {
        std::iter::empty()
    }
}
