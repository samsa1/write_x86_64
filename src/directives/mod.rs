use std::io::Write;

use crate::{reg::Label, traits::Writable};

/// Define expressions (for .set)
pub mod expr;

/// See [https://sourceware.org/binutils/docs-2.18/as/LNS-directives.html#LNS-directives]
#[allow(missing_docs)]
pub enum LocOptions {
    BasicBloc,
    PrologueEnd,
    EpilogueBegin,
    IsStmt(bool),
    Isa(usize),
}

impl Writable for LocOptions {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Self::BasicBloc => file.write_all(b"basic_block"),
            Self::PrologueEnd => file.write_all(b"prologue_end"),
            Self::EpilogueBegin => file.write_all(b"epilogue_begin"),
            Self::IsStmt(b) => {
                file.write_all(format!("is_smt {}", if *b { 1 } else { 0 }).as_bytes())
            }
            Self::Isa(i) => file.write_all(format!("isa {}", i).as_bytes()),
        }
    }
}

/// Defines various directives
pub enum Directive {
    /// .p2align
    P2Align(usize, Option<usize>, Option<usize>),

    /// .file [https://sourceware.org/binutils/docs-2.18/as/LNS-directives.html#LNS-directives]
    File(usize, String),
    /// .loc [https://sourceware.org/binutils/docs-2.18/as/LNS-directives.html#LNS-directives]
    Loc(usize, usize, Option<usize>, Vec<LocOptions>),
    /// .loc_marks_blocks [https://sourceware.org/binutils/docs-2.18/as/LNS-directives.html#LNS-directives]
    LocMarkBlocks(bool),

    /// .set [https://sourceware.org/binutils/docs-2.18/as/Set.html#Set]
    Set(Label, expr::Expr),
}

impl Writable for Directive {
    fn write_in(&self, file: &mut std::fs::File) -> std::io::Result<()> {
        match self {
            Directive::P2Align(i1, opt1, opt2) => {
                file.write_all(format!(".p2align {:#x}", i1).as_bytes())?;
                if opt1.is_some() || opt2.is_some() {
                    file.write_all(b",")?;
                }
                if let Some(i) = opt1 {
                    file.write_all(format!("{}", i).as_bytes())?;
                }
                if let Some(i) = opt2 {
                    file.write_all(format!(",{}", i).as_bytes())?;
                }
            }
            Directive::File(id, name) => {
                file.write_all(format!(".file {} \"", id).as_bytes())?;
                file.write_all(name.as_bytes())?;
                file.write_all(b"\"")?;
            }
            Directive::Loc(file_id, line, column, options) => {
                file.write_all(format!(".loc {} {}", file_id, line).as_bytes())?;
                if let Some(i) = column {
                    file.write_all(format!(" {}", i).as_bytes())?;
                }
                for opt in options {
                    file.write_all(b" ")?;
                    opt.write_in(file)?;
                }
            }
            Directive::LocMarkBlocks(b) => {
                file.write_all(format!(".loc_mark_blocks {}", if *b { 1 } else { 0 }).as_bytes())?;
            }

            Directive::Set(lab, expr) => {
                file.write_all(b".set ")?;
                lab.write_in(file)?;
                file.write_all(b", ")?;
                expr.write_in(file)?;
            }
        }
        std::io::Result::Ok(())
    }
}

/// .set lab1 lab2-lab3  directive
pub fn set_sub(lab1: Label, lab2: Label, lab3: Label) -> Directive {
    Directive::Set(lab1, expr::Expr::Sub(lab2, lab3))
}
