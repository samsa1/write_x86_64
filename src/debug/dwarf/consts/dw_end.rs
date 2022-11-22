///! Endianity sign encoding (figure 26 [https://dwarfstd.org/doc/DWARF4.pdf])7

pub const DEFAULT: u8 = 0x00;
pub const BIG: u8 = 0x01;
pub const LITTLE: u8 = 0x02;

pub enum Endianity {
    Default,
    Big,
    Little,
}

impl Endianity {
    pub fn as_u8(&self) -> u8 {
        match self {
            Endianity::Default => DEFAULT,
            Endianity::Big => BIG,
            Endianity::Little => LITTLE,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Endianity::Default => "DW_END_DEFAULT",
            Endianity::Big => "DW_END_BIG",
            Endianity::Little => "DW_END_LITTLE",
        }
    }
}
