extern crate clap;
use clap::{Arg, App};

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let slicecol = App::new("slicecol")
                    .version("1.0")
                    .author("Mike C. <mcutalo88@gmail.com>")
                    .arg(Arg::with_name("column")
                        .help("Column to slice")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("skip")
                        .help("Rows to skip")
                        .index(2))
                    .get_matches();


    let col_arg = slicecol.value_of("column").unwrap_or("0");
    let skip_arg = slicecol.value_of("skip").unwrap_or("0");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    for l in buffer.lines().skip(skip_arg.parse::<usize>().unwrap()) {
        println!("{}", l.split_whitespace().nth(col_arg.parse::<usize>().unwrap()).unwrap());
    }

    Ok(())
}