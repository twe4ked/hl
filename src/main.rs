use crossterm::style::Color;
use std::io::{stdin, stdout, BufReader};

fn main() {
    let opts = hl::opts!(
        Color::Red => "red",
        Color::Green => "green",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
        Color::Magenta => "magenta",
        Color::Cyan => "cyan",
        Color::White => "white"
    );

    let mut stdin = BufReader::new(stdin());
    let mut stdout = stdout();

    hl::hl(&opts, &mut stdin, &mut stdout);
}
