fn takes_ownership(text: String) {
    println!("takes_ownership: {text}");
}

fn borrows_text(text: &String) {
    println!("borrows_text: {text}");
}

fn gives_back(text: String) -> String {
    text
}

fn main() {
    let name = String::from("Rustacean");

    // EX02-A: This should not move `name`.
    borrows_text(name);

    // EX02-B: This should move `name` and be fixed appropriately.
    takes_ownership(name);
    println!("after takes_ownership: {name}");

    // EX02-C: Get ownership back into `name` without cloning.
    let name2 = String::from("Ferris");
    let name3 = gives_back(name2);
    println!("name3 = {name3}");
    println!("name2 = {name2}");
}
