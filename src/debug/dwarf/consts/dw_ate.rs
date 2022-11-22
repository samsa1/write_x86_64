///! Base type encoding (figure 25 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const ADDRESS: u8 = 0x01;
pub const BOOLEAN: u8 = 0x02;
pub const COMPLEX_FLOAT: u8 = 0x03;
pub const FLOAT: u8 = 0x04;
pub const SIGNED: u8 = 0x05;
pub const SIGNED_CHAR: u8 = 0x06;
pub const UNSIGNED: u8 = 0x07;
pub const UNSIGNED_CHAR: u8 = 0x08;
pub const IMAGINARY_FLOAT: u8 = 0x09;
pub const PACKED_DECIMAL: u8 = 0x0a;
pub const NUMERIC_STRING: u8 = 0x0b;
pub const EDITED: u8 = 0x0c;
pub const SIGNED_FIXED: u8 = 0x0d;
pub const UNSIGNED_FIXED: u8 = 0x0e;
pub const DECIMAL_FLOAT: u8 = 0x0f;
pub const UTF: u8 = 0x10;

pub enum BaseType {
    Address,
    Boolean,
    ComplexFloat,
    Float,
    Signed,
    SignedChar,
    Unsigned,
    UnsignedChar,
    ImaginatyFloat,
    PackedDecimal,
    NumericString,
    Edited,
    SignedFixed,
    UnsignedFixed,
    DecimalFloat,
    UTF,
}

impl BaseType {
    pub fn as_u8(&self) -> u8 {
        match self {
            BaseType::Address => ADDRESS,
            BaseType::Boolean => BOOLEAN,
            BaseType::ComplexFloat => COMPLEX_FLOAT,
            BaseType::Float => FLOAT,
            BaseType::Signed => SIGNED,
            BaseType::SignedChar => SIGNED_CHAR,
            BaseType::Unsigned => UNSIGNED,
            BaseType::UnsignedChar => UNSIGNED_CHAR,
            BaseType::ImaginatyFloat => IMAGINARY_FLOAT,
            BaseType::PackedDecimal => PACKED_DECIMAL,
            BaseType::NumericString => NUMERIC_STRING,
            BaseType::Edited => EDITED,
            BaseType::SignedFixed => SIGNED_FIXED,
            BaseType::UnsignedFixed => UNSIGNED_FIXED,
            BaseType::DecimalFloat => DECIMAL_FLOAT,
            BaseType::UTF => UTF,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BaseType::Address => "DW_ATE_ADDRESS",
            BaseType::Boolean => "DW_ATE_BOOLEAN",
            BaseType::ComplexFloat => "DW_ATE_COMPLEX_FLOAT",
            BaseType::Float => "DW_ATE_FLOAT",
            BaseType::Signed => "DW_ATE_SIGNED",
            BaseType::SignedChar => "DW_ATE_SIGNED_CHAR",
            BaseType::Unsigned => "DW_ATE_UNSIGNED",
            BaseType::UnsignedChar => "DW_ATE_UNSIGNED_CHAR",
            BaseType::ImaginatyFloat => "DW_ATE_IMAGINARY_FLOAT",
            BaseType::PackedDecimal => "DW_ATE_PACKED_DECIMAL",
            BaseType::NumericString => "DW_ATE_NUMERIC_STRING",
            BaseType::Edited => "DW_ATE_EDITED",
            BaseType::SignedFixed => "DW_ATE_SIGNED_FIXED",
            BaseType::UnsignedFixed => "DW_ATE_UNSIGNED_FIXED",
            BaseType::DecimalFloat => "DW_ATE_DECIMAL_FLOAT",
            BaseType::UTF => "DW_ATE_UTF",
        }
    }
}
