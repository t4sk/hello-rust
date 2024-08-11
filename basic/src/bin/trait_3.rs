// Trait input and output
trait Input {
    fn get(&self) -> u32;
}

trait Output {
    fn get(&self) -> u32;
}

impl Input for u32 {
    fn get(&self) -> u32 {
        return 123;
    }
}

impl Output for u32 {
    fn get(&self) -> u32 {
        return 456;
    }
}

fn func_in(i: impl Input) {
    println!("input {}", i.get());
}

fn func_out() -> Box<dyn Output> {
    let x: u32 = 1234;
    return Box::new(x);
}

fn main() {
    let x: u32 = 123;
    func_in(x);
    let x = func_out();
    println!("out {:?}", x.get());
}
