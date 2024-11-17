#![allow(unused)]

struct Hardhat {}
struct Foundry {}

trait SmartContractTool {
    fn compile(&self) -> String;
}

impl SmartContractTool for Hardhat {
    fn compile(&self) -> String {
        return "npx hardhat compile".to_string();
    }
}

impl SmartContractTool for Foundry {
    fn compile(&self) -> String {
        return "forge build".to_string();
    }
}

pub fn compile(tool: &impl SmartContractTool) {
    let cmd = tool.compile();
    println!("{}", cmd);
}

// input impl
fn func_impl_in(f: impl Fn(i32) -> i32) -> i32 {
    let x = f(1);
    x
}

// return impl
fn func_impl_out(x: i32) -> impl Fn(i32) -> i32 {
    let func = move |y: i32| x + y;
    func
}

fn main() {
    let tool = Hardhat {};
    compile(&tool);

    let tool = Foundry {};
    compile(&tool);
}
