use std::env;

fn main() {
    let args = parse_args();

    match &*args.function {
        "a" => a(),
        "b" => b(),
        s => eprintln!("Unknown function: {}", s),
    }
}

struct Arguments {
    function: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
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
}
