use std::env;
use ethers::core::abi::{encode, encode_packed, Token};
use ethers::types::{Address, U256};
use ethers::utils::{hex, keccak256};

pub fn calc_create2_addr(deployer: Address, salt: U256, init_code: &Vec<u8>) -> Address {
    // Encode uit256 into bytes32
    let mut bytes32 = [0u8; 32];
    salt.to_big_endian(&mut bytes32);

    let init_code_hash = keccak256(encode_packed(&[Token::Bytes(init_code.to_vec())]).unwrap());

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

// TODO: compare with get_create2_address
fn main() {
    let args: Vec<String> = env::args().collect();

    let deployer: Address = args[1].parse().unwrap();
    let init_code = hex::decode(&args[2]).unwrap();

    let start: U256 = U256::from_dec_str(&args[3]).unwrap();
    let end: U256 = U256::from_dec_str(&args[4]).unwrap();

    let mut i = start;
    while i < end {
        let addr = calc_create2_addr(deployer, i, &init_code);
        println!("{:?}, {:?}", i, addr);
        i = i.checked_add(U256::one()).unwrap();
    }
}
