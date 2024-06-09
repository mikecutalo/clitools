extern crate clap;
use clap::{App, Arg};

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let cslice = App::new("cslice")
        .version("1.0")
        .arg(
            Arg::with_name("column")
                .help("Column to slice")
                .required(true)
                .index(1),
        )
        .arg(Arg::with_name("skip").help("Rows to skip").index(2))
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .takes_value(true)
                .value_name("limit")
                .default_value("")
                .help("Limit output"),
        )
        .get_matches();

    let col_arg = cslice.value_of("column").unwrap_or("0");
    let skip_arg = cslice.value_of("skip").unwrap_or("0");
    let limit_arg = cslice.value_of("limit").unwrap();

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    let mut i: i128 = 0;

    for l in buffer.lines().skip(skip_arg.parse::<usize>().unwrap()) {
        if !l.is_empty() {
            println!(
                "{}",
                l.split_whitespace()
                    .nth(col_arg.parse::<usize>().unwrap())
                    .unwrap()
            );
        }

        if limit_arg != "" {
            i += 1;
            if limit_arg.parse::<i128>().unwrap() == i {
                break;
            }
        }
    }

    Ok(())
}
