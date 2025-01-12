#![allow(unused)]
// Return closure as output

/*
 move keyword must be used, which signals that all captures occur by value.
This is required because any captures by reference would be dropped as soon as the
function exited, leaving invalid references in the closure.
*/

fn create_fn() -> impl Fn() {
    let text = "hello".to_string();
    move || println!("fn {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "hello".to_string();
    move || println!("fn mut {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "hello".to_string();
    move || println!("fn once {}", text)
}

// TODO: dyn
// fn create_fn_once() -> dyn FnOnce() {
//     let text = "hello".to_string();
//     move || println!("fn once {}", text)
// }

fn main() {
    let f = create_fn();
    f();
    f();

    let mut f = create_fn_mut();
    f();
    f();

    let f = create_fn_once();
    f();
    // Note: cannot call again
    // f();
}
