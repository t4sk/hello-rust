use std::env;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;

use ethers::abi::Address;

use create::{calc_contract_addr, parse_args};

const MAX_THREADS: u64 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (deployer_addr, start, end) = parse_args(args);

    // Channel to send and recieve results from threads
    let (tx, rx) = channel();

    println!("Deployer: {:?}", deployer_addr);
    println!("nonce | contract address");

    let diff: u64 = end + 1 - start;
    assert!(diff >= MAX_THREADS);

    let n_per_thread = diff / MAX_THREADS;
    let rem = diff - n_per_thread * MAX_THREADS;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..MAX_THREADS {
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            let start = i * n_per_thread;
            let mut end = (i + 1) * n_per_thread;
            if i == MAX_THREADS - 1 {
                end += rem
            }

            for j in start..end {
                let contract_addr = calc_contract_addr(deployer_addr, j);
                tx.send((j, contract_addr)).unwrap();
            }
        });
        handles.push(handle);
    }

    // Drop the original sender to close channel
    drop(tx);

    // Collect results
    let mut addrs: Vec<(u64, Address)> = Vec::new();
    for msg in rx {
        addrs.push(msg);
    }

    for h in handles {
        h.join().unwrap();
    }

    for (i, addr) in addrs {
        println!("{}, {:?}", i, addr);
    }
}
