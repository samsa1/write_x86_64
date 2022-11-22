use crate::traits::Writable;
use crate::Data;
use std::io::prelude::*;

/// Data that can be stored in the data segments
pub enum DataEL {
    /// 1-byte value
    Byte(i8),
    /// 1-byte usingned value
    ByteU(u8),
    /// 2-byte value
    Word(i16),
    /// 2-byte usingned value
    ShortU(u16),
    /// 4-byte value
    Long(i32),
    /// 4-byte value
    LongU(u32),
    /// 8-byte value
    Quad(i64),
    /// writes n zeros
    Space(usize),
    /// Stores addresses 4 bytes
    AddressLong(super::reg::Label),
    /// Stores addresses 8 bytes
    AddressQuad(super::reg::Label),
    /// Stores string not terminated by zero
    Ascii(String),
    /// Asciz 0 terminated string
    Asciz(String),
}

impl Writable for DataEL {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Self::Byte(el) => {
                file.write_all(b"\t.byte  ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::ByteU(el) => {
                file.write_all(b"\t.byte  ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::Word(el) => {
                file.write_all(b"\t.word  ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::ShortU(el) => {
                file.write_all(b"\t.short ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::Long(el) => {
                file.write_all(b"\t.long  ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::LongU(el) => {
                file.write_all(b"\t.long  ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::Quad(el) => {
                file.write_all(b"\t.quad  ")?;
                file.write_all(format!("{:<10}", el).as_bytes())
            }
            Self::AddressLong(el) => {
                file.write_all(b"\t.long  ")?;
                el.write_in(file)
            }
            Self::AddressQuad(el) => {
                file.write_all(b"\t.quad  ")?;
                el.write_in(file)
            }
            Self::Ascii(str) => {
                file.write_all(b"\t.ascii \"")?;
                file.write_all(str.as_bytes())?;
                file.write_all(b"\"")
            }
            Self::Asciz(str) => {
                file.write_all(b"\t.asciz \"")?;
                file.write_all(str.as_bytes())?;
                file.write_all(b"\"")
            }
            Self::Space(i) => file.write_all(format!("\t.space {}", i).as_bytes()),
        }
    }

    // fn to_data_el(self) -> DataEL {
    //     DataEL {
    //         info: self,
    //         comment: None,
    //     }
    // }
}

/// Structure representing a single element of the data segment
// pub struct DataEL {
//     info: DataELInner,
//     comment: Option<String>,
// }

// impl DataEL {
//     /// Add a comment at the end of the corresponding line
//     pub fn add_comment(&mut self, comment: String) {
//         match &mut self.comment {
//             None => self.comment = Some(comment),
//             Some(str) => {
//                 str.push_str(" ## ");
//                 str.push_str(&comment);
//             }
//         }
//     }
// }

// impl Writable for DataEL {
//     fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
//         self.info.write_in(file)?;
//         match &self.comment {
//             None => (),
//             Some(str) => {
//                 file.write_all(b" ##")?;
//                 file.write_all(str.as_bytes())?;
//             }
//         }
//         file.write_all(b"\n")
//     }
// }

/// Structure representing the data segment
/*pub struct Data {
    infos: Vec<DataEL>,
}

impl Data {
    /// Create an empty data segment
    pub fn empty() -> Self {
        Self { infos: Vec::new() }
    }

    /// Create new chunck of data segment with optionally a label
    /// and values to put with it
    pub fn new(data: DataEL) -> Self {
        Self { infos: vec![data] }
    }

    #[doc(hidden)]
    pub fn from_strings(strings: HashMap<String, String>) -> Self {
        let info: Vec<Vec<_>> = strings
            .into_iter()
            .map(|(s1, s2)| {
                vec![
                    DataELInner::Label(super::reg::Label::from_str(s2)).to_data_el(),
                    DataELInner::Asciz(s1).to_data_el(),
                ]
            })
            .collect();
        Self {
            infos: info.into_iter().flatten().collect(),
        }
    }

    /// Write data segment in file
    /// Does not add the .data at the beginning!
    pub fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        for data in &self.infos {
            data.write_in(file)?;
        }
        Ok(())
    }

    /// Add a comment to last element on the data segment
    pub fn comment(mut self, str : String) -> Self {
        if self.infos.is_empty() {
            panic!("Cannot comment empty segment")
        } else {
            self.infos.last_mut().unwrap().add_comment(str);
            self
        }
    }

    /// Add a comment to last element on the data segment
    pub fn add_comment(&mut self, str : String) {
        if self.infos.is_empty() {
            panic!("Cannot comment empty segment")
        } else {
            self.infos.last_mut().unwrap().add_comment(str);
        }
    }
}
*/

/// Place a constant string (end with 0) in data area
pub fn dasciz(data: String) -> Data {
    Data::new(DataEL::Asciz(data))
}
/// Place a constant string (does not end with 0) in data area
pub fn dascii(data: String) -> Data {
    Data::new(DataEL::Ascii(data))
}

/// Place a 1 byte value in data area
pub fn dbyte(i: i8) -> Data {
    Data::new(DataEL::Byte(i))
}
/// Place an unsigned 1 byte value in data area
pub fn dubyte(i: u8) -> Data {
    Data::new(DataEL::ByteU(i))
}

/// Place a 2 bytes value in data area
pub fn dword(i: i16) -> Data {
    Data::new(DataEL::Word(i))
}

/// Place an unsigned 2 bytes value in data area
pub fn dushort(i: u16) -> Data {
    Data::new(DataEL::ShortU(i))
}

/// Place a 4 bytes value in data area
pub fn dlong(i: i32) -> Data {
    Data::new(DataEL::Long(i))
}

/// Place an unsigned 4 bytes value in data area
pub fn dulong(i: u32) -> Data {
    Data::new(DataEL::LongU(i))
}

/// Place a 8 bytes value in data area
pub fn dquad(i: i64) -> Data {
    Data::new(DataEL::Quad(i))
}

/// Place of addresse in the data area
pub fn daddress(addr: super::reg::Label) -> Data {
    Data::new(DataEL::AddressQuad(addr))
}

/// Place of addresse in the data area
pub fn dlong_label(addr: super::reg::Label) -> Data {
    Data::new(DataEL::AddressLong(addr))
}

/// Allocate n bytes (valued to 0) in the data segment
pub fn space(i: usize) -> Data {
    Data::new(DataEL::Space(i))
}
