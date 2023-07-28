use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    sort_works(&mut table);
    show(&table);
    assert_eq!(table["Gesualdo"][1], "many madrigals");

    // show(table);
    // assert_eq!(table["Gesualdo"][0], "many madrigals");
    // error[E0382]: borrow of moved value: `table`
    //   --> src/main.rs:29:16
    //    |
    // 5  |     let mut table = Table::new();
    //    |         --------- move occurs because `table` has type `HashMap<String, Vec<String>>`, which does not implement the `Copy` trait
    // ...
    // 28 |     show(table);
    //    |          ----- value moved here
    // 29 |     assert_eq!(table["Gesualdo"][0], "many madrigals");
    //    |                ^^^^^ value borrowed here after move
    //    |
    // note: consider changing this parameter type in function `show` to borrow instead if owning the value isn't necessary
    //   --> src/main.rs:32:16
    //    |
    // 32 | fn show(table: Table) {
    //    |    ----        ^^^^^ this parameter takes ownership of the value
    //    |    |
    //    |    in this function
    // help: consider cloning the value if the performance cost is acceptable
    //    |
    // 28 |     show(table.clone());
    //    |               ++++++++

    // For more information about this error, try `rustc --explain E0382`.

    // shared reference:
    //   &e returns shared reference for value e.
    //   if type of e was T, then type of &e is &T (ref T)
    //   shared reference is Copy type

    // mutable reference:
    //   Using a mutable reference to a value, the value can be read or changed.
    //   However, a mutable reference to the same value cannot be used simultaneously with any other reference (either a shared reference or a mutable reference).
    //   &mut e returns a variable reference to the value e.
    //   The type of this reference is &mut T (pronounced "ref mute T"). Variable references are not of type Copy.
}

fn show(table: &Table) {
    // table: &HashMap<String, Vec<String>>
    for (artist, works) in table {
        // artist: &String
        // works: &Vec<String>
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_reference() {
        // Back to Rust code from this point onward.
        let x = 10;
        let r = &x; // &x is shared reference to x
        assert!(*r == 10); // explicitly dereference r

        let mut y = 32;
        let m = &mut y; // & mut y is a mutable reference to y
        *m += 32; // explicitly dereference m to set y's value and to see y's new value
        assert!(*m == 64);

        struct Anime {
            name: &'static str,
            bechdel_pass: bool,
        }

        let aria = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };
        let anime_ref = &aria;
        assert_eq!(anime_ref.name, "Aria: The Animation");
        assert!(anime_ref.bechdel_pass);

        // Equivalent to the above, but with the deference writtenout:
        assert_eq!((*anime_ref).name, "Aria: The Animation");
        assert!((*anime_ref).bechdel_pass);

        let mut v = vec![1973, 1968];
        v.sort(); // implicitly borrows a mutable reference to v
        (&mut v).sort(); // equivalent; but more verbose
    }

    #[test]
    fn test_assign_reference() {
        let x = 10;
        let y = 20;
        let mut r = &x;

        let b = true;
        // let b = false;
        if b {
            r = &y;
        }

        assert!(*r == 10 || *r == 20);

        // 5.2.3
        #[allow(dead_code)]
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 1000, y: 729 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;

        assert_eq!(rrr.y, 729);
    }

    #[test]
    fn test_compare_reference() {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);

        assert!(rx == ry); // their referents are equal
        assert!(!std::ptr::eq(rx, ry)); // but occupy different address

        // assert!(rx == rrx); // error: type mismatch: `&i32` vs `&&i32`
        // error[E0277]: can't compare `{integer}` with `&{integer}`
        //    --> src/main.rs:171:20
        //     |
        // 171 |         assert!(rx == rrx); // error: type mismatch: `&i32` vs `&&i32`
        //     |                    ^^ no implementation for `{integer} == &{integer}`
        //     |
        //     = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
        //     = help: the following other types implement trait `PartialEq<Rhs>`:
        //               f32
        //               f64
        //               i128
        //               i16
        //               i32
        //               i64
        //               i8
        //               isize
        //             and 6 others
        //     = note: required for `&{integer}` to implement `PartialEq<&&{integer}>`

        // For more information about this error, try `rustc --explain E0277`.

        assert!(rx == *rrx); // this is okay

        // 5.2.6
        fn factorial(n: usize) -> usize {
            (1..n + 1).product()
        }

        let r = &factorial(6);
        // Arithmetic opperators can see through one level of references.
        assert_eq!(r + &1009, 1729);

        // 5.2.7
        // Fat pointers
        // 1. Reference to slice
        // 2. trait object
    }

    #[test]
    fn test_safety_of_reference() {
        {
            // let r;
            // {
            //     let x = 1;
            //     r = &x;
            // }
            // assert_eq!(&r, 1); // bad: read memory `x` used to occupy
            // error[E0277]: can't compare `&&{integer}` with `{integer}`
            //    --> src/main.rs:218:13
            //     |
            // 218 |             assert_eq!(&r, 1); // bad: read memory `x` used to occupy
            //     |             ^^^^^^^^^^^^^^^^^ no implementation for `&&{integer} == {integer}`
            //     |
            //     = help: the trait `PartialEq<{integer}>` is not implemented for `&&{integer}`
            //     = help: the following other types implement trait `PartialEq<Rhs>`:
            //               f32
            //               f64
            //               i128
            //               i16
            //               i32
            //               i64
            //               i8
            //               isize
            //             and 6 others
            //     = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

            // For more information about this error, try `rustc --explain E0277`.
        }

        // 5.3.2
        // This code has several problems, and doesn't compile.
        // static mut STASH: &i32;
        // fn f(p: &i32) {
        //     STASH = p;
        // }
        // error: free static item without body
        //    --> src/main.rs:243:9
        //     |
        // 243 |         static mut STASH: &i32;
        //     |         ^^^^^^^^^^^^^^^^^^^^^^-
        //     |                               |
        //     |                               help: provide a definition for the static: `= <expr>;`

        // error: could not compile `reference` due to previous error

        // static mut STASH: &i32 = &128;
        // fn f<'a>(p: &'a i32) {
        //     // 'a: "tick A"
        //     // fn f(p: &i32) {
        //     unsafe {
        //         STASH = p;
        //         // error: lifetime may not live long enough
        //         //    --> src/main.rs:262:17
        //         //     |
        //         // 258 |         fn f<'a>(p: &'a i32) {
        //         //     |              -- lifetime `'a` defined here
        //         // ...
        //         // 262 |                 STASH = p;
        //         //     |                 ^^^^^^^^^ assignment requires that `'a` must outlive `'static`

        //         // error: could not compile `reference` due to previous error
        //     }
        // }

        #[allow(dead_code)]
        {
            static mut STASH: &i32 = &128;
            // fn f(p: &'static i32) {
            fn f<'a>(p: &'static i32) {
                unsafe {
                    STASH = p;
                }
            }

            static WORTH_POINTING_AT: i32 = 1000;
            f(&WORTH_POINTING_AT);
        }

        // 5.3.3
        // This could be written more briefly: fn g(p: &i32),
        // but let's write out the lifetimes for now.
        #[allow(unused_variables)]
        {
            fn g<'a>(p: &'a i32) {}

            let x = 10;
            g(&x);
        }

        // 5.3.4
        // v should have at least one element.
        fn smallest(v: &[i32]) -> &i32 {
            let mut s = &v[0];

            for r in &v[1..] {
                if &r < &s {
                    s = r;
                }
            }
            s
        }

        let s;
        {
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);
            assert_eq!(*s, 0); // fine: parabola still alive
        }
        // assert_eq!(*s, 0); // bad: points to element of dropped array
        // error[E0597]: `parabola` does not live long enough
        //    --> src/main.rs:317:26
        //     |
        // 316 |             let parabola = [9, 4, 1, 0, 1, 4, 9];
        //     |                 -------- binding `parabola` declared here
        // 317 |             s = smallest(&parabola);
        //     |                          ^^^^^^^^^ borrowed value does not live long enough
        // 318 |             assert_eq!(*s, 0);
        // 319 |         }
        //     |         - `parabola` dropped here while still borrowed
        // 320 |         assert_eq!(*s, 0); // bad: points to element of dropped array
        //     |         ----------------- borrow later used here

        // For more information about this error, try `rustc --explain E0597`.

        // 5.3.5
        // This does not compile.
        // struct S {
        //     r: &i32,
        //     // error[E0106]: missing lifetime specifier
        //     //    --> src/main.rs:339:16
        //     //     |
        //     // 339 |             r: &i32,
        //     //     |                ^ expected named lifetime parameter
        //     //     |
        //     // help: consider introducing a named lifetime parameter
        //     //     |
        //     // 338 ~         struct S<'a> {
        //     // 339 ~             r: &'a i32,
        // }

        // let s;
        // {
        //     let x = 10;
        //     s = S { r: &x };
        //     // assert_eq!(&s.r, 10);
        // }
        // // assert_eq!(&s.r, 10); // bad: reads from dropped `x`
        // // error[E0277]: can't compare `&&i32` with `{integer}`
        // //    --> src/main.rs:348:9
        // //     |
        // // 348 |         assert_eq!(&s.r, 10); // bad: reads from dropped `x`
        // //     |         ^^^^^^^^^^^^^^^^^^^^ no implementation for `&&i32 == {integer}`
        // //     |
        // //     = help: the trait `PartialEq<{integer}>` is not implemented for `&&i32`
        // //     = help: the trait `PartialEq` is implemented for `i32`
        // //     = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

        // // Some errors have detailed explanations: E0106, E0277.
        // // For more information about an error, try `rustc --explain E0106`.

        // struct S {
        //     r: &'static i32,
        // }

        struct S<'a> {
            r: &'a i32,
        }

        // struct D {
        //     s: D, // not adequate
        // }
        // error[E0072]: recursive type `D` has infinite size
        //    --> src/main.rs:380:9
        //     |
        // 380 |         struct D {
        //     |         ^^^^^^^^
        // 381 |             s: D, // not adequate
        //     |                - recursive without indirection
        //     |
        // help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
        //     |
        // 381 |             s: Box<D>, // not adequate
        //     |                ++++ +

        // For more information about this error, try `rustc --explain E0072`.
        // error: could not compile `reference` due to previous error

        // struct D {
        //     s: S<'static>,
        // }

        struct D<'a> {
            s: S<'a>,
        }

        let s;
        {
            let x = 10;
            s = S { r: &x };
            let d = D { s: s };
            assert_eq!(*d.s.r, 10);
            // assert_eq!(*d.s.r == 10);
            // error: unexpected end of macro invocation
            //    --> src/main.rs:412:36
            //     |
            // 412 |             assert_eq!(*d.s.r == 10);
            //     |                                    ^ missing tokens in macro arguments
            //     |
            // note: while trying to match `,`
            //    --> /Users/tamakiii/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/macros/mod.rs:37:16
            //     |
            // 37  |     ($left:expr, $right:expr $(,)?) => {
            //     |                ^

            // error: could not compile `reference` due to previous erro

            // $ cat /Users/tamakiii/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/macros/mod.rs | head -n 40 | tail -n 26
            // /// Asserts that two expressions are equal to each other (using [`PartialEq`]).
            // ///
            // /// On panic, this macro will print the values of the expressions with their
            // /// debug representations.
            // ///
            // /// Like [`assert!`], this macro has a second form, where a custom
            // /// panic message can be provided.
            // ///
            // /// # Examples
            // ///
            // /// ```
            // /// let a = 3;
            // /// let b = 1 + 2;
            // /// assert_eq!(a, b);
            // ///
            // /// assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
            // /// ```
            // #[macro_export]
            // #[stable(feature = "rust1", since = "1.0.0")]
            // #[cfg_attr(not(test), rustc_diagnostic_item = "assert_eq_macro")]
            // #[allow_internal_unstable(core_panic)]
            // macro_rules! assert_eq {
            //     ($left:expr, $right:expr $(,)?) => {
            //         match (&$left, &$right) {
            //             (left_val, right_val) => {
            //                 if !(*left_val == *right_val) {
        }

        struct Record<'i> {
            i: &'i [u8],
        }

        fn parse_record<'i>(input: &'i [u8]) -> Record<'i> {
            // let mut result: Vec<&'i u8> = vec![];
            let mut result = vec![];
            for i in input {
                result.push(i);
            }
            Record { i: input }
        }

        let input = [10, 11, 12];
        let record = parse_record(&input);
        assert_eq!(record.i, [10, 11, 12]);

        // 5.3.6
        // {
        //     struct S<'a> {
        //         x: &'a i32,
        //         y: &'a i32,
        //     }

        //     let x = 10;
        //     let r;
        //     {
        //         let y = 20;
        //         {
        //             let s = S { x: &x, y: &y };
        //             r = s.x;
        //         }
        //     }

        //     println!("{}", r);

        //     // error[E0597]: `y` does not live long enough
        //     //    --> src/main.rs:486:43
        //     //     |
        //     // 484 |                 let y = 20;
        //     //     |                     - binding `y` declared here
        //     // 485 |                 {
        //     // 486 |                     let s = S { x: &x, y: &y };
        //     //     |                                           ^^ borrowed value does not live long enough
        //     // ...
        //     // 489 |             }
        //     //     |             - `y` dropped here while still borrowed
        //     // 490 |
        //     // 491 |             println!("{}", r);
        //     //     |                            - borrow later used here

        //     // For more information about this error, try `rustc --explain E0597`.
        // }

        #[allow(dead_code)]
        {
            struct S<'a, 'b> {
                x: &'a i32,
                y: &'b i32,
            }

            let x = 10;
            let r;
            {
                let y = 20;
                {
                    let s = S { x: &x, y: &y };
                    r = s.x;
                }
            }

            assert_eq!(*r, 10);
        }

        // fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {
        //     r
        // }

        // 5.3.7
        {
            struct S<'a, 'b> {
                x: &'a i32,
                y: &'b i32,
            }

            // fn sum_r_xy(r: &i32, s: S) -> i32 {
            fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, s: S<'b, 'c>) -> i32 {
                r + s.x + s.y
            }

            let s = S { x: &10, y: &20 };
            let v = sum_r_xy(&1, s);
            assert_eq!(v, 31);

            // fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
            fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32) {
                (&point[0], &point[2])
            }

            let point = first_third(&[1, 2, 3]);
            assert_eq!(point.0, &1);
            assert_eq!(point.1, &3);

            // self
            struct StringTable {
                elements: Vec<String>,
            }

            impl StringTable {
                // fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
                fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
                    for i in 0..self.elements.len() {
                        if self.elements[i].starts_with(prefix) {
                            return Some(&self.elements[i]);
                        }
                    }
                    None
                }
            }

            let table = StringTable {
                elements: vec!["Hello".to_string(), "World".to_string(), "Rust".to_string()],
            };

            assert_eq!(table.find_by_prefix("Hel"), Some(&"Hello".to_string()));
            assert_eq!(table.find_by_prefix("R"), Some(&"Rust".to_string()));
            assert_eq!(table.find_by_prefix("Test"), None);
        }
    }

    #[test]
    fn test_sharing_and_changes() {
        // 5.4
        {
            // let v = vec![4, 8, 19, 27, 34, 10];
            // let r = &v;
            // let aside = v; // move vector to aside
            // r[0]; // bad: uses `v`, which is now undefined
            // error[E0505]: cannot move out of `v` because it is borrowed
            //    --> src/main.rs:593:25
            //     |
            // 591 |             let v = vec![4, 8, 19, 27, 34, 10];
            //     |                 - binding `v` declared here
            // 592 |             let r = &v;
            //     |                     -- borrow of `v` occurs here
            // 593 |             let aside = v; // move vector to aside
            //     |                         ^ move out of `v` occurs here
            // 594 |             r[0]; // bad: uses `v`, which is now undefined
            //     |             - borrow later used here

            // For more information about this error, try `rustc --explain E0505`.

            let v = vec![4, 8, 19, 27, 34, 10];
            {
                let r = &v;
                r[0]; // ok: vector is still there
            }
            let aside = v;
            assert_eq!(aside.len(), 6);

            /// This is an inflexible (and totally non-optimized) version of the extend_from_slice method provided for
            /// vectors in the standard library. This function can be used to create vectors from slices of other vectors or arrays.
            ///
            /// [`Vec::extend_from_slice`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_slice
            ///
            /// ```
            /// let mut v = vec![1, 2];
            /// v.extend_from_slice(&[3, 4, 5]);
            ///
            /// assert_eq!(v, &[1, 2, 3, 4, 5]);
            /// ```
            fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
                for elt in slice {
                    vec.push(*elt);
                }
            }

            let mut wave = Vec::new();
            let head = vec![0.0, 1.0];
            let tail = [0.0, -1.0];

            extend(&mut wave, &head); // extend wave with another vector
            extend(&mut wave, &tail); // extend wave with an array

            assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

            // extend(&mut wave, &wave);
            // assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0,]);
            // error[E0502]: cannot borrow `wave` as immutable because it is also borrowed as mutable
            //    --> src/main.rs:632:31
            //     |
            // 632 |             extend(&mut wave, &wave);
            //     |             ------ ---------  ^^^^^ immutable borrow occurs here
            //     |             |      |
            //     |             |      mutable borrow occurs here
            //     |             mutable borrow later used by call

            // For more information about this error, try `rustc --explain E0502`.

            // **Shared access is read-only access. **
            // The value borrowed by a shared reference can only be read. During the lifetime of the shared reference, not only the
            // reference, but everything that can be traced back to it, cannot be changed in any way.
            // Nothing in that structure can be referenced by a variable reference. The owner can only read it.
            // The reason is that the owner is in a read-only state. Anything referenced by shared reference is completely frozen.
            //
            // **Variable access becomes exclusive access. **
            // A value borrowed by a variable reference can only be accessed through that reference. Variable reference raw
            // During the lifetime of a variable reference, references to the reference and everything that can be traced back to it
            // cannot be referenced from any other path. The only reference that may overlap the lifetime of a variable reference is
            // a reference borrowed from that variable reference.
            #[allow(unused_variables)]
            {
                // {
                //     let mut x = 10;
                //     let r1 = &x;
                //     let r2 = &x; // ok: multiple share borrows permitted
                //     x += 10; // error cannot assign to `x` because it is borrowed
                //     assert_eq!(x, 20);

                //     let m = &mut x; // error: cannot borrow `x` as mutable because it is also borrowed as immutable

                //     println!("{}, {}, {}", r1, r2, m); // the references are used here, so their lifetimes must last at least this long
                //     // error[E0506]: cannot assign to `x` because it is borrowed
                //     //    --> src/main.rs:672:17
                //     //     |
                //     // 670 |                 let r1 = &x;
                //     //     |                          -- `x` is borrowed here
                //     // 671 |                 let r2 = &x; // ok: multiple share borrows permitted
                //     // 672 |                 x += 10; // error cannot assign to `x` because it is borrowed
                //     //     |                 ^^^^^^^ `x` is assigned to here but it was already borrowed
                //     // ...
                //     // 677 |                 println!("{}, {}, {}", r1, r2, m);
                //     //     |                                        -- borrow later used here

                //     // For more information about this error, try `rustc --explain E0506`.
                // }

                // {
                //     let mut y = 20;
                //     let m1 = &mut y;
                //     let m2 = &mut y; // error: cannot borrow as mutable more than once
                //     let z = y; // error: cannot use `y` because it was mutably borrowed

                //     println!("{}, {}, {}", m1, m2, z); // references are used here
                //     // error[E0499]: cannot borrow `y` as mutable more than once at a time
                //     //    --> src/main.rs:697:30
                //     //     |
                //     // 696 |                     let m1 = &mut y;
                //     //     |                              ------ first mutable borrow occurs here
                //     // 697 |                     let m2 = &mut y; // error: cannot borrow as mutable more than once
                //     //     |                              ^^^^^^ second mutable borrow occurs here
                //     // ...
                //     // 700 |                     println!("{}, {}, {}", m1, m2, z); // references are used here
                //     //     |                                            -- first borrow later used here

                //     // error[E0503]: cannot use `y` because it was mutably borrowed
                //     //    --> src/main.rs:698:29
                //     //     |
                //     // 696 |                     let m1 = &mut y;
                //     //     |                              ------ `y` is borrowed here
                //     // 697 |                     let m2 = &mut y; // error: cannot borrow as mutable more than once
                //     // 698 |                     let z = y; // error: cannot use `y` because it was mutably borrowed
                //     //     |                             ^ use of borrowed `y`
                //     // 699 |
                //     // 700 |                     println!("{}, {}, {}", m1, m2, z); // references are used here
                //     //     |                                            -- borrow later used here

                //     // Some errors have detailed explanations: E0499, E0503.
                //     // For more information about an error, try `rustc --explain E0499`.
                // }

                // {
                //     let mut w = (107, 109);
                //     let r = &w;
                //     let r0 = &r.0; // ok: reborrowing shared as shared
                //     let m1 = &mut r.1; // error: can't reborrow shared as mutable

                //     // warning: variable does not need to be mutable
                //     //    --> src/main.rs:729:25
                //     //     |
                //     // 729 |                     let mut w = (107, 109);
                //     //     |                         ----^
                //     //     |                         |
                //     //     |                         help: remove this `mut`
                //     //     |
                //     //     = note: `#[warn(unused_mut)]` on by default

                //     // error[E0596]: cannot borrow `r.1` as mutable, as it is behind a `&` reference
                //     //    --> src/main.rs:732:30
                //     //     |
                //     // 732 |                     let m1 = &mut r.1; // error: can't reborrow shared as mutable
                //     //     |                              ^^^^^^^^ `r` is a `&` reference, so the data it refers to cannot be borrowed as mutable
                //     //     |
                //     // help: consider changing this to be a mutable reference
                //     //     |
                //     // 730 |                     let r = &mut w;
                //     //     |                             ~~~~~~

                //     // For more information about this error, try `rustc --explain E0596`.
                // }

                // {
                //     let mut v = (136, 139);
                //     let m = &mut v;
                //     let m0 = &mut m.0; // ok: reborrowing mutable from mutable
                //     *m0 = 137;

                //     let r1 = &m.1; // ok: reborrowing shared from mutable
                //     *m0 = 137;

                //     v.1; // error: access through other paths still forbidden
                //     println!("{}", r1); // r1 gets used here
                //     // error[E0503]: cannot use `v.1` because it was mutably borrowed
                //     //    --> src/main.rs:767:21
                //     //     |
                //     // 760 |                     let m = &mut v;
                //     //     |                             ------ `v` is borrowed here
                //     // ...
                //     // 767 |                     v.1; // error: access through other paths still forbidden
                //     //     |                     ^^^ use of borrowed `v`
                //     // 768 |                     println!("{}", r1); // r1 gets used here
                //     //     |                                    -- borrow later used here

                //     // For more information about this error, try `rustc --explain E0503`.
                // }

                // ```cpp
                // struct File {
                //     int descriptor;
                //
                //     File(itn d): descriptor(d) {}
                //
                //     File& operator=(const File &rhs) {
                //         close(descriptor);
                //         descriptor = dup(rhs.descriptor);
                //         return *this;
                //     }
                // }
                // File f(open("foo.txt", ...));
                // f = f;
                // ```
                {
                    struct File {
                        descriptor: i32,
                    }

                    fn new_file(d: i32) -> File {
                        File { descriptor: d }
                    }

                    fn clone_from(this: &mut File, rhs: &File) {
                        close(this.descriptor);
                        this.descriptor = dup(rhs.descriptor);
                    }
                }
            }
        }
    }
}
