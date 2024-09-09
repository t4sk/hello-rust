use ethers::abi::{encode, Token};
use ethers::types::{Address, U256};
use ethers::utils::{keccak256, hex};

pub fn calc_create2_addr(sender: Address, salt: U256, init_code: Vec<u8>) -> Address {
    let encoded = encode(&[
        Token::FixedBytes(vec![0xff]),
        Token::Address(sender),
        Token::Uint(salt),
        Token::Bytes(init_code)
    ]);

    let hash = keccak256(encoded);
    Address::from_slice(&hash[12..])
}

fn main() {
    let sender: Address = "0x000000000000000000000000000000000000dead".parse().unwrap();
    let salt: U256 = 1.into();
    let init_code: Vec<u8> = hex::decode("6080604052348015600f57600080fd5b5060405160208061010183398101806040528101908080518201929190602001805190602001909291905050505b600081905091905056fea2646970667358221220e69e07275da7055d3a5676c26a35cc51b68eec57be097e58c939af5e1153544464736f6c63430007060033").unwrap();

    let addr = calc_create2_addr(sender, salt, init_code);

    println!("{:?}", addr);
}
