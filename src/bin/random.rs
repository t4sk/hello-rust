use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for i in 0..10 {
        println!("{}", i);
        println!("Integer: {}", rng.gen_range(0..10));
        println!("Float: {}", rng.gen_range(0.0..10.0));
    }
}
