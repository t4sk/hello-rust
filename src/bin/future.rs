#![allow(unused)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

// Hello
async fn hello() {
    println!("hello");
}

// Return type of a async function is a compiler-generated anonymous type
// A type that implements Future<Output = i32>
fn hello_fut() -> impl Future<Output = ()> {
    async {
        println!("hello");
    }
}

struct Hello;

impl Future for Hello {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("hello");
        Poll::Ready(())
    }
}

fn hello_impl_fut() -> Hello {
    Hello
}

// Sleep
async fn sleep(dt: u64) {
    tokio::time::sleep(Duration::from_millis(dt)).await;
}

fn sleep_fut(dt: u64) -> impl Future<Output = ()> {
    async move {
        tokio::time::sleep(Duration::from_millis(dt)).await;
    }
}

fn sleep_impl_fut(dt: u64) -> Sleep {
    Sleep {
        time: Instant::now() + Duration::from_millis(dt),
    }
}

#[derive(Debug)]
struct Sleep {
    pub time: Instant,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if Instant::now() >= self.time {
            Poll::Ready(())
        } else {
            // waker must be called when returning Pending
            // Forgetting to do this results in the task hanging indefinitely

            // This wakes the executor on every loop which wastes CPU cycles
            // println!("pending");
            // cx.waker().wake_by_ref();

            let waker = cx.waker().clone();
            let time = self.time;

            // Thread is spawned per call to sleep
            std::thread::spawn(move || {
                let now = Instant::now();
                if now < time {
                    std::thread::sleep(time - now);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    // Compiler generated anonymous type
    let fut: _ = hello_fut();
    // Need to await for output
    println!("hello_fut: {:?}", fut.await);

    let fut = hello_impl_fut();
    println!("hello_impl_fut: {:?}", fut.await);

    sleep_impl_fut(1000).await;
    println!("⏰");
}
