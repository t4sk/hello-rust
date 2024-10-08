use zkstark::montgomery::inv;

fn main() {
    let a = u128::MAX;
    let i = inv(a);
    println!("{} {}", i, i.wrapping_mul(a));
}
