use std::f64::INFINITY;

fn main() {
    println!("Hello, world!");
}

fn square_loop(mut x: f64, t: u64) -> f64 {
    let mut n = 0;
    // loop {
    while n < t {
        x = x * x;
        n += 1;
    }
    x
}

fn square_add_loop(c: f64, t: u64) -> f64 {
    let mut x = 0.;
    let mut n = 0;
    // loop {
    while n < t {
        x = x * x + c;
        n += 1;
    }
    x
}

#[test]
fn test_square_loop() {
    assert_eq!(square_loop(1.0, 1), 1.0);
    assert_eq!(square_loop(1.0, 10), 1.0);
    assert_eq!(square_loop(1.0, 100), 1.0);

    assert_eq!(square_loop(1.01, 1), 1.0201);
    assert_eq!(square_loop(1.01, 10), 26612.566117305338);
    assert_eq!(square_loop(1.01, 100), INFINITY);

    assert_eq!(square_loop(1.1, 1), 1.2100000000000002);
    assert_eq!(square_loop(1.1, 10), 2.432817896953651e42);
    assert_eq!(square_loop(1.1, 100), INFINITY);

    assert_eq!(square_loop(2.0, 1), 4.0);
    assert_eq!(square_loop(2.0, 10), INFINITY);
    assert_eq!(square_loop(2.0, 100), INFINITY);
}

#[test]
fn test_square_add_loop() {
    assert_eq!(square_add_loop(0.25, 1), 0.25);
    assert_eq!(square_add_loop(0.25, 10), 0.430549106102856);
    assert_eq!(square_add_loop(0.25, 100), 0.4906042201293854);
    assert_eq!(square_add_loop(0.25, 100000), 0.49999000132792415);

    assert_eq!(square_add_loop(0.251, 100), 476.37172174929185);
    assert_eq!(square_add_loop(0.251, 1000), INFINITY);
}
