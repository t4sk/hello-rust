#![allow(unused)]

trait Language {
    fn name(&self) -> String;
    fn run(&self, file_path: &str) -> String;
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait CompiledLanguage: Language + Compiler {
    fn exec(&self, file_path: &str) {
        let cmd = self.compile(file_path);
        println!("{cmd}");

        let cmd = self.run(file_path);
        println!("{cmd}");
    }
}

struct Rust;

impl Language for Rust {
    fn name(&self) -> String {
        "Rust".to_string()
    }

    fn run(&self, file_path: &str) -> String {
        format!("caro run {file_path}")
    }
}

impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}

impl CompiledLanguage for Rust {}

fn main() {
    let lang = Rust;
    let file_path = "hello.rs";
    lang.exec(file_path);
}
