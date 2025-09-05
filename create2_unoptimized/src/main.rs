use alloy_primitives::{Address, B256, U256, keccak256};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;

fn create2_addr(deployer: Address, salt: B256, init_code: &[u8]) -> Address {
    // keccak256(init_code)
    let init_code_hash = keccak256(init_code);

    // Build preimage: 0xff + deployer + salt + keccak256(init_code)
    let mut buf = Vec::with_capacity(1 + 20 + 32 + 32);
    buf.push(0xff);
    buf.extend_from_slice(deployer.as_slice());
    buf.extend_from_slice(salt.as_slice());
    buf.extend_from_slice(init_code_hash.as_slice());

    // keccak256(preimage), take last 20 bytes
    let hash = keccak256(&buf);
    Address::from_slice(&hash[12..32])
}

fn check(addr: Address, target_pattern: &str) -> bool {
    hex::encode(addr).starts_with(target_pattern)
}

const NUM_THREADS: u128 = 4;

/*
DEPLOYER=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef
INIT_CODE=600a600c600039600a6000f3602a60805260206080f3
TARGET=1111
cargo run $DEPLOYER $INIT_CODE $TARGET
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let deployer = Address::from_str(&args[1]).unwrap();
    let init_code = Arc::new(hex::decode(&args[2]).unwrap());
    let target_pattern = args[3].clone();

    let (tx, rx): (Sender<U256>, Receiver<U256>) = mpsc::channel();

    let start = Instant::now();
    println!("Spawning {NUM_THREADS} threads...");
    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        let init_code = Arc::clone(&init_code);
        let target = target_pattern.clone();
        thread::spawn(move || {
            // NOTE: Search salt up to u128 max, not U256 max
            let mut j: u128 = i;
            let mut k = 0;
            loop {
                let salt = B256::from(U256::from(j));
                let addr = create2_addr(deployer, salt, &init_code);

                if check(addr, &target) {
                    println!("Salt: {j}");
                    println!("Address: {:?}", addr);
                    tx.send(U256::from(j)).unwrap();
                    break;
                }

                j += NUM_THREADS;
                k += 1;

                if k % 10000 == 0 {
                    println!("Thread {i}: checked {k} salts");
                }
            }
        });
    }

    if let Ok(msg) = rx.recv() {
        println!("Received {:?}", msg);
    }

    let dt = start.elapsed();
    println!("Time elapsed: {:.3} seconds", dt.as_secs_f64());
}
