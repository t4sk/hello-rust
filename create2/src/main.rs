use ethers::core::abi::Address;
use ethers::core::types::U256;
use ethers::core::utils::hex::ToHex;
use ethers::core::utils::keccak256;
use ethers::core::utils::rlp;
use ethers::core::utils::rlp::RlpStream;

fn main() {
    let addr = Address::zero();
    let nonce = U256::zero();
    println!("addr {:?}", addr);
    println!("nonce {:?}", nonce);

    let mut s = RlpStream::new();
    s.append(&addr);
    s.append(&nonce);

    let encoded = rlp::encode(&s.out());
    println!("{:?}", encoded);

    let b32 = keccak256(encoded);
    println!("zero zero addr 0x{}", b32.encode_hex::<String>());
}

// https://docs.rs/ethers/latest/ethers/core/utils/rlp/fn.encode.html
// https://docs.rs/ethers/latest/ethers/core/utils/fn.keccak256.html

// https://docs.rs/create2/latest/src/create2/lib.rs.html#6-13
