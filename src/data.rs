use std::collections::HashMap;
use std::io::prelude::*;
use std::ops::Add;

/// Data that can be stored in the data segments
pub enum DataELInner {
    /// 1-byte value
    Byte(i8),
    /// 2-byte value
    Word(i16),
    /// 4-byte value
    Long(i32),
    /// 8-byte value
    Quad(i64),
    /// writes n zeros
    Space(usize),
    /// Stores addresses
    Address(super::reg::Label),
    /// Stores string (finished by 0)
    String(String),

    /// Label
    Label(super::reg::Label),
}

impl DataELInner {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Self::Byte(el) => {
                file.write_all(b"\t.byte ")?;
                file.write_all(format!("{}", el).as_bytes())
            }
            Self::Word(el) => {
                file.write_all(b"\t.word")?;
                file.write_all(format!("{}", el).as_bytes())
            }
            Self::Long(el) => {
                file.write_all(b"\t.long")?;
                file.write_all(format!("{}", el).as_bytes())
            }
            Self::Quad(el) => {
                file.write_all(b"\t.quad")?;
                file.write_all(format!("{}", el).as_bytes())
            }
            Self::Address(el) => {
                file.write_all(b"\t.quad ")?;
                el.write_in(file)
            }
            Self::String(str) => {
                file.write_all(b"\t.string \"")?;
                file.write_all(str.as_bytes())?;
                file.write_all(b"\"")
            }
            Self::Space(i) => file.write_all(format!("\t.space {}", i).as_bytes()),
            Self::Label(el) => {
                el.write_in(file)?;
                file.write_all(b":")
            }
        }
    }

    fn to_data_el(self) -> DataEL {
        DataEL {
            info: self,
            comment: None,
        }
    }
}

/// Structure representing a single element of the data segment
pub struct DataEL {
    info: DataELInner,
    comment: Option<String>,
}

impl DataEL {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        self.info.write_in(file)?;
        match &self.comment {
            None => (),
            Some(str) => {
                file.write_all(b" ##")?;
                file.write_all(str.as_bytes())?;
            }
        }
        file.write_all(b"\n")
    }

    /// Add a comment at the end of the corresponding line
    pub fn add_comment(&mut self, comment: String) {
        match &mut self.comment {
            None => self.comment = Some(comment),
            Some(str) => {
                str.push_str(" ## ");
                str.push_str(&comment);
            }
        }
    }
}

/// Structure representing the data segment
pub struct Data {
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
                    DataELInner::String(s1).to_data_el(),
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
}

impl Add for Data {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut infos = self.infos;
        let mut infos2 = other.infos;
        infos.append(&mut infos2);
        Self { infos }
    }
}

/// Add a label in data area
pub fn label(name: crate::reg::Label) -> Data {
    Data::new(DataELInner::Label(name).to_data_el())
}

/// Place a constant string (end with 0) in data area
pub fn dstring(data: String) -> Data {
    Data::new(DataELInner::String(data).to_data_el())
}

/// Place a list 1 bytes values in data area
pub fn dbyte(i: i8) -> Data {
    Data::new(DataELInner::Byte(i).to_data_el())
}

/// Place a list 2 bytes values in data area
pub fn dword(i: i16) -> Data {
    Data::new(DataELInner::Word(i).to_data_el())
}

/// Place a list 4 bytes values in data area
pub fn dlong(i: i32) -> Data {
    Data::new(DataELInner::Long(i).to_data_el())
}

/// Place a list 8 bytes values in data area
pub fn dquad(i: i64) -> Data {
    Data::new(DataELInner::Quad(i).to_data_el())
}

/// Place a list of addresses in the data area
pub fn daddress(addr: super::reg::Label) -> Data {
    Data::new(DataELInner::Address(addr).to_data_el())
}

/// Allocate n bytes (valued to 0) in the data segment
pub fn space(i: usize) -> Data {
    Data::new(DataELInner::Space(i).to_data_el())
}
