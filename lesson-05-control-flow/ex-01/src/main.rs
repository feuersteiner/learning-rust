fn fizzbuzz(n: u32) -> String {
    let fizz = n % 3 == 0;
    let buzz = n % 5 == 0;

    if fizz && buzz {
        "FizzBuzz".to_string()
    } else if fizz {
        "Fizz".to_string()
    } else if buzz {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

fn main() {
    for i in 1..21 {
        if i % 7 == 0 {
            continue;
        }
        println!("{}",fizzbuzz(i));
    }
}
