fn smallest_type(value: i32) -> &'static str {
    let abs_value = value.abs();
    if abs_value < (i8::MAX as i32) {return "i8";}
    else if abs_value < (i16::MAX  as i32) {return "i16";}
    else {return "i32";}
}

fn smallest_type_more_rusty(value: i32) -> &'static str {
    let abs_value = value.abs();
    if abs_value < (i8::MAX as i32) { "i8"}
    else if abs_value < (i16::MAX  as i32) { "i16"}
    else { "i32"}
}

fn main() {
    println!("not rusty:");
    println!("{}",smallest_type(32));
    println!("{}",smallest_type(i8::MAX as i32 + 32 ));
    println!("{}",smallest_type(i16::MAX as i32 + 32 ));
    println!("{}",smallest_type(i16::MIN as i32 - 32 ));
    println!("more rusty:");
    println!("{}",smallest_type_more_rusty(32));
    println!("{}",smallest_type_more_rusty(i8::MAX as i32 + 32 ));
    println!("{}",smallest_type_more_rusty(i16::MAX as i32 + 32 ));
    println!("{}",smallest_type_more_rusty(i16::MIN as i32 - 32 ));
}
