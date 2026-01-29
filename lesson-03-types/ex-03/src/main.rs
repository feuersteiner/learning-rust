fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    return ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
}

fn mid(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    let (x1, y1) = a;
    let (x2, y2) = b;

    return ((x2 + x1) / 2.0, (y2 + y1) / 2.0);
}

fn main() {
    println!("Hello, world!");
    // distance
    assert_eq!(distance((0.0, 0.0), (3.0, 4.0)), 5.0);
    assert_eq!(distance((0.0, 0.0), (0.0, 0.0)), 0.0);
    assert_eq!(distance((-3.0, -4.0), (0.0, 0.0)), 5.0);
    assert_eq!(distance((1.0, 1.0), (4.0, 5.0)), 5.0);
    // midpoint
    assert_eq!(mid((0.0, 0.0), (4.0, 4.0)), (2.0, 2.0));
    assert_eq!(mid((-2.0, -2.0), (2.0, 2.0)), (0.0, 0.0));
    assert_eq!(mid((0.0, 0.0), (0.0, 0.0)), (0.0, 0.0));
    assert_eq!(mid((1.0, 3.0), (5.0, 7.0)), (3.0, 5.0));
}
