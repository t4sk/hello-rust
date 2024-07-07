enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn direction_vector(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => {
            println!("red");
        }
        Color::Rgb(r, g, b) => println!("r {} g {} b {}", r, g, b),
        Color::Hex(s) => println!("hex {}", s),
        _ => println!("other color"),
    }
}

fn div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        return None;
    }
    Some(x / y)
}

fn main() {
    let direction = Direction::Up;
    let color = Color::Rgb(100, 200, 0);

    let direction = Direction::Up;

    let v = direction_vector(direction);
    println!("{:?}", v);

    print_color(color);

    let z = div(1, 2);
    if let Some(num) = z {
        println!("{}", num);
    }
}
