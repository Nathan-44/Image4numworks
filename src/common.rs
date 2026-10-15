#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flags {
    Base = 0,
    Transparent = 1,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumImageFormat {
    Simple = 0,
}
