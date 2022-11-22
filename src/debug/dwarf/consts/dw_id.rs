///! Identifier case encoding (figure 32 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const CASE_SENSITIVE: u8 = 0x01;
pub const UP_CASE: u8 = 0x02;
pub const DOWN_CASE: u8 = 0x03;
pub const CASE_INSENSITIVE: u8 = 0x04;

pub enum DWId {
    CaseSensitive,
    UpCase,
    DownCase,
    CaseInsensitive,
}

impl DWId {
    pub fn as_u8(&self) -> u8 {
        match self {
            DWId::CaseSensitive => CASE_SENSITIVE,
            DWId::UpCase => UP_CASE,
            DWId::DownCase => DOWN_CASE,
            DWId::CaseInsensitive => CASE_INSENSITIVE,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DWId::CaseSensitive => "DW_ID_CASE_SENSITIVE",
            DWId::UpCase => "DW_ID_UP_CASE",
            DWId::DownCase => "DW_ID_DOWN_CASE",
            DWId::CaseInsensitive => "DW_ID_CASE_INSENSITIVE",
        }
    }
}
