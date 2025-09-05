use std::{
    env,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Instant,
};

use alloy_primitives::{Address, B256, U256, keccak256};

const NUM_THREADS: u128 = 4;

/// Parse (case-insensitive) hex prefix into (full_bytes, optional high nibble).
fn parse_hex_prefix(s: &str) -> (Vec<u8>, Option<u8>) {
    let s = s.to_ascii_lowercase();
    let full_len = s.len() / 2;
    let mut bytes = Vec::with_capacity(full_len);
    let mut it = s.as_bytes().iter().copied();

    for _ in 0..full_len {
        let hi = it.next().unwrap();
        let lo = it.next().unwrap();
        bytes.push((hex_val(hi) << 4) | hex_val(lo));
    }
    let half = it.next().map(hex_val);
    (bytes, half)
}

#[inline]
fn hex_val(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => 10 + (c - b'a'),
        b'A'..=b'F' => 10 + (c - b'A'),
        _ => panic!("invalid hex char"),
    }
}

/// Compare raw address bytes against parsed hex prefix.
#[inline]
fn addr_starts_with(addr: &Address, prefix_full: &[u8], prefix_half: Option<u8>) -> bool {
    let a = addr.as_slice(); // 20 bytes
    if !a.starts_with(prefix_full) {
        return false;
    }
    if let Some(half) = prefix_half {
        if prefix_full.len() >= a.len() {
            return false;
        }
        let next_byte = a[prefix_full.len()];
        let next_hi_nibble = next_byte >> 4;
        return next_hi_nibble == half;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <DEPLOYER> <INIT_CODE_HEX> <HEX_PREFIX>", args[0]);
        std::process::exit(1);
    }

    let deployer = Address::from_str(&args[1]).expect("bad deployer");
    let init_code = hex::decode(&args[2]).expect("bad init code hex");
    let (prefix_full, prefix_half) = parse_hex_prefix(&args[3]);

    let init_code_hash = keccak256(&init_code);

    // Precompute the constant parts of the preimage:
    // preimage = 0xff (1) || deployer (20) || salt (32) || init_code_hash (32) => 85 bytes
    let head: [u8; 21] = {
        let mut h = [0u8; 21];
        h[0] = 0xff;
        h[1..].copy_from_slice(deployer.as_slice());
        h
    };
    // Copy hash into a fixed 32-byte array
    let mut tail = [0u8; 32];
    tail.copy_from_slice(init_code_hash.as_slice());

    let (tx, rx) = mpsc::channel::<(U256, Address)>();
    let found = Arc::new(AtomicBool::new(false));

    let start = Instant::now();
    println!("Spawning {} threads...", NUM_THREADS);

    // Share small read-only data
    let prefix_full = Arc::new(prefix_full);
    let found_arc = found.clone();

    for i in 0..(NUM_THREADS as usize) {
        let tx = tx.clone();
        let prefix_full = prefix_full.clone();
        let found = found_arc.clone();

        // Move small arrays by value (no Arc needed)
        let head_local = head;
        let tail_local = tail;

        thread::spawn(move || {
            // Preimage buffer reused each iteration; overwrite only the salt window
            let mut preimage = [0u8; 85];
            preimage[..21].copy_from_slice(&head_local);
            preimage[53..].copy_from_slice(&tail_local);

            // Salt search spaced by thread count to avoid overlap
            let mut j: u128 = i as u128;

            while !found.load(Ordering::Relaxed) {
                let salt_b256 = B256::from(U256::from(j));
                preimage[21..53].copy_from_slice(salt_b256.as_slice());

                // keccak256(preimage) -> last 20 bytes is the address
                let h = keccak256(&preimage);
                let addr = Address::from_slice(&h[12..32]);

                if addr_starts_with(&addr, &prefix_full, prefix_half) {
                    if !found.swap(true, Ordering::SeqCst) {
                        // first winner sends
                        let _ = tx.send((U256::from(j), addr));
                    }
                    break;
                }
                j += NUM_THREADS;
            }
        });
    }

    drop(tx); // close sender

    if let Ok((salt, addr)) = rx.recv() {
        println!("Found!");
        // Print salt as 0x… by converting through B256 for reliable 32-byte BE encoding
        let salt_be = B256::from(salt);
        println!("Salt (u128): {salt}");
        println!("Salt (U256): 0x{}", hex::encode(salt_be));
        println!("Address: 0x{}", hex::encode(addr));
    } else {
        println!("No result received (all threads exited).");
    }

    let dt = start.elapsed();
    println!("Time elapsed: {:.3} seconds", dt.as_secs_f64());
}
