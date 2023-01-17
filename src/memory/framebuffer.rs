#[derive(Default)]
pub struct FrameBuffer {}

pub struct Pixel {
    x: usize,
    y: usize,
    state: bool,
}

impl FrameBuffer {
    pub fn iter(&self) -> impl Iterator<Item = Pixel> {
        std::iter::empty()
    }
}
