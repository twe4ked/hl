use clap::{App, Arg};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug)]
struct Opt {
    color: Color,
    regex: Regex,
    index: usize,
}

#[derive(Debug)]
enum Operation {
    Start,
    End,
}

#[derive(Debug)]
struct Style {
    operation: Operation,
    color: Color,
    order: usize,
}

impl Style {
    fn start(color: Color, order: usize) -> Self {
        Self {
            operation: Operation::Start,
            color,
            order,
        }
    }

    fn end(color: Color, order: usize) -> Self {
        Self {
            operation: Operation::End,
            color,
            order,
        }
    }
}

macro_rules! opts {
    ( $( $color:path => $name:expr ),* ) => {
        {
            let mut app = App::new("hl")
                .version("0.1.0")
                .author("Odin Dutton <odindutton@gmail.com>")
                .about("Highlight patterns.");
            $(
                app = app.arg(
                    Arg::with_name($name)
                        .short($name.chars().next().unwrap().to_string())
                        .long($name)
                        .value_name("PATTERN")
                        .help(concat!("Highlight PATTERN in ", $name))
                        .takes_value(true)
                );
            )*
            let matches = app.get_matches();

            let mut opts = Vec::new();
            $(
                matches.value_of($name).map(|pattern| {
                    let regex = Regex::new(&pattern).unwrap_or_else(|_| {
                        eprintln!("Invalid regex: {:?}", pattern);
                        std::process::exit(1);
                    });
                    let index = matches.index_of($name).unwrap();
                    opts.push(Opt {
                        color: $color,
                        regex,
                        index,
                    });
                });
            )*
            opts
        }
    };
}

fn main() {
    let opts = opts!(
        Color::Red => "red",
        Color::Green => "green",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
        Color::Magenta => "magenta",
        Color::Cyan => "cyan",
        Color::White => "white"
    );

    let mut stdin = BufReader::new(std::io::stdin());
    let mut stdout = std::io::stdout();

    hl(&opts, &mut stdin, &mut stdout);
}

fn hl<T, U>(opts: &[Opt], reader: &mut U, output: &mut T)
where
    T: Write,
    U: BufRead,
{
    let mut input = String::new();
    loop {
        let len = reader.read_line(&mut input).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(e.raw_os_error().unwrap_or(1));
        });
        if len == 0 {
            break;
        }

        let mut indices = HashMap::<usize, Vec<Style>>::new();
        for opt in opts {
            for mat in opt.regex.find_iter(&input) {
                indices
                    .entry(mat.start())
                    .or_insert_with(Vec::new)
                    .push(Style::start(opt.color, opt.index));
                indices
                    .entry(mat.end())
                    .or_insert_with(Vec::new)
                    .push(Style::end(opt.color, opt.index));
            }
        }
        for (_, v) in indices.iter_mut() {
            v.sort_by(|a, b| a.order.cmp(&b.order));
        }

        let mut stack = Vec::new();
        input.chars().enumerate().for_each(|(i, c)| {
            if let Some(styles) = indices.get(&i) {
                for style in styles {
                    match style.operation {
                        Operation::Start => {
                            stack.push(style.color);
                            let _ = write!(output, "{}", SetForegroundColor(style.color));
                        }
                        Operation::End => {
                            if let Some(pos) = stack.iter().rposition(|x| x == &style.color) {
                                stack.remove(pos);
                            }
                            let _ = write!(output, "{}", ResetColor);
                            for color in &stack {
                                let _ = write!(output, "{}", SetForegroundColor(*color));
                            }
                        }
                    }
                }
            }
            let _ = write!(output, "{}", c);
        });
        input.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_multi_line() {
        let opts = vec![
            Opt {
                color: Color::Red,
                regex: Regex::new("foo bar baz").unwrap(),
                index: 0,
            },
            Opt {
                color: Color::Blue,
                regex: Regex::new("ba").unwrap(),
                index: 1,
            },
            Opt {
                color: Color::Green,
                regex: Regex::new("bar").unwrap(),
                index: 2,
            },
        ];

        let mut output = Vec::new();
        let input = "foo bar baz qux bar bar
foo bar
qux baz
bar
foo bar baz bar"
            .as_bytes();
        let mut input = BufReader::new(input);
        hl(&opts, &mut input, &mut output);
        let output = String::from_utf8(output).unwrap();

        let red = format!("{}", SetForegroundColor(Color::Red));
        let green = format!("{}", SetForegroundColor(Color::Green));
        let blue = format!("{}", SetForegroundColor(Color::Blue));
        let reset = format!("{}", ResetColor);

        let expected = format!(
            "{red}foo {blue}{green}ba{reset}{red}{green}r{reset}{red} {blue}ba{reset}{red}z{reset} qux \
{blue}{green}ba{reset}{green}r{reset} {blue}{green}ba{reset}{green}r{reset}
foo {blue}{green}ba{reset}{green}r{reset}
qux {blue}ba{reset}z
{blue}{green}ba{reset}{green}r{reset}
{red}foo {blue}{green}ba{reset}{red}{green}r{reset}{red} {blue}ba{reset}{red}z{reset} {blue}{green}ba{reset}{green}r",
            red = red,
            green = green,
            blue = blue,
            reset = reset
        );

        println!("Output:\n{}{}", &output, reset);
        println!("Expected:\n{}{}", &expected, reset);
        assert_eq!(output, expected);
    }
}
