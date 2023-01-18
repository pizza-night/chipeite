#[derive(Debug, Default)]
pub struct KeyState(u16);

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Key {
    fn selector(self) -> u16 {
        0b1000_0000_0000_0000 >> (self as u16)
    }
}

impl KeyState {
    fn get(&self, index: Key) -> bool {
        (self.0 & index.selector()) != 0
    }

    fn set(&mut self, index: Key) {
        self.0 |= index.selector();
    }

    fn unset(&mut self, index: Key) {
        self.0 &= !index.selector()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALUES: [Key; 16] = [
        Key::Zero,
        Key::One,
        Key::Two,
        Key::Three,
        Key::Four,
        Key::Five,
        Key::Six,
        Key::Seven,
        Key::Eight,
        Key::Nine,
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
    ];

    #[test]
    fn setting_and_getting_it_works() {
        let mut keystate = KeyState::default();
        for k in VALUES {
            keystate.set(k);
            assert!(keystate.get(k));
        }
    }

    #[test]
    fn clearing_a_works() {
        let mut keystate = KeyState(0xFF);
        for k in VALUES {
            keystate.unset(k);
            assert!(!keystate.get(k));
        }
    }
}
