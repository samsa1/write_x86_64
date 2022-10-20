use super::Text;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

/// Structure representing a file
pub struct File {
    /// Code segment
    pub text_ss: Text,
    /// Data segment
    pub data_ss: super::data::Data,
}

impl File {
    #[doc(hidden)]
    pub fn new(text_ss: Text, strings: HashMap<String, String>) -> Self {
        let data_ss = super::data::Data::from_strings(strings);
        Self { text_ss, data_ss }
    }

    #[cfg(target_os = "macos")]
    fn write_globl_main(file: &mut fs::File) -> std::io::Result<()> {
        file.write_all(b"\t.globl\t_main\n")
    }

    #[cfg(target_os = "linux")]
    fn write_globl_main(file: &mut fs::File) -> std::io::Result<()> {
        file.write_all(b"\t.globl\tmain\n")
    }

    /// Method to print assembly file in given file
    pub fn print_in(self, file_name: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(file_name)?;
        file.write_all(b"\t.text\n")?;
        Self::write_globl_main(&mut file)?;
        self.text_ss.write_in(&mut file)?;
        file.write_all(b"\t.data\n")?;

        self.data_ss.write_in(&mut file)
    }
}
