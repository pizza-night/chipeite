pub struct FrameBuffer {
    vec: Box<[[bool; 64]; 32]>,
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            vec: vec![[false; 64]; 32].try_into().unwrap(),
        }
    }
}

pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub state: bool,
}

impl FrameBuffer {
    pub fn reset(&mut self) {
        for line in self.vec.iter_mut() {
            line.fill(false)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.vec.iter().enumerate().flat_map(|(y, lines)| {
            lines
                .iter()
                .enumerate()
                .map(move |(x, &state)| Pixel { x, y, state })
        })
    }

    pub fn write(&mut self, x: usize, y: usize, bytes: &[u8]) -> bool {
        let mut colided = false;
        self.vec
            .iter_mut()
            .skip(y)
            .flat_map(|line| line.iter_mut())
            .skip(x)
            .zip(bytes.iter().flat_map(|u| iterate_u8(*u)))
            .for_each(|(v, b)| {
                let old = *v;
                *v ^= b;
                if !colided && old && !*v {
                    colided = true
                }
            });
        colided
    }
}

fn iterate_u8(u: u8) -> impl Iterator<Item = bool> {
    let mut eight_states = u;
    std::iter::from_fn(move || {
        let r = eight_states & 0b1000_0000;
        eight_states = eight_states.wrapping_shl(1);
        Some(r != 0)
    })
    .take(8)
}
