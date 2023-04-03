use crate::reg::Sizes;
use std::fmt::Debug;

/// Trait representing registers (used by Operand<R>)
pub trait Reg: Debug + Clone {
    /// Write register in file
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()>;

    /// Convert to 3 bits (+ flag)
    fn to_bits(&self) -> (bool, u8);

    /// Register size
    const SIZE: Sizes;
}

/// Trait for structures that can be written
pub trait Writable {
    /// Write structure in the file
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()>;
}
