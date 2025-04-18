#![allow(unused)]

// mpsc - multi producer, single consumer
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    // Simple example
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Unwrap Result - Err if receiver is dropped
    tx.send("hello".to_string()).unwrap();
    let res = rx.recv();
    println!("{:?}", res);

    // This will fail
    // std::mem::drop(rx);
    // tx.send("hello".to_string()).unwrap();

    // Simple example with thread
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("hello".to_string());
    });

    // recv block main thread until a new message arrives
    println!("waiting...");
    let res = rx.recv();
    println!("received: {:?}", res);

    // try_recv - doesn't block main thread
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("hello".to_string());
    });

    loop {
        // try_recv doesn't block main thread
        match rx.try_recv() {
            Ok(msg) => println!("{msg}"),
            Err(TryRecvError::Empty) => {
                println!("no message");
            }
            Err(TryRecvError::Disconnected) => {
                println!("disconnected");
                break;
            }
        }
        // This code is executed even if no new message arrives
        thread::sleep(Duration::from_millis(10));
    }

    // Send several messages
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..3 {
            tx.send(format!("msg {i}"));
        }
    });

    loop {
        match rx.recv() {
            Ok(msg) => println!("{msg}"),
            Err(err) => {
                println!("{err:?}");
                break;
            }
        }
    }

    // For loop
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..3 {
            tx.send(format!("msg {i}"));
        }
    });

    for msg in rx {
        println!("for loop: {msg}");
    }

    // Multiple producer, single consumer
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let animals: Vec<String> = ["🦀", "🐸", "🐱", "🐻", "🦝", "🐺"]
        .into_iter()
        .map(|s| s.into())
        .collect();

    for emoji in animals.into_iter() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(emoji.to_string());
        });
    }

    // main hangs if tx is not dropped
    // Dropping tx signals that no more messages will be sent, allowing rx to terminate properly
    std::mem::drop(tx);

    let mut s = "".to_string();

    // Blocks until all message is received or tx is dropped
    while let Ok(msg) = rx.recv() {
        s += &msg;
    }

    // No need to join the threads
    println!("{s}");
}
