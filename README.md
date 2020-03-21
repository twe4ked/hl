# hl

![](https://raw.githubusercontent.com/twe4ked/hl/master/screenshot.png)

## Help

```
USAGE:
    hl [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --black <PATTERN>           Highlight PATTERN in black
    -b, --blue <PATTERN>            Highlight PATTERN in blue
    -c, --cyan <PATTERN>            Highlight PATTERN in cyan
        --dark-blue <PATTERN>       Highlight PATTERN in dark-blue
        --dark-cyan <PATTERN>       Highlight PATTERN in dark-cyan
        --dark-green <PATTERN>      Highlight PATTERN in dark-green
        --dark-grey <PATTERN>       Highlight PATTERN in dark-grey
        --dark-magenta <PATTERN>    Highlight PATTERN in dark-magenta
        --dark-red <PATTERN>        Highlight PATTERN in dark-red
        --dark-yellow <PATTERN>     Highlight PATTERN in dark-yellow
    -g, --green <PATTERN>           Highlight PATTERN in green
        --grey <PATTERN>            Highlight PATTERN in grey
    -m, --magenta <PATTERN>         Highlight PATTERN in magenta
    -r, --red <PATTERN>             Highlight PATTERN in red
    -w, --white <PATTERN>           Highlight PATTERN in white
    -y, --yellow <PATTERN>          Highlight PATTERN in yellow
```

## Example:

```
$ cargo install hl
$ hl --red foo\ bar\ baz --blue ba --green bar < fixtures/example.txt
```
