use ethers::core::utils::rlp::RlpStream;
use ethers::types::Address;
use ethers::utils::keccak256;

// https://ethereum.stackexchange.com/questions/760/how-is-the-address-of-an-ethereum-contract-computed#:~:text=The%20address%20for%20an%20Ethereum,then%20hashed%20with%20Keccak%2D256.
pub fn calc_contract_addr(deployer_addr: Address, nonce: u64) -> Address {
    // RLP encode(address, nonce)
    let mut stream = RlpStream::new();
    stream.begin_list(2);
    stream.append(&deployer_addr);
    stream.append(&nonce);
    let out = stream.out();

    let hash = keccak256(out);
    Address::from_slice(&hash[12..])
}

pub fn parse_args(args: Vec<String>) -> (Address, u64, u64) {
    let deployer_addr: Address = args[1].parse().unwrap();
    let start: u64 = args[2].parse().unwrap();
    let end: u64 = args[3].parse().unwrap();

    (deployer_addr, start, end)
}
