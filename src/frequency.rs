use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut frequency = HashMap::new();
    let mut buffer = String::new();
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    for l in buffer.lines() {
        let item = frequency.entry(l).or_insert(0);
        *item += 1;
    }

    println!("{:?}", frequency);

    Ok(())
}

