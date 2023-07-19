use std::env;

fn main() {
    let args = parse_args();

    match &*args.function {
        "a" => a(),
        "b" => b(),
        "c" => c(),
        s => eprintln!("Unknown function: {}", s),
    }
}

struct Arguments {
    function: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        eprintln!(
            "Error wrong number of arguments: expected {}, got {}.",
            1,
            args.len()
        );

        std::process::exit(1);
    }

    Arguments {
        function: args[0].clone(),
    }
}

fn a() {
    let mut i = 1;
    loop {
        i *= 10; // panic: attempt to multiplywith overflow
                 // (but only in debug builds!)
        println!("{}", i);
    }

    // $ cargo run
    // 10
    // 100
    // 1000
    // 10000
    // 100000
    // 1000000
    // 10000000
    // 100000000
    // 1000000000
    // thread 'main' panicked at 'attempt to multiply with overflow', src/main.rs:4:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // $ cargo run --release | head -n 20
    // 10
    // 100
    // 1000
    // 10000
    // 100000
    // 1000000
    // 10000000
    // 100000000
    // 1000000000
    // 1410065408
    // 1215752192
    // -727379968
    // 1316134912
    // 276447232
    // -1530494976
    // 1874919424
    // 1569325056
    // -1486618624
    // -1981284352
    // 1661992960
    // thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', library/std/src/io/stdio.rs:1008:9
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}

fn b() {
    let mut i: i32 = 1;
    loop {
        // panic: multiplication overflow (in any build)
        i = i.checked_mul(10).expect("multiplication overflowed");
    }

    // $ target/release/datatypes b
    // thread 'main' panicked at 'multiplication overflowed', src/main.rs:56:31
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}

fn c() {
    // Get out command-line arguments as a vector of Strings.
    let languages: Vec<String> = std::env::args().skip(2).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_as() {
        assert_eq!(10_i8 as u16, 10_u16); // in-range
        assert_eq!(2525_u16 as i16, 2525_i16); // in-range

        assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
        assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

        // Conversions that are out of range for the destination
        // produce value that are equivalent to the original modulo 2^N,
        // where N is the width of the destination in bits.
        // This is sometimes called "truncation".
        assert_eq!(1000_i16 as u8, 232_u8);

        assert_eq!(65535_u32 as i16, -1_i16);

        assert_eq!(-1_i8 as u8, 255_u8);
        assert_eq!(255_u8 as i8, -1_i8);
    }

    #[test]
    fn test_int_impl() {
        assert_eq!(2_u16.pow(4), 16); // exponentiation
        assert_eq!((-4_i32).abs(), 4); // absolute value
        assert_eq!(0b101101_u8.count_ones(), 4); // population count (bit count)
    }

    #[test]
    fn test_err() {
        // println!("{}", (-4).abs());
        // error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`
        //   --> src/main.rs:36:29
        //    |
        // 36 |         println!("{}", (-4).abs());
        //    |                             ^^^

        // For more information about this error, try `rustc --explain E0689`.
        // error: could not compile `datatypes` due to previous error

        assert_eq!((-4_i32).abs(), 4);
        assert_eq!(i32::abs(-4), 4);

        // It should be noted here that method calls have a higher precedence than
        // unary preposition operators. For this reason, special care should be taken
        // when making method calls on negative values. In the second assertion above,
        // `-4_i32` is enclosed in parentheses, but if we omit the parentheses and
        // use `-4_i32.abs()`, we would call the `abs` method on the positive value `4`
        // and apply the sign inversion operator to it, resulting in `-4`.
        assert_eq!(-4_i32.abs(), -4);
    }

    #[test]
    fn test_checked_ops() {
        // The sum of 10 an d20 can be represented as a u8.
        assert_eq!(10_u8.checked_add(20), Some(30));

        // Unfortunately, the sum of 100 and 200 cannot.
        assert_eq!(100_u8.checked_add(200), None);

        // Do the addition; ppanic if it overflows
        // let x: u8 = 100;
        // let y: u8 = 200;
        // let sum = x.checked_add(y).unwrap();
        // thread 'tests::test_checked_ops' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:157:36

        // Oddly, signed division can overflow too, in one particular case.
        // A signed n-bit type can represent -2^{n-1}, but not 2^{n-1}.
        assert_eq!((-128_i8).checked_div(-1), None);
        assert_eq!((127_i8).checked_div(-1), Some(-127));
    }

    #[test]
    fn test_wrapping_mul() {
        // The first product can be represented as a u16;
        // the second cannot, so we get 250000 module 2^{16}
        assert_eq!(100_u16.wrapping_mul(200), 20000);
        assert_eq!(500_u16.wrapping_mul(500), 53392);

        // Operations on signed types may wrap to negative values.
        assert_eq!(500_i16.wrapping_mul(500), -12144);

        // In bitwise shift operations, the shift distance
        // is wrapped to fall within the size of the value.
        // So a shift of 17 bits in a 16-bit type is
        // a shift of 1.
        assert_eq!(5_i16.wrapping_shl(17), 10);
        assert_eq!(5_i16.wrapping_shl(1), 10);
        assert_eq!(5_i16.wrapping_shl(2), 20);
        assert_eq!(5_i16.wrapping_shl(3), 40);
    }

    #[test]
    fn test_saturating_add_sub() {
        assert_eq!(32760_i16.saturating_add(10), 32767);
        assert_eq!((-32760_i16).saturating_sub(10), -32768);
    }

    #[test]
    fn test_overflowing_add_sub() {
        assert_eq!(255_u8.overflowing_sub(2), (253, false));
        assert_eq!(255_u8.overflowing_sub(3), (252, false));
        assert_eq!(255_u8.overflowing_add(2), (1, true));
        assert_eq!(255_u8.overflowing_add(3), (2, true));

        // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
        assert_eq!(5_u16.overflowing_shl(17), (10, true));
    }

    #[test]
    fn test_floating_points() {
        assert!((-1. / f32::INFINITY).is_sign_negative());
        assert_eq!(-f32::MIN, f32::MAX);

        assert_eq!((-1. / f32::INFINITY), -0.);
        assert_eq!(-f32::MIN, 3.4028235e38);
        assert_eq!(-f32::MAX, -3.4028235e38);

        assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
        assert_eq!((-1.01f64).floor(), -2.0);
        assert_eq!(5f32.sqrt(), 2.236068);

        assert_eq!((2.0_f64).sqrt(), 1.4142135623730951);
        assert_eq!(f64::sqrt(2.0), 1.4142135623730951);
    }

    #[test]
    fn test_bool() {
        assert_eq!(false as i32, 0);
        assert_eq!(true as i32, 1);
    }

    #[test]
    fn test_character() {
        assert_eq!('\x2A', '*');
        assert_eq!('\u{2A}', '*');

        assert_eq!('*' as i32, 42);
        assert_eq!('\u{CA0}' as u16, 0xca0);
        assert_eq!('\u{CA0}' as i8, -0x60); // U+0CAO truncated to eight bits, signed
    }

    #[test]
    fn test_tuple() {
        // fn slit_at(&self, mid: usize) -> (&str, &str);
        // pub const fn split_at(&self, mid: usize) -> (&[T], &[T]);
        let v = [1, 2, 3, 4, 5, 6];
        let (left, right) = v.split_at(3);
        assert_eq!(left, [1, 2, 3]);
        assert_eq!(right, [4, 5, 6]);

        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");

        let tmp = text.split_at(21);
        assert_eq!(tmp.0, "I see the eigenvalue ");
        assert_eq!(tmp.1, "in thine eye");

        // fn swap<T>(x: &mut T, y: &mut T) -> () {}
    }

    #[test]
    fn test_pointer() {
        let tuple = ((0, 0), (1440, 900));
        assert_eq!(tuple.0, (0, 0));

        let t = (12, "eggs");
        let b = Box::new(t); // allocate a tuple in the heap
        assert_eq!(b.0, 12);
        assert_eq!(b.1, "eggs");

        // *mut T, *const T: raw pointer, unsafe

        // Array, Vecotr, Slice
        // let array: [usize; 2] = [0, 1];
        let array = [0, 1];
        assert_eq!(array.len(), 2);
        assert_eq!(array[0], 0);
        assert_eq!(array[1], 1);

        let mut vector = Vec::new();
        vector.push(0);
        vector.push(1);
        assert_eq!(array.len(), 2);
        assert_eq!(array[0], 0);
        assert_eq!(array[1], 1);

        let slice = &[0, 1];
        assert_eq!(slice.len(), 2);
        assert_eq!(array[0], 0);
        assert_eq!(array[1], 1);
    }

    #[test]
    fn test_array() {
        let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
        let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
        assert_eq!(lazy_caterer[3], 7);
        assert_eq!(taxonomy.len(), 3);

        let mut sieve = [true; 10000];
        for i in 2..100 {
            if sieve[i] {
                let mut j = i * i;
                while j < 10000 {
                    sieve[j] = false;
                    j += i;
                }
            }
        }

        assert!(sieve[211]);
        assert!(!sieve[9876]);

        let mut chaos = [3, 5, 4, 1, 2];
        chaos.sort();
        assert_eq!(chaos, [1, 2, 3, 4, 5]);

        let mut primes = vec![2, 3, 5, 7];
        assert_eq!(primes.iter().product::<i32>(), 210);
        primes.push(11);
        primes.push(13);
        assert_eq!(primes.iter().product::<i32>(), 30030);

        fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
            vec![0; rows * cols]
        }

        let pixel_buffer = new_pixel_buffer(2, 2);
        assert_eq!(pixel_buffer.len(), 4);
        assert_eq!(pixel_buffer[0], 0);
        assert_eq!(pixel_buffer[1], 0);
        assert_eq!(pixel_buffer[2], 0);
        assert_eq!(pixel_buffer[3], 0);

        let mut pal = Vec::new();
        pal.push("step");
        pal.push("on");
        pal.push("no");
        pal.push("pets");
        assert_eq!(pal, vec!["step", "on", "no", "pets"]);

        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, [0, 1, 2, 3, 4]);

        // A palindrome!
        let mut palindrone = vec!["a man", "a plan", "a canal", "panama"];
        palindrone.reverse();
        // Reasonable yet disappointing:
        assert_eq!(palindrone, vec!["panama", "a canal", "a plan", "a man"]);

        let reverse: Vec<_> = palindrone
            .clone()
            .into_iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect();
        assert_eq!(reverse, vec!["amanap", "lanac a", "nalp a", "nam a"]);

        let reverse: Vec<String> = palindrone
            .clone()
            .into_iter()
            .map(|s| s.chars().rev().collect())
            .collect();
        assert_eq!(reverse, vec!["amanap", "lanac a", "nalp a", "nam a"]);
    }

    #[test]
    fn test_vec_with_capacity() {
        let mut v = Vec::with_capacity(2);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 2);

        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);

        v.push(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 4);
        // // Typically prints "capacity is now 4":
        // println!("capacity is now {}", v.capacity());
    }

    #[test]
    fn test_vec_insert_remove() {
        let mut v = vec![10, 20, 30, 40, 50];

        // Make the element at index 3 be 35.
        v.insert(3, 35);
        assert_eq!(v, [10, 20, 30, 35, 40, 50]);

        // Remove the element at index 1.
        v.remove(1);
        assert_eq!(v, [10, 30, 35, 40, 50]);
    }

    #[test]
    fn test_vec_pop() {
        let mut v = vec!["Snow Puff", "Glass Gem"];
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some("Glass Gem"));
        assert_eq!(v.pop(), Some("Snow Puff"));
        assert_eq!(v.len(), 0);
        assert_eq!(v.pop(), None);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_slice() {
        let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
        let a: [f64; 4] = [0.0, -0.707, 1.0, -0.707];

        let sv: &[f64] = &v;
        let sa: &[f64] = &a;

        // Stack frame
        // * v
        //   * 所有権のあるポインタ[1]
        //   * 容量:4
        //   * 長さ:4
        // * a
        //   * 0.0 [2]
        //   * -0.700
        //   * -1.0
        //   * 0.707
        // * sa
        //   * 所有権のあるポインタ [2]
        //   * 4
        // * sv
        //   * 所有権のあるポインタ [1]
        //   * 4
        // Heap
        // * 0.0 [1]
        // * 0.707
        // * 1.0
        // * 0.707

        fn sum(n: &[f64]) -> f64 {
            let mut result = 0.0;
            for elt in n {
                result += elt;
            }
            result
        }

        assert_eq!(sum(&v), 2.4139999999999997); // works on arrays
        assert_eq!(sum(&a), -0.4139999999999999); // works on vectors

        assert_eq!(sum(sv), 2.4139999999999997);
        assert_eq!(sum(sa), -0.4139999999999999);

        assert_eq!(v.len(), 4);
        assert_eq!(a.len(), 4);

        assert_eq!(sv.len(), 4);
        assert_eq!(sa.len(), 4);

        assert_eq!(sum(&v[0..2]), 0.707);
        assert_eq!(sum(&a[0..2]), -0.707);
        assert_eq!(sum(&sv[0..2]), 0.707);
        assert_eq!(sum(&sa[0..2]), -0.707);
    }
}
