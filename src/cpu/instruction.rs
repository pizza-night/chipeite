use std::marker::PhantomData;

pub trait Len {
    type Pred;
}

pub struct Four;
pub struct Three;
pub struct Two;
pub struct One;
pub struct Zero;

impl Len for Four {
    type Pred = Three;
}

impl Len for Three {
    type Pred = Two;
}

impl Len for Two {
    type Pred = One;
}

impl Len for One {
    type Pred = Zero;
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction<Len> {
    value: u16,
    _marker: PhantomData<Len>,
}

impl Instruction<Four> {
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self {
            value: u16::from_be_bytes(bytes),
            _marker: PhantomData,
        }
    }

    pub fn four(self) -> u16 {
        self.value
    }
}

impl<T> Instruction<T>
where
    T: Len,
{
    pub fn one(self) -> (u8, Instruction<T::Pred>) {
        let v = self.value >> (3 * 4);
        (
            v as _,
            Instruction {
                value: v << 1,
                _marker: PhantomData,
            },
        )
    }
}

impl<T> Instruction<T>
where
    T: Len,
    T::Pred: Len,
{
    pub fn two(self) -> (u8, Instruction<<T::Pred as Len>::Pred>) {
        let v = self.value >> (2 * 4);
        (
            v as _,
            Instruction {
                value: v << 2,
                _marker: PhantomData,
            },
        )
    }
}

impl<T> Instruction<T>
where
    T: Len,
    T::Pred: Len,
    <T::Pred as Len>::Pred: Len,
{
    pub fn three(self) -> (u16, Instruction<<<T::Pred as Len>::Pred as Len>::Pred>) {
        let v = self.value >> 4;
        (
            v as _,
            Instruction {
                value: v << 3,
                _marker: PhantomData,
            },
        )
    }
}
