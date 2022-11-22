///! Calling convention encoding (figure 33 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const NORMAL: u8 = 0x01;
pub const PROGRAM: u8 = 0x02;
pub const NOCALL: u8 = 0x03;

/// Information about the calling convention
pub enum DWCC {
    Normal,
    Program,

    /// Non standart
    NoCall,
}

impl DWCC {
    pub fn as_u8(&self) -> u8 {
        match self {
            DWCC::NoCall => NOCALL,
            DWCC::Normal => NORMAL,
            DWCC::Program => PROGRAM,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DWCC::NoCall => "DW_CC_NOCALL",
            DWCC::Normal => "DW_CC_NORMAL",
            DWCC::Program => "DW_CC_PROGRAM",
        }
    }
}
