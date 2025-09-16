use alloy_primitives::{B256, keccak256};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn hash_leaf<A: AsRef<[u8]>>(val: A) -> B256 {
    keccak256(&val)
}

fn hash_pair(left: B256, right: B256) -> B256 {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(left.as_ref());
    buf[32..].copy_from_slice(right.as_ref());
    keccak256(buf)
}

fn calc_root_hash(hashes: &mut [B256]) -> B256 {
    let mut n = hashes.len();
    assert!(n > 0);

    while n > 1 {
        for i in (0..n).step_by(2) {
            let left = hashes[i];
            let right = hashes[usize::min(i + 1, n - 1)];
            hashes[i / 2] = hash_pair(left, right);
        }
        // div by 2 and round up
        n = (n + 1) / 2
    }

    hashes[0]
}

fn get_proof(hashes: &mut [B256], mut idx: usize) -> Vec<B256> {
    let mut proof: Vec<B256> = Vec::new();
    let mut n = hashes.len();
    assert!(n > 0);

    while n > 1 {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let j = if idx & 1 == 1 {
            idx - 1
        } else {
            usize::min(idx + 1, n - 1)
        };
        proof.push(hashes[j]);
        idx /= 2;

        for i in (0..n).step_by(2) {
            let left = hashes[i];
            let right = hashes[usize::min(i + 1, n - 1)];
            hashes[i / 2] = hash_pair(left, right);
        }
        // div by 2 and round up
        n = (n + 1) / 2
    }

    proof
}

fn verify(root: B256, proof: &[B256], hashes: &[B256], mut idx: usize) -> bool {
    let mut h = hashes[idx];
    for p in proof {
        let (left, right): (B256, B256) = if idx & 1 == 0 { (h, *p) } else { (*p, h) };
        h = hash_pair(left, right);
        idx /= 2;
    }
    h == root
}

// cargo run
fn main() {
    /*
    // Example of Merkle tree algorithms reading values from a file
    // cargo run test.txt 2
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();
    let idx: usize = args[2].parse().unwrap();

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let hashes: Vec<B256> = reader
        .lines()
        .map(|line| {
            let v: String = line.unwrap().trim().to_string();
            let h = hash_leaf(v.clone());
            println!("{v}: {h}");
            h
        })
        .collect();
    */

    // Example of Merkle tree algorithms from hard coded values
    let vals: Vec<String> = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let idx: usize = 5;

    let mut hashes: Vec<B256> = Vec::new();
    for v in vals {
        let h = hash_leaf(v.clone());
        println!("{v}: {h}");
        hashes.push(h);
    }

    let root = calc_root_hash(&mut hashes.clone());
    let proof = get_proof(&mut hashes.clone(), idx);

    println!("root {:?}", root);
    println!("leaf {:?}", hashes[idx]);
    println!("index: {idx}");

    for (i, p) in proof.iter().enumerate() {
        println!("proof {i}: {:#?}", p);
    }

    let is_valid = verify(root, &proof, &hashes, idx);
    println!("{:?}", is_valid);
}
