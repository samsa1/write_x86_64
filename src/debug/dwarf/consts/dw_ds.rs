///! Decimal sign encoding (figure 26 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const UNSIGNED: u8 = 0x01;
pub const LEADING_OVERPUNCH: u8 = 0x02;
pub const TRAILING_OVERPUNCH: u8 = 0x03;
pub const LEADING_SEPARATE: u8 = 0x04;
pub const TRAILING_SEPARATE: u8 = 0x05;

pub enum DecSign {
    Unsigned,
    LeadingOverpunch,
    TrailingOverpunch,
    LeadingSeparate,
    TrailingSeparate,
}

impl DecSign {
    pub fn as_u8(&self) -> u8 {
        match self {
            DecSign::Unsigned => UNSIGNED,
            DecSign::LeadingOverpunch => LEADING_OVERPUNCH,
            DecSign::TrailingOverpunch => TRAILING_OVERPUNCH,
            DecSign::LeadingSeparate => LEADING_SEPARATE,
            DecSign::TrailingSeparate => TRAILING_SEPARATE,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DecSign::Unsigned => "DW_DS_UNSIGNED",
            DecSign::LeadingOverpunch => "DW_DS_LEADING_OVERPUNCH",
            DecSign::TrailingOverpunch => "DW_DS_TRAILING_OVERPUNCH",
            DecSign::LeadingSeparate => "DW_DS_LEADING_SEPARATE",
            DecSign::TrailingSeparate => "DW_DS_TRAILING_SEPARATE",
        }
    }
}
