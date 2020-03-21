use crossterm::style::Color;
use std::io::{stdin, stdout, BufReader};

fn main() {
    let opts = hl::opts!(
        Color::Red => ("red", Some("r")),
        Color::Green => ("green", Some("g")),
        Color::Yellow => ("yellow", Some("y")),
        Color::Blue => ("blue", Some("b")),
        Color::Magenta => ("magenta", Some("m")),
        Color::Cyan => ("cyan", Some("c")),
        Color::White => ("white", Some("w")),
        Color::Grey => ("grey", None),
        Color::Black => ("black", None),
        Color::DarkRed => ("dark-red", None),
        Color::DarkGreen => ("dark-green", None),
        Color::DarkYellow=> ("dark-yellow", None),
        Color::DarkBlue => ("dark-blue", None),
        Color::DarkMagenta => ("dark-magenta", None),
        Color::DarkCyan => ("dark-cyan", None),
        Color::DarkGrey => ("dark-grey", None)
    );

    let mut stdin = BufReader::new(stdin());
    let mut stdout = stdout();

    hl::hl(&opts, &mut stdin, &mut stdout);
}
