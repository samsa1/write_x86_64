///! Line Number Standart Opcode encoding (figure 37 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const COPY: u8 = 0x01;
pub const ADVANCE_PC: u8 = 0x02;
pub const ADVANCE_LINE: u8 = 0x03;
pub const SET_FILE: u8 = 0x04;
pub const SET_COLUMN: u8 = 0x05;
pub const NEGATE_STMT: u8 = 0x06;
pub const SET_BASIC_BLOCK: u8 = 0x07;
pub const CONST_ADD_PC: u8 = 0x08;
pub const FIXED_ADVANCE_PC: u8 = 0x09;
pub const SET_PROLOGUE_END: u8 = 0x0a;
pub const SET_EPILOGUE_BEGIN: u8 = 0x0b;
pub const SET_ISA: u8 = 0x0c;

pub enum LNSO {
    Copy,
    AdvancePC,
    AdvanceLine,
    SetFile,
    SetColumn,
    NegateStmt,
    SetBasicBlock,
    ConstAddPC,
    FixedAdvancePC,
    SetPrologueEnd,
    SetEpilogueBegin,
    SetISA,
}

impl LNSO {
    pub fn as_u8(&self) -> u8 {
        match self {
            LNSO::Copy => COPY,
            LNSO::AdvancePC => ADVANCE_PC,
            LNSO::AdvanceLine => ADVANCE_LINE,
            LNSO::SetFile => SET_FILE,
            LNSO::SetColumn => SET_COLUMN,
            LNSO::NegateStmt => NEGATE_STMT,
            LNSO::SetBasicBlock => SET_BASIC_BLOCK,
            LNSO::ConstAddPC => CONST_ADD_PC,
            LNSO::FixedAdvancePC => FIXED_ADVANCE_PC,
            LNSO::SetPrologueEnd => SET_PROLOGUE_END,
            LNSO::SetEpilogueBegin => SET_EPILOGUE_BEGIN,
            LNSO::SetISA => SET_ISA,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LNSO::Copy => "DW_LNS_COPY",
            LNSO::AdvancePC => "DW_LNS_ADVANCE_PC",
            LNSO::AdvanceLine => "DW_LNS_ADVANCE_LINE",
            LNSO::SetFile => "DW_LNS_SET_FILE",
            LNSO::SetColumn => "DW_LNS_SET_COLUMN",
            LNSO::NegateStmt => "DW_LNS_NEGATE_STMT",
            LNSO::SetBasicBlock => "DW_LNS_SET_BASIC_BLOCK",
            LNSO::ConstAddPC => "DW_LNS_CONST_ADD_PC",
            LNSO::FixedAdvancePC => "DW_LNS_FIXED_ADVANCE_PC",
            LNSO::SetPrologueEnd => "DW_LNS_SET_PROLOGUE_END",
            LNSO::SetEpilogueBegin => "DW_LNS_SET_EPILOGUE_BEGIN",
            LNSO::SetISA => "DW_LNS_SET_ISA",
        }
    }
}
