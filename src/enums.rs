#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    Cmyk { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}

pub fn enums() {
    let color: Color = Color::Red;

    let color_match: String = match color {
        Color::Red => String::from("Red"),
        Color::Green => String::from("Green"),
        Color::Blue => String::from("Blue"),
        Color::RgbColor(0, 0, 0) => String::from("Black"),
        Color::RgbColor(r, g, b) => format!("{},{},{}", r, g, b),
        Color::Cmyk { cyan, magenta, yellow, black} => format!("{},{},{},{}", cyan, magenta, yellow, black),
    };

    println!("Enums: {}", color_match);
}