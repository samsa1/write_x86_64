use crate::reg::Sizes;

/// Trait representing registers (used by Operand<R>)
pub trait Reg {
    /// Write register in file
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()>;

    /// Register size
    const SIZE: Sizes;
}

/// Trait for structures that can be written
pub trait Writable {
    /// Write structure in the file
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()>;
}
