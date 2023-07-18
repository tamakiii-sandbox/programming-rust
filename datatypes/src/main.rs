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
}
