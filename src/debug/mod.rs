/// Defines the dwarf format
pub mod dwarf;

pub trait DebugSymb {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()>;
}
