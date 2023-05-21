mod mipsi;

pub use mipsi::MipsI;
// TODO: Improve error (maybe use thiserror crate)
// TODO: Implement DEBUG or Display
// TODO: FP(CP1) specific opcode
// Improve: Tables should be Result<Option<>, E> so that if instructions that should already been
// parsed error

pub trait Decode<T> {
    type Error;

    fn decode(&self) -> Result<T, Self::Error>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DecodeError {
    #[error("cannot decode u32 {:x}", u32)]
    CannotDecodeU8 { u32: u32 },
}

#[derive(Debug, PartialEq)]
pub struct Register {
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub sa: u8,
}

#[derive(Debug, PartialEq)]
pub struct Immediate {
    pub rs: u8,
    pub rt: u8,
    pub immediate: u16,
}

#[derive(Debug, PartialEq)]
pub struct Jump {
    pub target: u32,
}
