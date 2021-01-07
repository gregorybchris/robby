#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Color {
    Blue,
    Default,
    Red,
}

fn get_color_code(color: Color) -> String {
    match color {
        Color::Blue => String::from("\x1B[34m"),
        Color::Default => String::from("\x1B[0m"),
        Color::Red => String::from("\x1B[31m"),
    }
}

pub fn print_color(message: String, color: Color) {
    let color_code = get_color_code(color);
    let reset_code = get_color_code(Color::Default);
    print!("{}{}{}", color_code, message, reset_code)
}
