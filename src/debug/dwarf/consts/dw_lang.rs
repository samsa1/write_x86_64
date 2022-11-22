///! Language encoding (figure 31 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const C89: u16 = 0x0001;
pub const C: u16 = 0x0002;
pub const ADA83: u16 = 0x0003;
pub const C_PLUS_PLUS: u16 = 0x0004;
pub const COBOL74: u16 = 0x0005;
pub const COBOL85: u16 = 0x0006;
pub const FORTRAN77: u16 = 0x0007;
pub const FORTRAN90: u16 = 0x0008;
pub const PASCAL83: u16 = 0x0009;
pub const MODULA2: u16 = 0x000a;
pub const JAVA: u16 = 0x000b;
pub const C99: u16 = 0x000c;
pub const ADA95: u16 = 0x000d;
pub const FORTRAN95: u16 = 0x000e;
pub const PLI: u16 = 0x000f;
pub const OBJ_C: u16 = 0x0010;
pub const OBJ_C_PLUS_PLUS: u16 = 0x0011;
pub const UPC: u16 = 0x0012;
pub const D: u16 = 0x0013;
pub const PYTHON: u16 = 0x0014;

pub enum DWLang {
    Ada83,
    Ada95,
    C,
    C89,
    C99,
    CPlusPlus,
    Cobol74,
    Cobol85,
    D,
    Fortran77,
    Fortran90,
    Fortran95,
    Java,
    Modula2,
    ObjC,
    ObjCPlusPlus,
    Pascal83,
    PLI,
    Python,
    UPC,
}

impl DWLang {
    pub fn as_u16(&self) -> u16 {
        match self {
            DWLang::Ada83 => ADA83,
            DWLang::Ada95 => ADA95,
            DWLang::C => C,
            DWLang::C89 => C89,
            DWLang::C99 => C99,
            DWLang::CPlusPlus => C_PLUS_PLUS,
            DWLang::Cobol74 => COBOL74,
            DWLang::Cobol85 => COBOL85,
            DWLang::D => D,
            DWLang::Fortran77 => FORTRAN77,
            DWLang::Fortran90 => FORTRAN90,
            DWLang::Fortran95 => FORTRAN95,
            DWLang::Java => JAVA,
            DWLang::Modula2 => MODULA2,
            DWLang::ObjC => OBJ_C,
            DWLang::ObjCPlusPlus => OBJ_C_PLUS_PLUS,
            DWLang::Pascal83 => PASCAL83,
            DWLang::PLI => PLI,
            DWLang::Python => PYTHON,
            DWLang::UPC => UPC,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DWLang::Ada83 => "DW_LANG_ADA83",
            DWLang::Ada95 => "DW_LANG_ADA95",
            DWLang::C => "DW_LANG_C",
            DWLang::C89 => "DW_LANG_C89",
            DWLang::C99 => "DW_LANG_C99",
            DWLang::CPlusPlus => "DW_LANG_C_PLUS_PLUS",
            DWLang::Cobol74 => "DW_LANG_COBOL74",
            DWLang::Cobol85 => "DW_LANG_COBOL85",
            DWLang::D => "DW_LANG_D",
            DWLang::Fortran77 => "DW_LANG_FORTRAN77",
            DWLang::Fortran90 => "DW_LANG_FORTRAN90",
            DWLang::Fortran95 => "DW_LANG_FORTRAN95",
            DWLang::Java => "DW_LANG_JAVA",
            DWLang::Modula2 => "DW_LANG_MODULA2",
            DWLang::ObjC => "DW_LANG_OBJ_C",
            DWLang::ObjCPlusPlus => "DW_LANG_OBJ_C_PLUS_PLUS",
            DWLang::Pascal83 => "DW_LANG_PASCAL83",
            DWLang::PLI => "DW_LANG_PLI",
            DWLang::Python => "DW_LANG_PYTHON",
            DWLang::UPC => "DW_LANG_UPC",
        }
    }
}
