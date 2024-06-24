fn main() {
    let s = String::from("hello world");

    // &s[..5]
    let hello = &s[0..5];
    // &s[6..]
    let world = &s[6..11];
}
