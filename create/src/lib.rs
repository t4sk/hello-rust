use ethers::abi::encode;
use ethers::types::Address;
use ethers::utils::keccak256;

pub fn calc_contract_addr(deployer_addr: Address, nonce: u64) -> Address {
    let encoded = encode(&[
        ethers::abi::Token::Address(deployer_addr),
        ethers::abi::Token::Uint(nonce.into()),
    ]);

    let hash = keccak256(encoded);
    Address::from_slice(&hash[12..])
}

pub fn parse_args(args: Vec<String>) -> (Address, u64, u64) {
    let deployer_addr: Address = args[1].parse().unwrap();
    let start: u64 = args[2].parse().unwrap();
    let end: u64 = args[3].parse().unwrap();

    (deployer_addr, start, end)
}
