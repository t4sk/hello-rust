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

fn main() {
    let direction = Direction::Up;
    let color = Color::Rgb(100, 200, 0);
}
