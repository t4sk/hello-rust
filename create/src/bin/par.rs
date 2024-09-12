use ethers::abi::Address;
use ethers::types::U256;
use std::env;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

use create::{calc_contract_addr, parse_args};

const MAX_THREADS: u64 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (deployer_addr, start, end) = parse_args(args);

    // Channel to send and recieve results from threads
    let (tx, rx) = channel();

    println!("Deployer: {:?}", deployer_addr);
    println!("nonce | contract address");

    let diff: U256 = end + 1 - start;
    assert!(diff >= MAX_THREADS.into());

    let n_per_thread: U256 = diff / MAX_THREADS;
    let rem: U256 = diff - n_per_thread * MAX_THREADS;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..MAX_THREADS {
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            let start = n_per_thread * i;
            let mut end = n_per_thread * (i + 1);
            if i == MAX_THREADS - 1 {
                end += rem
            }

            let mut j = start;
            while j < end {
                let contract_addr = calc_contract_addr(deployer_addr, j);
                tx.send((j, contract_addr)).unwrap();
                j += 1u64.into();
            }
        });
        handles.push(handle);
    }

    // Drop the original sender to close channel
    drop(tx);

    // Collect results
    let mut addrs: Vec<(U256, Address)> = Vec::new();
    for msg in rx {
        addrs.push(msg);
    }

    // Sort by nonce
    addrs.sort_by(|a, b| a.0.cmp(&b.0));

    for h in handles {
        h.join().unwrap();
    }

    for (i, addr) in addrs {
        println!("{:?}, {:?}", i, addr);
    }
}
