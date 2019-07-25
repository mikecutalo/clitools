use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut unique = HashSet::new();
    let mut buffer = String::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    for l in buffer.lines() {
        unique.insert(l);
    }

    for item in unique {
        println!("{}", item);
    }

    Ok(())
}