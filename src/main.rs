use clap::{App, Arg, ArgMatches};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Default)]
struct Opts {
    red: Option<(Regex, usize)>,
    green: Option<(Regex, usize)>,
    yellow: Option<(Regex, usize)>,
    blue: Option<(Regex, usize)>,
    magenta: Option<(Regex, usize)>,
    cyan: Option<(Regex, usize)>,
    white: Option<(Regex, usize)>,
}

impl Opts {
    fn patterns(&self) -> Vec<(Color, &(Regex, usize))> {
        use Color::*;
        let mut patterns = Vec::new();
        if let Some(red) = &self.red {
            patterns.push((Red, red));
        }
        if let Some(green) = &self.green {
            patterns.push((Green, green));
        }
        if let Some(yellow) = &self.yellow {
            patterns.push((Yellow, yellow));
        }
        if let Some(blue) = &self.blue {
            patterns.push((Blue, blue));
        }
        if let Some(magenta) = &self.magenta {
            patterns.push((Magenta, magenta));
        }
        if let Some(cyan) = &self.cyan {
            patterns.push((Cyan, cyan));
        }
        if let Some(white) = &self.white {
            patterns.push((White, white));
        }
        patterns
    }
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
    ( $( $x:expr ),* ) => {
        {
            let mut app = App::new("hl")
                .version("0.1.0")
                .author("Odin Dutton <odindutton@gmail.com>")
                .about("Highlight patterns.");
            $(
                 app = app.arg(arg($x, concat!("Highlight PATTERN in ", $x)));
            )*
            let matches = app.get_matches();

            Opts {
                red: get_arg(&matches, "red"),
                green: get_arg(&matches, "green"),
                yellow: get_arg(&matches, "yellow"),
                blue: get_arg(&matches, "blue"),
                magenta: get_arg(&matches, "magenta"),
                cyan: get_arg(&matches, "cyan"),
                white: get_arg(&matches, "white"),
            }
        }
    };
}

fn main() {
    let opts = opts!["red", "green", "yellow", "blue", "magenta", "cyan", "white"];

    let mut stdin = BufReader::new(std::io::stdin());
    let mut stdout = std::io::stdout();

    hl(&opts, &mut stdin, &mut stdout);
}

fn arg<'a, 'b>(name: &'a str, help: &'a str) -> Arg<'a, 'b> {
    Arg::with_name(name)
        .short(name.chars().next().unwrap().to_string())
        .long(name)
        .value_name("PATTERN")
        .help(help)
        .takes_value(true)
}

fn get_arg(matches: &ArgMatches, key: &str) -> Option<(Regex, usize)> {
    matches.value_of(key).map(|pattern| {
        let regex = Regex::new(&pattern).unwrap_or_else(|_| {
            eprintln!("Invalid regex: {:?}", pattern);
            std::process::exit(1);
        });
        (regex, matches.index_of(key).unwrap())
    })
}

fn hl<T, U>(opts: &Opts, reader: &mut U, output: &mut T)
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
        for (color, (regex, order)) in opts.patterns() {
            for mat in regex.find_iter(&input) {
                indices
                    .entry(mat.start())
                    .or_insert_with(Vec::new)
                    .push(Style::start(color, *order));
                indices
                    .entry(mat.end())
                    .or_insert_with(Vec::new)
                    .push(Style::end(color, *order));
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
        let opts = Opts {
            red: Some((Regex::new("foo bar baz").unwrap(), 0)),
            blue: Some((Regex::new("ba").unwrap(), 1)),
            green: Some((Regex::new("bar").unwrap(), 2)),
            ..Opts::default()
        };

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
