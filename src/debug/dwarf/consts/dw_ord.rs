///! Ordering encoding (figure 35 [https://dwarfstd.org/doc/DWARF4.pdf])

pub const ROW_MAJOR: u8 = 0x00;
pub const COL_MAJOR: u8 = 0x01;

pub enum Ordering {
    RowMajor,
    ColMajor,
}

impl Ordering {
    pub fn as_u8(&self) -> u8 {
        match self {
            Ordering::RowMajor => ROW_MAJOR,
            Ordering::ColMajor => COL_MAJOR,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Ordering::RowMajor => "DW_ORD_ROW_MAJOR",
            Ordering::ColMajor => "DW_ORD_COL_MAJOR",
        }
    }
}
