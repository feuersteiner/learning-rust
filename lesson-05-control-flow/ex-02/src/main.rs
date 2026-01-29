fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        false
    } else if n == 2 || n == 3 {
        true
    } else {
        let sqr = (n as f64).sqrt() as u32;
        for i in 2..=sqr {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

fn nth_prime(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        let mut prime_counter = 0;
        let mut last_prime = 0;
        let mut i = 2;

        loop {
            if is_prime(i) {
                prime_counter += 1;
                last_prime = i;
            }

            if prime_counter == n {
                break last_prime;
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(23), true);
    assert_eq!(nth_prime(1), 2);
    assert_eq!(nth_prime(5), 11);
    assert_eq!(nth_prime(10), 29);
    assert_eq!(nth_prime(100), 541);
}
