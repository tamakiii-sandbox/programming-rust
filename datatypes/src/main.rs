fn main() {
    println!("Hello, world!");
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
}
