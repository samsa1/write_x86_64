///! Discriminant encoding (figure 35 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const LABEL: u8 = 0x00;
pub const RANGE: u8 = 0x01;

pub enum Discriminant {
    Label,
    Range,
}

impl Discriminant {
    pub fn as_u8(&self) -> u8 {
        match self {
            Discriminant::Label => LABEL,
            Discriminant::Range => RANGE,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Discriminant::Label => "DW_DSC_LABEL",
            Discriminant::Range => "DW_DSC_RANGE",
        }
    }
}
