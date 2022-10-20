use super::*;
use std::process::Command;

#[test]
#[cfg(target_os = "macos")]
fn hello_world() {
    let file_name = "asm_file.s";

    let text_ss = label(new_label("main"))
        + pushq(reg!(RBP))
        + leaq(lab!(new_label("my_string")), RDI)
        + call(reg::Label::printf())
        + leaq(lab!(new_label("my_string2")), RDI)
        + call(reg::Label::printf())
        + xorq(reg!(RAX), reg!(RAX))
        + popq(RBP)
        + ret();

    let data_ss = data::dstring("my_string".to_string(), "Hello".to_string())
        + data::dstring("my_string2".to_string(), " World\n".to_string());

    let file = file::File { text_ss, data_ss };

    file.print_in(file_name).unwrap();

    Command::new("gcc")
        .arg(file_name)
        .output()
        .expect("failed linking");
    std::fs::remove_file(file_name).unwrap();

    let output = Command::new("./a.out")
        .arg(file_name)
        .output()
        .expect("failed running");
    std::fs::remove_file("a.out").unwrap();
    assert_eq!(&output.stdout, b"Hello World\n");
}

#[test]
#[cfg(target_os = "linux")]
fn hello_world() {
    let file_name = "asm_file.s";
    let text_ss = label(new_label("main"))
        + pushq(reg!(RBP))
        + movq(lab!(new_label("my_string")), RDI)
        + call(reg::Label::printf())
        + xorq(reg!(RAX), reg!(RAX))
        + popq(RBP)
        + ret();

    let data_ss = data::dstring("my_string".to_string(), "Hello World\n".to_string());

    let file = file::File { text_ss, data_ss };

    file.print_in(file_name).unwrap();
    Command::new("gcc")
        .arg(file_name)
        .output()
        .expect("failed linking");
    std::fs::remove_file(file_name).unwrap();

    let output = Command::new("./a.out")
        .arg(file_name)
        .output()
        .expect("failed running");
    std::fs::remove_file("a.out").unwrap();
    assert_eq!(&output.stdout, b"Hello World\n");
}
