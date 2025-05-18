#![allow(unused)]

use std::sync::Arc;
// use std::sync::Mutex;
use tokio::join;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn f(logger: Arc<Mutex<Vec<String>>>, msg: &'static str) {
    // Using std::sync::Mutex blocks the thread,
    // which can cause the main function to hang.
    // let mut logs = logger.lock().unwrap();

    // Yield execution to the executor if the lock cannot be immediately acquired.
    let mut logs = logger.lock().await;
    sleep(Duration::from_millis(10)).await;
    logs.push(msg.to_string());
}

#[tokio::main]
async fn main() {
    let logger = Arc::new(Mutex::new(Vec::new()));

    join!(f(logger.clone(), "f1"), f(logger.clone(), "f2"));
    f(logger.clone(), "f3").await;

    println!("{:?}", logger);
}
