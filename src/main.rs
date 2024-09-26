use std::process::Command;

fn main() {
    let output = Command::new("i686-elf-ld")
        .args(&["-T", "linker.ld", "-g", "-r", ".build/kernel.asm.o"])
        .output()
        .expect("Failed to execute linker");

    if !output.status.success() {
        panic!("Linker error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
