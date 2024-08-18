#![allow(unused)]

struct Solidity {}
struct Vyper {}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        String::from(format!("solc {}", file_path))
    }

}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        String::from(format!("vyper {}", file_path))
    }
}

fn compile(lang: &impl Compiler, file_path: &str) {
    println!("{}", lang.compile(file_path));
}

// Trait bound syntax

fn main() {
    let sol = Solidity {};
    let vy = Vyper {};

    compile(&sol, "Hello.sol");
    compile(&vy, "Hello.vy");
}
