mod utils;

use std::io::{self};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut unique = HashSet::new();
    let input = utils::process_stdin();

    for l in input.lines() {
        unique.insert(l);
    }

    for item in unique {
        println!("{}", item);
    }

    Ok(())
}