// Return closure as output

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

    // TODO: why mut on f?
    let mut f = create_fn_mut();
    f();
    f();

    let f = create_fn_once();
    f();
    // Note: cannot call again
    // f();
}
