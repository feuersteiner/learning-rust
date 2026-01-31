fn main() {
    let name = String::from("Ada");
    let greeting = format!("Hi, {name}");

    // EX01-A: This move should break later use of `name`.
    let name_moved = &name;
    println!("name_moved = {name_moved}");
    println!("name = {name}");

    // EX01-B: Copy types should still work after assignment.
    let x = 42;
    let y = x;
    println!("x = {x}, y = {y}");

    let g2 = &greeting;
    println!("g2 = {g2}");
    println!("greeting = {greeting}");
}
