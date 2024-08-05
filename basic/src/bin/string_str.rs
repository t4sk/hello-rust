fn string_length(s: String) -> usize {
    s.len()
    // String is dropped here
}

fn str_length(s: &str) -> usize {
    s.len()
    // String is dropped here
}

// String = Vec<u8> valid UTF-8
// &str = &[u8] valid UTF-8
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

    let s = r#"
        { "a": 1,
          "b": { "c": 2 },
          "d": 3
        }
    "#;
    println!("{s}");
}
