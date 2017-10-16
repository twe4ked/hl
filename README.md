# hl

## Help

```
Usage: hl [--COLOR=PATTERN ...]
    -r PATTERN, --red=PATTERN        Highlight PATTERN in red
    -g PATTERN, --green=PATTERN      Highlight PATTERN in green
    -y PATTERN, --yellow=PATTERN     Highlight PATTERN in yellow
    -b PATTERN, --blue=PATTERN       Highlight PATTERN in blue
    -m PATTERN, --magenta=PATTERN    Highlight PATTERN in magenta
    -c PATTERN, --cyan=PATTERN       Highlight PATTERN in cyan
    -w PATTERN, --white=PATTERN      Highlight PATTERN in white
    -h, --help                       Display help
```

## Example:

```
$ make
$ cat fixtures/example.txt | ./hl --red foo\ bar\ baz --blue ba --green bar
```
