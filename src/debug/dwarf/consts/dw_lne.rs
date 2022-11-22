///! Line Number Extended Opcode encoding (figure 38 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const END_SEQUENCE: u8 = 0x01;
pub const SET_ADDRESS: u8 = 0x02;
pub const DEFINE_FILE: u8 = 0x03;
pub const SET_DISCRIMINATOR: u8 = 0x04;

pub enum LNEO {
    EndSequence,
    SetAddress,
    DefineFile,
    SetDisciminator,
}

impl LNEO {
    pub fn as_u8(&self) -> u8 {
        match self {
            LNEO::EndSequence => END_SEQUENCE,
            LNEO::SetAddress => SET_ADDRESS,
            LNEO::DefineFile => DEFINE_FILE,
            LNEO::SetDisciminator => SET_DISCRIMINATOR,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LNEO::EndSequence => "DW_LNE_END_SEQUENCE",
            LNEO::SetAddress => "DW_LNE_SET_ADDRESS",
            LNEO::DefineFile => "DW_LNE_DEFINE_FILE",
            LNEO::SetDisciminator => "DW_LNE_SET_DISCRIMINATOR",
        }
    }
}
