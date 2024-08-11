#![allow(unused)]
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

// input impl
fn func_impl_in(f: impl Fn(i32) -> i32) -> i32 {
    let x = f(1);
    x
}

// return impl
fn func_impl_out(x: i32) -> impl Fn(i32) -> i32 {
    let func = move |y: i32| x + y;
    func
}

fn main() {
    let x: u32 = 123;
    func_in(x);
    let x = func_out();
    println!("out {:?}", x.get());

    let x = func_impl_in(|x| x + 1);
    println!("f {}", x);

    let f = func_impl_out(1);
    println!("f {}", f(2));
}
