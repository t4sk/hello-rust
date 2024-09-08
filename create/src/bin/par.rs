use std::env;
use std::thread;
use std::thread::JoinHandle;

use create::{calc_contract_addr, parse_args};

const MAX_THREADS: u64 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (deployer_addr, start, end) = parse_args(args);

    println!("Deployer: {:?}", deployer_addr);
    println!("nonce | contract address");

    let diff: u64 = end + 1 - start;
    assert!(diff >= MAX_THREADS);

    let n_per_thread = diff / MAX_THREADS;
    let rem = diff - n_per_thread * MAX_THREADS;

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..MAX_THREADS {
        let handle = thread::spawn(move || {
            let start = i * n_per_thread;
            let mut end = (i + 1) * n_per_thread;
            if i == MAX_THREADS - 1 {
                end += rem
            }

            for j in start..end {
                let contract_addr = calc_contract_addr(deployer_addr, j);
                // TODO: store in vec and sort
                println!("{:?}, {:?}", j, contract_addr);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}
