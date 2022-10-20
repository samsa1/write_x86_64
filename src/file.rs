use super::Asm;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;

/// Structure representing a file
pub struct File {
    /// Code segment
    pub code_ss: Asm,
    /// Data segment
    pub data_ss: super::data::Data,
}

impl File {
    #[doc(hidden)]
    pub fn new(code_ss: Asm, strings: HashMap<String, String>) -> Self {
        let data_ss = super::data::Data::from_strings(strings);
        Self { code_ss, data_ss }
    }

    #[cfg(target_os = "macos")]
    fn write_globl_main(file : &mut fs::File) -> std::io::Result<()> {
        file.write_all(b"\t.globl\t_main\n")
    }

    #[cfg(target_os = "linux")]
    fn write_globl_main(file : &mut fs::File) -> std::io::Result<()> {
        file.write_all(b"\t.globl\tmain\n")
    }

    /// Method to print assembly file in given file
    pub fn print_in(self, file_name: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(file_name)?;
        file.write_all(b"\t.text\n")?;
        Self::write_globl_main(&mut file)?;
        self.code_ss.write_in(&mut file)?;
        file.write_all(b"\t.data\n")?;

        self.data_ss.write_in(&mut file)?;

        file.write_all(b"_my_string:\n\t.string \"%zd\\n\"\n")
    }
}