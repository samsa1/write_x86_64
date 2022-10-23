use super::Text;
use std::fs;
use std::io::prelude::*;

/// Structure representing a file
pub struct File {
    /// Entry point of code
    pub globl: Option<super::reg::Label>,

    /// Code segment
    pub text_ss: Text,

    /// Data segment
    pub data_ss: super::data::Data,
}

impl File {
    /// Method to print assembly file in given file
    pub fn print_in(self, file_name: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(file_name)?;
        file.write_all(b"\t.text\n")?;
        match self.globl {
            None => (),
            Some(main) => {
                file.write_all(b"\t.globl\t")?;
                main.write_in(&mut file)?;
                file.write_all(b"\n")?;
            }
        }
        self.text_ss.write_in(&mut file)?;
        file.write_all(b"\t.data\n")?;

        self.data_ss.write_in(&mut file)
    }
}
