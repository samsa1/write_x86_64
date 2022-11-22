use crate::Data;

/// Defines the dwarf format
/// [https://dwarfstd.org/doc/DWARF4.pdf]
#[allow(missing_docs)]
pub mod dwarf;

/// Structure representing the output of converting a debug symbol structure to data
pub struct DebugSegments {
    /// .debug_abbrev segment
    pub debug_abbrev: Data,
    /// .debug_info segment
    pub debug_info: Data,
    /// .debug_str segment
    pub debug_str: Data,
}
