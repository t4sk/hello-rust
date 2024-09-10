use std::env;

use create::{calc_contract_addr, parse_args};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (deployer_addr, start, end) = parse_args(args);

    println!("Deployer: {:?}", deployer_addr);
    println!("nonce | contract address");

    for i in start..(end + 1) {
        let contract_addr = calc_contract_addr(deployer_addr, i);
        println!("{}, {:?}", i, contract_addr);
    }
}
