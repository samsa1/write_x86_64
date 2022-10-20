use std::collections::HashMap;
use std::io::prelude::*;
use std::ops::Add;

/// Data that can be stored in the data segments
pub enum DataEL {
    /// 1-byte value
    Byte(Vec<i8>),
    /// 2-byte value
    Word(Vec<i16>),
    /// 4-byte value
    Long(Vec<i32>),
    /// 8-byte value
    Quad(Vec<i64>),
    /// writes n zeros
    Space(usize),
    /// Stores addresses
    Address(Vec<super::reg::Label>),
    /// Stores string (finished by 0)
    String(String),
}

impl DataEL {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Self::Byte(vec) => {
                file.write_all(b"\t.byte")?;
                let mut vec = vec.iter();
                match vec.next() {
                    None => (),
                    Some(el) => file.write_all(format!(" {}", el).as_bytes())?,
                }
                for el in vec {
                    file.write_all(format!(", {}", el).as_bytes())?;
                }
                file.write_all(b"\n")
            }
            Self::Word(vec) => {
                file.write_all(b"\t.word")?;
                let mut vec = vec.iter();
                match vec.next() {
                    None => (),
                    Some(el) => file.write_all(format!(" {}", el).as_bytes())?,
                }
                for el in vec {
                    file.write_all(format!(", {}", el).as_bytes())?;
                }
                file.write_all(b"\n")
            }
            Self::Long(vec) => {
                file.write_all(b"\t.long")?;
                let mut vec = vec.iter();
                match vec.next() {
                    None => (),
                    Some(el) => file.write_all(format!(" {}", el).as_bytes())?,
                }
                for el in vec {
                    file.write_all(format!(", {}", el).as_bytes())?;
                }
                file.write_all(b"\n")
            }
            Self::Quad(vec) => {
                file.write_all(b"\t.quad")?;
                let mut vec = vec.iter();
                match vec.next() {
                    None => (),
                    Some(el) => file.write_all(format!(" {}", el).as_bytes())?,
                }
                for el in vec {
                    file.write_all(format!(", {}", el).as_bytes())?;
                }
                file.write_all(b"\n")
            }
            Self::Address(vec) => {
                file.write_all(b"\t.quad ")?;
                let mut vec = vec.iter();
                match vec.next() {
                    None => (),
                    Some(el) => el.write_in(file)?,
                }
                for el in vec {
                    file.write_all(b", ")?;
                    el.write_in(file)?;
                }
                file.write_all(b"\n")
            }
            Self::String(str) => {
                file.write_all(b"\t.string \"")?;
                file.write_all(str.as_bytes())?;
                file.write_all(b"\"\n")
            }
            Self::Space(i) => file.write_all(format!("\t.space {}\n", i).as_bytes()),
        }
    }
}

/// Structure representing the data segment
pub struct Data {
    infos: Vec<(Option<String>, DataEL)>,
}

impl Data {
    /// Create an empty data segment
    pub fn empty() -> Self {
        Self { infos: Vec::new() }
    }

    /// Create new chunck of data segment with optionally a label
    /// and values to put with it
    pub fn new(name: Option<String>, data: DataEL) -> Self {
        Self {
            infos: vec![(name, data)],
        }
    }

    #[doc(hidden)]
    pub fn from_strings(strings: HashMap<String, String>) -> Self {
        Self {
            infos: strings
                .into_iter()
                .map(|(s1, s2)| (Some(s2), DataEL::String(s1)))
                .collect(),
        }
    }

    #[cfg(target_os = "linux")]
    /// Write data segment in file
    /// Does not add the .data at the beginning!
    pub fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        for (name, data) in &self.infos {
            match name {
                None => (),
                Some(name) => {
                    file.write_all(name.as_bytes())?;
                    file.write_all(b":\n")?;
                }
            }
            data.write_in(file)?;
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    /// Write data segment in file
    /// Does not add the .data at the beginning!
    pub fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        for (name, data) in &self.infos {
            match name {
                None => (),
                Some(name) => {
                    file.write_all(b"_")?;
                    file.write_all(name.as_bytes())?;
                    file.write_all(b":\n")?;
                }
            }
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

/// Place a constant string (end with 0) in data area
pub fn dstring(name: String, data: String) -> Data {
    Data::new(Some(name), DataEL::String(data))
}

/// Place a list 1 bytes values in data area
pub fn dbyte(name: String, i: Vec<i8>) -> Data {
    Data::new(Some(name), DataEL::Byte(i))
}

/// Place a list 2 bytes values in data area
pub fn dword(name: String, i: Vec<i16>) -> Data {
    Data::new(Some(name), DataEL::Word(i))
}

/// Place a list 4 bytes values in data area
pub fn dlong(name: String, i: Vec<i32>) -> Data {
    Data::new(Some(name), DataEL::Long(i))
}

/// Place a list 8 bytes values in data area
pub fn dquad(name: String, i: Vec<i64>) -> Data {
    Data::new(Some(name), DataEL::Quad(i))
}

/// Place a list of addresses in the data area
pub fn daddress(name: String, addr: Vec<super::reg::Label>) -> Data {
    Data::new(Some(name), DataEL::Address(addr))
}

/// Allocate n bytes (valued to 0) in the data segment
pub fn space(i: usize) -> Data {
    Data::new(None, DataEL::Space(i))
}
