fn string_length(s: String) -> usize {
    s.len()
    // String is dropped here
}

fn str_length(s: &str) -> usize {
    s.len()
    // String is dropped here
}

fn main() {
    // String
    let msg = String::from("Hi");
    let len = string_length(msg);
    println!("String length = {}", len);

    // &str - String slice
    // - immutable
    let msg = String::from("Hi");
    let s: &str = &msg;
    let len = str_length(s);
    println!("Str length = {}", len);
    println!("msg = {}", msg);
}
