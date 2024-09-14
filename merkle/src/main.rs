use ethers::core::abi::encode;
use ethers::core::abi::Token;
use ethers::types::{H256, U256};
use ethers::utils::keccak256;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn hash_val(val: u64) -> [u8; 32] {
    let u256: U256 = U256::from(val);
    let encoded = encode(&[Token::Uint(u256)]);
    keccak256(encoded)
}

fn hash(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut a: H256 = left.into();
    let mut b: H256 = right.into();

    if b < a {
        (a, b) = (b, a);
    }

    let encoded = encode(&[
        Token::FixedBytes(a.as_bytes().to_vec()),
        Token::FixedBytes(b.as_bytes().to_vec()),
    ]);

    keccak256(encoded)
}

fn min(x: usize, y: usize) -> usize {
    if x < y {
        x
    } else {
        y
    }
}

fn calc_root_hash(hashes: &mut Vec<[u8; 32]>) -> [u8; 32] {
    let mut n = hashes.len();

    while n > 1 {
        for i in (0..n).step_by(2) {
            let left = hashes[i];
            let right = hashes[min(i + 1, n - 1)];
            hashes[i / 2] = hash(left, right);
        }
        // div by 2 and round up
        // if n is even => n = n / 2
        // else         => n = (n + 1) / 2
        n = (n + (n & 1)) / 2;
    }

    hashes[0]
}

fn get_proof(hashes: &mut Vec<[u8; 32]>, idx: usize) -> Vec<[u8; 32]> {
    let mut proof: Vec<[u8; 32]> = Vec::new();
    let mut n = hashes.len();
    let mut k = idx;

    while n > 1 {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let j = if k & 1 == 1 { k - 1 } else { min(k + 1, n - 1) };
        proof.push(hashes[j]);
        k /= 2;

        for i in (0..n).step_by(2) {
            let left = hashes[i];
            let right = hashes[min(i + 1, n - 1)];
            hashes[i / 2] = hash(left, right);
        }
        // div by 2 and round up
        // if n is even => n = n / 2
        // else         => n = (n + 1) / 2
        n = (n + (n & 1)) / 2;
    }

    proof
}

fn verify(root: [u8; 32], proof: &Vec<[u8; 32]>, leaf_hash: [u8; 32]) -> bool {
    let mut h = leaf_hash;

    for i in 0..proof.len() {
        h = hash(proof[i], h);
    }

    h == root
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args[1].clone()).unwrap();
    let reader = BufReader::new(file);

    let mut hashes: Vec<[u8; 32]> = Vec::new();
    for line in reader.lines() {
        let v: u64 = line.unwrap().parse().unwrap();
        hashes.push(hash_val(v));
    }

    let root = calc_root_hash(&mut hashes.clone());
    let idx = 7;
    let proof = get_proof(&mut hashes.clone(), idx);

    let r: H256 = root.into();
    println!("root {:?}", r);

    let leaf: H256 = hashes[idx].into();
    println!("leaf {:?}", leaf);

    for p in proof.iter() {
        let h: H256 = p.into();
        println!("proof {:#?}", h);
    }

    let is_valid = verify(root, &proof, hashes[idx]);
    println!("{:?}", is_valid);
}
