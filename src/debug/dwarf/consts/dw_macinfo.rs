///! Macinfo Type encoding (figure 38 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const DEFINE: u8 = 0x01;
pub const UNDEF: u8 = 0x02;
pub const START_FILE: u8 = 0x03;
pub const END_FILE: u8 = 0x04;
pub const VENDOR_EXT: u8 = 0xff;

pub enum MacInfo {
    Define,
    Undef,
    StartFile,
    EndFile,
    VendorExt,
}

impl MacInfo {
    pub fn as_u8(&self) -> u8 {
        match self {
            MacInfo::Define => DEFINE,
            MacInfo::Undef => UNDEF,
            MacInfo::StartFile => START_FILE,
            MacInfo::EndFile => END_FILE,
            MacInfo::VendorExt => VENDOR_EXT,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MacInfo::Define => "DW_MACINFO_DEFINE",
            MacInfo::Undef => "DW_MACINFO_UNDEF",
            MacInfo::StartFile => "DW_MACINFO_START_FILE",
            MacInfo::EndFile => "DW_MACINFO_END_FILE",
            MacInfo::VendorExt => "DW_MACINFO_VENDOR_EXT",
        }
    }
}
