///! Visibility encoding (figure 29 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const LOCAL: u8 = 0x01;
pub const EXPORTED: u8 = 0x02;
pub const QUALIFIED: u8 = 0x03;

pub enum Visibility {
    Local,
    Exported,
    Qualified,
}

impl Visibility {
    pub fn as_u8(&self) -> u8 {
        match self {
            Visibility::Local => LOCAL,
            Visibility::Exported => EXPORTED,
            Visibility::Qualified => QUALIFIED,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Visibility::Local => "DW_VIS_LOCAL",
            Visibility::Exported => "DW_VIS_EXPORTED",
            Visibility::Qualified => "DW_VIS_QUALIFIED",
        }
    }
}
