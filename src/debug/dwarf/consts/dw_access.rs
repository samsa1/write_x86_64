///! Accessibility encoding (figure 25 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const PUBLIC: u8 = 0x01;
pub const PROTECTED: u8 = 0x02;
pub const PRIVATE: u8 = 0x03;

pub enum DWAccess {
    Public,
    Protected,
    Private,
}

impl DWAccess {
    pub fn as_u8(&self) -> u8 {
        match self {
            DWAccess::Public => PUBLIC,
            DWAccess::Protected => PROTECTED,
            DWAccess::Private => PRIVATE,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            DWAccess::Public => "DW_ACCESS_PUBLIC",
            DWAccess::Protected => "DW_ACCESS_PROTECTED",
            DWAccess::Private => "DW_ACCESS_PRIVATE",
        }
    }
}
