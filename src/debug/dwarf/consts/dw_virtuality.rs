///! Virtuality encoding (figure 30 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const NONE: u8 = 0x00;
pub const VIRTUAL: u8 = 0x01;
pub const PURE_VIRTUAL: u8 = 0x02;

pub enum Virtuality {
    None,
    Virtual,
    PureVirtual,
}

impl Virtuality {
    pub fn as_u8(&self) -> u8 {
        match self {
            Virtuality::None => NONE,
            Virtuality::Virtual => VIRTUAL,
            Virtuality::PureVirtual => PURE_VIRTUAL,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Virtuality::None => "DW_VIRTUALITY_NONE",
            Virtuality::Virtual => "DW_VIRTUALITY_VIRTUAL",
            Virtuality::PureVirtual => "DW_VIRTUALITY_PURE_VIRTUAL",
        }
    }
}
