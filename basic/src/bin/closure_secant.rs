fn abs(x: f32, y: f32) -> f32 {
    return if x >= y { x - y } else { y - x };
}

// TODO: what's this closure syntax?
fn f_solve<F: Fn(f32) -> f32>(f: F, x0: f32, x1: f32, n: usize, delta: f32) -> f32 {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut f0 = f(x0);
    let mut f1;
    for i in 0..n {
        println!("{} {}", i, x0);

        if abs(f0, 0.0) <= delta {
            return x0;
        }

        f1 = f(x1);
        if abs(f1, f0) <= delta {
            return x0;
        }

        let x2 = x1 - f1 * (x1 - x0) / (f1 - f0);
        x0 = x1;
        x1 = x2;
        f0 = f1;
    }
    panic!("Solution not found");
}

fn f(x: f32, a: f32) -> f32 {
    x * x + a
}

fn main() {
    let a = -2.0;
    let x0 = 0.0;
    let x1 = 4.0;
    let n = 100;
    let delta = 0.001;

    let x = f_solve(|x| f(x, a), x0, x1, n, delta);
    println!("x {}", x);
    println!("f(x) {}", f(x, a));
}
