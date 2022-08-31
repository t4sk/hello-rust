use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let duration = Duration::new(1, 0);

    for i in 0..5 {
        sleep(duration);
        println!("{}", now.elapsed().as_secs());
    }
}
