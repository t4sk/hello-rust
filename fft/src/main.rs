#![allow(unused)]

fn eval(poly: &[i64], x0: i64, m: i64) -> i64 {
    let mut y = 0;
    let mut x = 1;

    for c in poly.iter() {
        y = (y + (c * x).rem_euclid(m)).rem_euclid(m);
        x = (x * x0).rem_euclid(m);
    }

    y
}

fn fft(poly: &[i64], xs: &[i64], m: i64) -> Vec<i64> {
    let d = poly.len();
    if d == 1 {
        return vec![poly[0]];
    }

    // split poly into even and odd
    let mut pe = Vec::new();
    let mut po = Vec::new();
    for i in (0..d).step_by(2) {
        pe.push(poly[i]);
        if i + 1 < d {
            po.push(poly[i + 1]);
        }
    }

    let x_len = xs.len();
    let mut x_evens = Vec::new();
    for i in (0..x_len).step_by(2) {
        x_evens.push(xs[i]);
    }

    let ye = fft(&pe, &x_evens, m);
    let yo = fft(&po, &x_evens, m);

    let mut ys = vec![0; x_len];
    let l = d / 2;
    for i in (0..l) {
        let v = xs[i] * yo[i];
        ys[i] = (ye[i] + v).rem_euclid(m);
        ys[i + l] = (ye[i] - v).rem_euclid(m);
    }

    ys
}

fn main() {
    // m must be prime
    let m = 337;
    let poly = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let xs = vec![1, 85, 148, 111, 336, 252, 189, 226];

    let ys = fft(&poly, &xs, m);
    println!("{:?}", ys);

    for x in xs.iter() {
        let y = eval(&poly, *x, m);
        println!("{}", y);
    }
}
