#![allow(unused)]

struct Solidity {}
struct Vyper {}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {}", file_path)
    }
}

trait Test {
    fn test(&self, file_path: &str) -> String;
}

impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("forge test {}", file_path)
    }
}

impl Test for Vyper {
    fn test(&self, file_path: &str) -> String {
        format!("hardhat {}", file_path)
    }
}

fn compile(lang: &impl Compiler, file_path: &str) {
    println!("{}", lang.compile(file_path));
}

// Multiple trait bound + where
fn test<T>(lang: &T, file_path: &str)
where
    T: Compiler + Test,
{
    println!("compile {}", lang.compile(file_path));
    println!("test {}", lang.test(file_path));
}

// Trait bound syntax

fn main() {
    let sol = Solidity {};
    let vy = Vyper {};

    test(&sol, "Hello.sol");
    test(&vy, "Hello.vy");
}
