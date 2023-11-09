fn main() {
    print_padovan();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan); // dropped here
}

#[cfg(test)]
mod test {
    use std::rc::{Rc, Weak};

    use crate::print_padovan;

    #[test]
    fn test_padovan() {
        print_padovan();
    }

    #[test]
    fn test_composers() {
        struct Person {
            name: String,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });
        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });
        composers.push(Person {
            name: "Lully".to_string(),
            birth: 1632,
        });

        for composer in &composers {
            println!("{}, born {}", composer.name, composer.birth)
        }

        assert_eq!("Palestrina", composers[0].name);
        assert_eq!(1525, composers[0].birth);
    }

    #[test]
    fn test_assignment_move() {
        // let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        // let t = s;
        // let u = s;
        // 56 |         let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        //    |             - move occurs because `s` has type `Vec<String>`, which does not implement the `Copy` trait
        // 57 |         let t = s;
        //    |                 - value moved here
        // 58 |         let u = s;
        //    |                 ^ value used here after move

        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s.clone();
        let u = s.clone();

        assert_eq!(s[0], "udon");
        assert_eq!(t[0], "udon");
        assert_eq!(u[0], "udon");
    }

    #[test]
    fn test_operations_accompany_with_move() {
        let mut _s = "Govinda".to_string();
        _s = "Siddhartha".to_string(); // value "Govinda" dropped here
    }

    #[test]
    fn test_move_and_control_flow() {
        use rand::Rng;

        fn random_bool() -> bool {
            let number = rand::thread_rng().gen_range(0..1);
            if number > 0 {
                true
            } else {
                false
            }
        }

        fn f(_x: Vec<u8>) {}
        fn g(_x: Vec<u8>) {}
        // fn h(_x: Vec<u8>) {}

        let c = random_bool();
        let x = vec![10, 20, 30];
        if c {
            f(x)
        } else {
            g(x);
        }

        // h(x);
        // error[E0382]: use of moved value: `x`
        //    --> src/main.rs:104:11
        //     |
        // 97  |         let x = vec![10, 20, 30];
        //     |             - move occurs because `x` has type `Vec<u8>`, which does not implement the `Copy` trait
        // 98  |         if c {
        // 99  |             f(x)
        //     |               - value moved here
        // 100 |         } else {
        // 101 |             g(x);
        //     |               - value moved here
        // ...
        // 104 |         h(x);
        //     |           ^ value used here after move
        //     |
        // note: consider changing this parameter type in function `g` to borrow instead if owning the value isn't necessary
        //    --> src/main.rs:93:18
        //     |
        // 93  |         fn g(_x: Vec<u8>) {}
        //     |            -     ^^^^^^^ this parameter takes ownership of the value
        //     |            |
        //     |            in this function
        // note: consider changing this parameter type in function `f` to borrow instead if owning the value isn't necessary
        //    --> src/main.rs:92:18
        //     |
        // 92  |         fn f(_x: Vec<u8>) {}
        //     |            -     ^^^^^^^ this parameter takes ownership of the value
        //     |            |
        //     |            in this function
        // help: consider cloning the value if the performance cost is acceptable
        //     |
        // 101 |             g(x.clone());
        //     |                ++++++++
        // help: consider cloning the value if the performance cost is acceptable
        //     |
        // 99  |             f(x.clone())
        //     |                ++++++++

        // For more information about this error, try `rustc --explain E0382`.
        // error: could not compile `ownership` due to previous error

        // let x = vec![10, 20, 30];
        // #[allow(while_true)]
        // while true {
        //     g(x); // bad: x whould be moved in first iteration
        //           // uninitialized in second
        // }

        // fn h() {}
        //    --> src/main.rs:153:9
        //     |
        // 94  |         fn h(_x: Vec<u8>) {}
        //     |         ----------------- previous definition of the value `h` here
        // ...
        // 153 |         fn h() {}
        //     |         ^^^^^^ `h` redefined here
        //     |
        //     = note: `h` must be defined only once in the value namespace of this block

        fn e(_x: Vec<u8>) {}
        fn n() -> Vec<u8> {
            vec![11, 22, 33]
        }

        // let mut x = vec![10, 20, 30];
        // #[allow(while_true)]
        // while true {
        //     g(x); // move from x
        //     x = n(); // give x a fresh value
        // }
        // e(x);

        let mut x = vec![10, 20, 30];
        for _i in 0..10 {
            g(x); // move from x
            x = n(); // give x a fresh value
        }
        e(x);
    }

    #[test]
    fn test_move_and_value_ref_by_index() {
        // Build a vector of the string "101", "102", ... "105"
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string());
        }

        assert_eq!(&v[1], "102");

        // // Pull out random elements from the vector.
        // let third = v[2]; // error: Cannot move out of index of Vec
        // let fifth = v[4]; // here too
        //    --> src/main.rs:187:21
        //     |
        // 187 |         let third = v[2]; // error: Cannot move out of index of Vec
        //     |                     ^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
        //     |
        // help: consider borrowing here
        //     |
        // 187 |         let third = &v[2]; // error: Cannot move out of index of Vec
        //     |                     +

        // error[E0507]: cannot move out of index of `Vec<String>`
        //    --> src/main.rs:188:21
        //     |
        // 188 |         let fifth = v[4]; // here too
        //     |                     ^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
        //     |
        // help: consider borrowing here
        //     |
        // 188 |         let fifth = &v[4]; // here too
        //     |                     +

        // 1. Pop a value off the end of the ector:
        let fifth = v.pop().expect("vector empty!");
        assert_eq!(fifth, "105");

        // 2. Move a value out of a given index in the vector,
        // and move the last element into its spot:
        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        // 3. Swap in another value for the one we're taking out:
        let third = std::mem::replace(&mut v[2], "substitute".to_string());
        assert_eq!(third, "103");

        // Let's see what's left of our vector.
        assert_eq!(v, vec!["101", "104", "substitute"]);

        {
            // struct Label {
            //     number: u32,
            // }

            // fn print(l: Label) {
            //     println!("STAMP: {}", l.number);
            // }

            // let l = Label { number: 3 };
            // print(l);
            // println!("my label number is: {}", l.number);
            // error[E0382]: borrow of moved value: `l`
            //    --> src/main.rs:245:44
            //     |
            // 243 |         let l = Label { number: 3 };
            //     |             - move occurs because `l` has type `Label`, which does not implement the `Copy` trait
            // 244 |         print(l);
            //     |               - value moved here
            // 245 |         println!("my label number is: {}", l.number);
            //     |                                            ^^^^^^^^ value borrowed here after move
            //     |
            // note: consider changing this parameter type in function `print` to borrow instead if owning the value isn't necessary
            //    --> src/main.rs:239:21
            //     |
            // 239 |         fn print(l: Label) {
            //     |            -----    ^^^^^ this parameter takes ownership of the value
            //     |            |
            //     |            in this function
            //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        }

        {
            #[derive(Copy, Clone)]
            struct Label {
                number: u32,
            }

            fn print(l: Label) -> u32 {
                l.number
            }

            let l = Label { number: 3 };
            assert_eq!(print(l), 3)
        }

        {
            // #[derive(Copy, Clone)]
            // struct Label {
            //     name: String,
            // }
            // error[E0204]: the trait `Copy` may not be implemented for this type
            //    --> src/main.rs:282:22
            //     |
            // 282 |             #[derive(Copy, Clone)]
            //     |                      ^^^^
            // 283 |             struct Label {
            // 284 |                 name: String,
            //     |                 ------------ this field does not implement `Copy`
            //     |
            //     = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)

            // For more information about this error, try `rustc --explain E0204`.
        }
    }

    #[test]
    fn test_rc_and_arc() {
        // Arc: atomic reference count
        // In concurrent programming, "atomicity" refers to operations that are indivisible or uninterruptible.
        // In other words, an atomic operation is one that always runs to completion without the possibility of
        // being interrupted by another operation.

        // Rust can infer all thesetypes; written out for clarity
        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t: Rc<String> = s.clone();
        let _u: Rc<String> = s.clone();

        assert!(s.contains("shira"));
        assert_eq!(t.find("taki"), Some(5));

        // s.push_str(" noodles");
        // error[E0596]: cannot borrow data in an `Rc` as mutable
        //    --> src/main.rs:318:9
        //     |
        // 318 |         s.push_str(" noodles");
        //     |         ^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
        //     |
        //     = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<String>`

        // For more information about this error, try `rustc --explain E0596`.
    }

    #[test]
    fn test_rc_weak() {
        let empty: Weak<i64> = Weak::new();
        assert_eq!(empty.strong_count(), 0);
        assert_eq!(empty.strong_count(), 0);

        let upgraded = empty.upgrade(); // Option<Rc<i64>>
        assert!(upgraded.is_none());
    }

    #[test]
    fn test_as_ptr() {
        use std::ptr;
        use std::rc::Rc;

        let strong = Rc::new("hello".to_owned()); // Rc<String>
        let weak = Rc::downgrade(&strong); // Weak<String>

        let ptr = &*strong; // &String
        assert!(ptr::eq(ptr, weak.as_ptr()));
        assert_eq!("hello", unsafe { &*weak.as_ptr() });

        // assert_eq!("hello", &*weak.as_ptr());
        // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
        //    --> src/main.rs:350:29
        //     |
        // 350 |         assert_eq!("hello", &*weak.as_ptr());
        //     |                             ^^^^^^^^^^^^^^^ dereference of raw pointer
        //     |
        //     = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

        // For more information about this error, try `rustc --explain E0133`.
    }
}
