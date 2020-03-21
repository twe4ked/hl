use clap::{App, Arg, ArgMatches};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Default)]
struct Opts {
    red: Option<(String, usize)>,
    green: Option<(String, usize)>,
    yellow: Option<(String, usize)>,
    blue: Option<(String, usize)>,
    magenta: Option<(String, usize)>,
    cyan: Option<(String, usize)>,
    white: Option<(String, usize)>,
}

impl Opts {
    fn patterns(&self) -> Vec<(Color, &(String, usize))> {
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
enum Style {
    Start(Color, usize),
    End(Color, usize),
}

impl Style {
    fn order(&self) -> usize {
        match self {
            Self::Start(_, order) => *order,
            Self::End(_, order) => *order,
        }
    }
}

fn main() {
    let matches = App::new("hl")
        .version("0.1.0")
        .author("Odin Dutton <odindutton@gmail.com>")
        .about("Highlight patterns.")
        .arg(arg("red", "Highlight PATTERN in red"))
        .arg(arg("green", "Highlight PATTERN in green"))
        .arg(arg("yellow", "Highlight PATTERN in yellow"))
        .arg(arg("blue", "Highlight PATTERN in blue"))
        .arg(arg("magenta", "Highlight PATTERN in magenta"))
        .arg(arg("cyan", "Highlight PATTERN in cyan"))
        .arg(arg("white", "Highlight PATTERN in white"))
        .get_matches();

    let opts = Opts {
        red: get_arg(&matches, "red"),
        green: get_arg(&matches, "green"),
        yellow: get_arg(&matches, "yellow"),
        blue: get_arg(&matches, "blue"),
        magenta: get_arg(&matches, "magenta"),
        cyan: get_arg(&matches, "cyan"),
        white: get_arg(&matches, "white"),
    };

    let input = read_stdin();
    let mut stdout = std::io::stdout();

    hl(&opts, &input, &mut stdout);
}

fn arg<'a>(name: &'a str, help: &'a str) -> Arg<'a> {
    Arg::with_name(name)
        .short(name.chars().next().unwrap())
        .long(name)
        .value_name("PATTERN")
        .help(help)
        .takes_value(true)
}

fn get_arg(matches: &ArgMatches, key: &str) -> Option<(String, usize)> {
    if let Some(v) = matches.value_of(key) {
        Some((v.to_string(), matches.index_of(key).unwrap()))
    } else {
        None
    }
}

fn hl<T>(opts: &Opts, input: &str, output: &mut T)
where
    T: Write,
{
    let mut indices = HashMap::<usize, Vec<Style>>::new();
    for (color, (pattern, order)) in opts.patterns() {
        for mat in Regex::new(&pattern).unwrap().find_iter(&input) {
            indices
                .entry(mat.start())
                .or_insert_with(Vec::new)
                .push(Style::Start(color, *order));
            indices
                .entry(mat.end())
                .or_insert_with(Vec::new)
                .push(Style::End(color, *order));
        }
    }
    for (_, v) in indices.iter_mut() {
        v.sort_by(|a, b| a.order().cmp(&b.order()));
    }

    let mut stack = Vec::new();
    input.chars().enumerate().for_each(|(i, c)| {
        if let Some(start_or_ends) = indices.get(&i) {
            for start_or_end in start_or_ends {
                match start_or_end {
                    Style::Start(color, _) => {
                        stack.push(color);
                        write!(output, "{}", SetForegroundColor(*color)).unwrap();
                    }
                    Style::End(color, _) => {
                        if let Some(pos) = stack.iter().rposition(|x| x == &color) {
                            stack.remove(pos);
                        }
                        write!(output, "{}", ResetColor).unwrap();
                        for x in &stack {
                            write!(output, "{}", SetForegroundColor(**x)).unwrap();
                        }
                    }
                }
            }
        }
        write!(output, "{}", c).unwrap();
    });
}

fn read_stdin() -> String {
    use std::io::{stdin, Read};
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();
    String::from_utf8(input).expect("invalid UTF-8 input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_multi_line() {
        let opts = Opts {
            red: Some(("foo bar baz".to_string(), 0)),
            blue: Some(("ba".to_string(), 1)),
            green: Some(("bar".to_string(), 2)),
            ..Opts::default()
        };

        let mut output = Vec::new();
        let input = "foo bar baz qux bar bar
foo bar
qux baz
bar
foo bar baz bar"
            .to_string();
        hl(&opts, &input, &mut output);
        let output = String::from_utf8(output).unwrap();

        let red = format!("{}", SetForegroundColor(Color::Red));
        let green = format!("{}", SetForegroundColor(Color::Green));
        let blue = format!("{}", SetForegroundColor(Color::Blue));
        let reset = format!("{}", ResetColor);

        let expected = format!(
            "{red}foo {blue}{green}ba{reset}{red}{green}r{reset}{red} {blue}ba{reset}{red}z{reset} qux {blue}{green}ba{reset}{green}r{reset} {blue}{green}ba{reset}{green}r{reset}
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
