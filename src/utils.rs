use std::io::{self, Read};


// Simply processes stdin and return a buffer
pub fn process_stdin() -> std::string::String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer);
    return buffer;
}