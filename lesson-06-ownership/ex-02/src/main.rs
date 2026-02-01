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
    borrows_text(&name);

    // EX02-B: This should move `name` and be fixed appropriately.
    takes_ownership(name);
    println!("after takes_ownership: {name}");
}
