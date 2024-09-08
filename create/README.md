```shell
cargo build

cargo run --bin [one or par] [address] [start nonce] [end nonce]

# single thread
cargo run --bin one 0x5995922552401d005f263c0581aa01060a3e02a1 0 25

# parallel
cargo run --bin par 0x5995922552401d005f263c0581aa01060a3e02a1 0 25
```
