use std::io::Write;

use crate::{reg::Label, traits::Writable};

/// Expressions are not recursive, use .set multiple times to build a recursive expression
pub enum Expr {
    /// Substract both labels
    Sub(Label, Label),
    /// Add both labels
    Add(Label, Label),
    /// Unsigned constant
    UConst(usize),
    /// Signed constant
    SConst(isize),
}

impl Writable for Expr {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Expr::Sub(lab1, lab2) => {
                lab1.write_in(file)?;
                file.write_all(b"-")?;
                lab2.write_in(file)
            }
            Expr::Add(lab1, lab2) => {
                lab1.write_in(file)?;
                file.write_all(b"+")?;
                lab2.write_in(file)
            }
            Expr::UConst(c) => file.write_all(format!("{c}").as_bytes()),
            Expr::SConst(c) => file.write_all(format!("{c}").as_bytes()),
        }
    }
}
