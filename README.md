# clitools
![](https://github.com/mcutalo88/clitools/workflows/clitools/badge.svg)

## My personal collection of cli tools

This project is mostly for me to learn rust, while also yeilding
some useful tools.

### Installation
Clone this repo and use cargo to install. It is assumed that you already have the rust toolchain installed.
```
cargo install --path . --force
```

### frequency
Takes stdin and count's the frequency of matching lines, the output is sorted in descending order.
```
> echo "meow\nmeow\ncat" | frequency
2 meow
1 cat
```

### unique
Takes stdin and outputs all unique lines.
```
echo "meow\nmeow\ncat" | unique
cat
meow
```

### cslice
Takes stdin and outputs the column specified.
```
➜ cslice -h
cslice 1.0

USAGE:
    cslice [OPTIONS] <column> [skip]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --limit <limit>    Limit output [default: ]

ARGS:
    <column>    Column to slice
    <skip>      Rows to skip
```

```
echo "meow cat woof" | cslice 1
cat
```
