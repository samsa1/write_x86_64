///! Inline encoding (figure 34 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const NOT_INLINED: u8 = 0x00;
pub const INLINED: u8 = 0x01;
pub const DECLARED_NOT_INLINED: u8 = 0x02;
pub const DECLARED_INLINED: u8 = 0x03;

pub enum Inline {
    NotInline,
    Inlined,
    DeclaredNotInlined,
    DeclaredInlined,
}

impl Inline {
    pub fn as_u8(&self) -> u8 {
        match self {
            Inline::NotInline => NOT_INLINED,
            Inline::Inlined => INLINED,
            Inline::DeclaredNotInlined => DECLARED_NOT_INLINED,
            Inline::DeclaredInlined => DECLARED_INLINED,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Inline::NotInline => "DW_INL_NOT_INLINED",
            Inline::Inlined => "DW_INL_INLINED",
            Inline::DeclaredNotInlined => "DW_INL_DECLARED_NOT_INLINED",
            Inline::DeclaredInlined => "DW_INL_DECLARED_INLINED",
        }
    }
}
