# hl

## Help

```
USAGE:
    hl [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --blue <PATTERN>       Highlight PATTERN in blue
    -c, --cyan <PATTERN>       Highlight PATTERN in cyan
    -g, --green <PATTERN>      Highlight PATTERN in green
    -m, --magenta <PATTERN>    Highlight PATTERN in magenta
    -r, --red <PATTERN>        Highlight PATTERN in red
    -w, --white <PATTERN>      Highlight PATTERN in white
    -y, --yellow <PATTERN>     Highlight PATTERN in yellow
```

## Example:

```
$ make
$ cat fixtures/example.txt | ./hl --red foo\ bar\ baz --blue ba --green bar
```
