// Compile: $ rustc return_2.rs
// Generate assembly: $ rustc return_2.rs --emit=asm
use clap::Parser;

#[derive(Parser)]
struct Args {
    compiler: bool,
    exec: bool,
    optimise: bool,
    decompiler: bool,
}

fn main() {
    2
}