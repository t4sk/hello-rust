fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn print(&self) {
        println!("Book title: {}", self.title);
    }
}

fn main() {
    let s1 = String::from("abcef");
    let s2 = String::from("xyz");
    let res = longest(&s1, &s2);
    println!("{}", res);

    let title = "Mastering Ethereum";
    let book = Book { title };

    book.print();

    // TODO: lifetime elision

    // static lifetime
    let s: &'static str = "STATIC";
}
