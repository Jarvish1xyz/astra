use std::io::{self, Write};

pub fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim().to_string()
}