# clitools

## My personal collection of cli tools

This project is mostly for me to learn rust, while also yeilding
some useful tools that I could use.

### frequency
Takes stdin and count's the frequency of matching lines, the output is sorted from
greatest to least.
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
➜ cslice --help
cslice 1.0
Mike C. <mcutalo88@gmail.com>

USAGE:
    cslice <column> [skip]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <column>    Column to extract
    <skip>      Rows to skip
```

```
echo "meow cat woof" | cslice 1
cat
```


### Other tools to build
1. sort
1. filetype
1. watch
