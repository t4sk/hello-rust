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

fn main() {
    let tool = Hardhat {};
    compile(&tool);

    let tool = Foundry {};
    compile(&tool);
}
