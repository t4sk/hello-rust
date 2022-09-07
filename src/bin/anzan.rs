use rand::Rng;
use std::io;

const NUM_RANDS: u32 = 5;

fn main() {
    loop {
        let mut rng = rand::thread_rng();

        let mut nums: Vec<u32> = vec![];
        let mut answer: u32 = 0;
        for _ in 0..NUM_RANDS {
            let r = rng.gen_range(1..10);
            nums.push(r);
            answer += r;
        }

        // println!("{:?} {}", nums, answer);
        println!("{:?}", nums);

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to readline");

            let num: u32 = input.trim().parse().expect("input not an integer");

            if num == answer {
                println!("correct!");
                break;
            }
        }
    }
}
