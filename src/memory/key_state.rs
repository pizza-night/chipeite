use sdl2::keyboard::Keycode;

#[derive(Debug, Default)]
pub struct KeyState(u16);

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Key {
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

impl TryFrom<Keycode> for Key {
    type Error = ();
    fn try_from(value: Keycode) -> Result<Self, Self::Error> {
        match value {
            Keycode::Num0 => Ok(Key::Zero),
            Keycode::Num1 => Ok(Key::One),
            Keycode::Num2 => Ok(Key::Two),
            Keycode::Num3 => Ok(Key::Three),
            Keycode::Num4 => Ok(Key::Four),
            Keycode::Num5 => Ok(Key::Five),
            Keycode::Num6 => Ok(Key::Six),
            Keycode::Num7 => Ok(Key::Seven),
            Keycode::Num8 => Ok(Key::Eight),
            Keycode::Num9 => Ok(Key::Nine),
            Keycode::A => Ok(Key::A),
            Keycode::B => Ok(Key::B),
            Keycode::C => Ok(Key::C),
            Keycode::D => Ok(Key::D),
            Keycode::E => Ok(Key::E),
            Keycode::F => Ok(Key::F),
            _ => Err(()),
        }
    }
}

impl KeyState {
    pub fn is_set(&self, index: Key) -> bool {
        (self.0 & index.selector()) != 0
    }

    pub fn set(&mut self, index: Key) {
        self.0 |= index.selector();
    }

    pub fn unset(&mut self, index: Key) {
        self.0 &= !index.selector()
    }
}

impl From<u8> for Key {
    fn from(val: u8) -> Self {
        match val {
            0x0 => Self::Zero,
            0x1 => Self::One,
            0x2 => Self::Two,
            0x3 => Self::Three,
            0x4 => Self::Four,
            0x5 => Self::Five,
            0x6 => Self::Six,
            0x7 => Self::Seven,
            0x8 => Self::Eight,
            0x9 => Self::Nine,
            0xA => Self::A,
            0xB => Self::B,
            0xC => Self::C,
            0xD => Self::D,
            0xE => Self::E,
            0xF => Self::F,
            _ => panic!("Invalid key"),
        }
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
            assert!(keystate.is_set(k));
        }
    }

    #[test]
    fn clearing_a_works() {
        let mut keystate = KeyState(0xFF);
        for k in VALUES {
            keystate.unset(k);
            assert!(!keystate.is_set(k));
        }
    }
}
