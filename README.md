
# write_x86_64

## Objective

This crate is written in the purpose of helping people implement a x86_64 assembly backend for a compiler in rust.
Thus this crate implements data structures to write simple x86_64 instructions but also type check those instruction.


## Usage

Generate a Hello World program for x86_64 macos
```rust
use write_x86_64::*;

fn main() {

    let globl = label(new_label("main"))

    let text_ss = 
        label(new_label("main"))
        + pushq(reg!(RBP))
        + leaq(lab!(new_label("my_string")), RDI)
        + call(reg::Label::printf())
        + leaq(lab!(new_label("my_string2")), RDI)
        + call(reg::Label::printf())
        + xorq(reg!(RAX), reg!(RAX))
        + popq(RBP)
        + ret();

    let data_ss = 
        data::dstring("my_string".to_string(),
            "Hello".to_string())
        + data::dstring("my_string2".to_string(),
            " World\n".to_string());

    let file = file::File {
        globl,
        text_ss,
        data_ss,
    };
    
    file.print_in("asm_file.s").unwrap();
}
```

## Contributing

Contribution are welcomed, you can also ask to add some
instructions if you are using this crate and would want more
instructions available.


## Project using this crate:

- A compiler for a subset of Rust : https://github.com/samsa1/SamRustCompiler
