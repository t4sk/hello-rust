#![allow(unused)]

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait Test {
    fn test(&self, file_path: &str) -> String {
        // Default implementation
        format!("test {file_path}")
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc (version: {}) {}", self.version, file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper (version: {}) {}", self.version, file_path)
    }
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

// Input lang is any type that implements the Compiler trait
fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

// Input lang is any type that implements the Test trait
fn test(lang: &impl Test, file_path: &str) -> String {
    lang.test(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };
    let vy = Vyper {
        version: "0.4".to_string(),
    };

    println!("{}", compile(&sol, "Hello.sol"));
    println!("{}", compile(&vy, "Hello.vy"));
    println!("{}", test(&sol, "Hello.sol"));
    println!("{}", test(&vy, "Hello.vy"));
}
