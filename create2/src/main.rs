use ethers::core::abi::{encode, encode_packed, Token};
use ethers::types::{Address, U256};
use ethers::utils::{hex, keccak256};

pub fn calc_create2_addr(deployer: Address, salt: U256, init_code: Vec<u8>) -> Address {
    // Encode uit256 into bytes32
    let mut bytes32 = [0u8; 32];
    salt.to_big_endian(&mut bytes32);

    let init_code_hash = keccak256(encode_packed(&[Token::Bytes(init_code)]).unwrap());

    let encoded = encode_packed(&[
        Token::FixedBytes(vec![0xff]),
        Token::Address(deployer),
        Token::FixedBytes(bytes32.to_vec()),
        Token::FixedBytes(init_code_hash.to_vec()),
    ])
    .unwrap();

    let hash = keccak256(encoded);
    Address::from_slice(&hash[12..])
}

// TODO: one and par
// TODO: user input
fn main() {
    let deployer: Address = "0x5B38Da6a701c568545dCfcB03FcB875f56beddC4"
        .parse()
        .unwrap();
    let salt: U256 = U256::from(2);
    let init_code: Vec<u8> = hex::decode("6080604052348015600f57600080fd5b5060405160208061010183398101806040528101908080518201929190602001805190602001909291905050505b600081905091905056fea2646970667358221220e69e07275da7055d3a5676c26a35cc51b68eec57be097e58c939af5e1153544464736f6c63430007060033").unwrap();

    let addr = calc_create2_addr(deployer, salt, init_code);

    println!("{:?}", addr);
}
