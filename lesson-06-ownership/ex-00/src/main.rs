fn main() {
    // Exercise 0 - Step 2: ownership via move
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {s2}");
    
    // TODO: after you fill the above, try using s1 here and see the error.
    println!("s1 = {s1}");
    // console:
    // borrow of moved value: `s1`
    // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
}
