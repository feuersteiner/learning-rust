fn identify_value(input: &str) -> &'static str {
    if str::parse::<i32>(input).is_ok() {
        "integer"
    } else if str::parse::<f32>(input).is_ok() {
        "float"
    } else if str::parse::<bool>(input).is_ok() {
        "boolean"
    } else {
        "string"
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", identify_value("12"));
    println!("{}", identify_value("12.1"));
    println!("{}", identify_value("hello"));
    println!("{}", identify_value("false"));
    println!("{}", identify_value("0"));
}
