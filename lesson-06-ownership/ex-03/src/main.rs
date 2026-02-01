use std::io::{self};

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|i| i.to_string())
        .collect();

    println!("{words:?}");
}
