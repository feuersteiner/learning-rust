use std::io::{self};

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let name: String = input.trim().to_string();
    println!("Hello {name}!\nhow old are you?");
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let age: i32 = str::parse::<i32>(input.trim()).unwrap();
    let coming_age: i32 = age + 1;
    println!("next year you'll be {coming_age} years old!");
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let floater: f64 = str::parse::<f64>(input.trim()).unwrap();
    println!("oh {floater} is fun, here's how it looks trimmed {floater:.2}!");
}
