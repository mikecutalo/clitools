use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut frequency = HashMap::new();
    let mut buffer = String::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    for l in buffer.lines() {
        if !l.is_empty() {
            *frequency.entry(l).or_insert(0) += 1;
        }
    }

    // Convert to vec so we can sort
    let mut count_vec: Vec<_> = frequency.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for item in count_vec {
        println!("{} {}", item.1, item.0);
    }

    Ok(())
}

