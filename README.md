
# write_x86_64 rust crate

## Objective

This crate is written in the purpose of helping people implement a x86_64 assembly backend for a compiler in rust.
Thus, this crate implements data structures to write simple x86_64 instructions but also type check those instructions.


## Usage

Generate a Hello World program for x86_64 macOS (also works on Linux)
```rust
use write_x86_64::*;

fn main() {
    let file_name = "asm_file.s";

    let text_ss = Segment::label(new_label("main"))
        + pushq(reg!(RBP))
        + leaq(lab!(new_label("my_string")), RDI)
        + call(reg::Label::printf())
        + leaq(lab!(new_label("my_string2")), RDI)
        + call(reg::Label::printf())
        + xorq(reg!(RAX), reg!(RAX))
        + popq(RBP)
        + ret();

    let data_ss = Data::label(new_label("my_string"))
        + data::dasciz("Hello".to_string())
        + Data::label(new_label("my_string2"))
        + data::dasciz(" World\\n".to_string());

    let file = file::File {
        globl: Some(new_label("main")),
        text_ss,
        data_ss,
    };

    file.print_in(file_name).unwrap();
}
```

## Contributing

Contribution are welcomed, you can also ask to add some
instructions if you are using this crate and would want more
instructions available.

## Future work

We are currently trying to implement DWARF debug symbols.
Any contribution, testing, comment are welcomed.

## Project using this crate:

- A compiler for a subset of Rust : https://github.com/samsa1/SamRustCompiler
