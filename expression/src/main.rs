fn main() {
    // 55. A LISP programmer knows the value of everything, but the cost of nothing.
    // http://www.cs.yale.edu/homes/perlis-alan/quotes.html
    println!("Hello, world!");
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::fmt::Error;
    use std::iter::IntoIterator;
    use std::net::{AddrParseError, IpAddr};
    use std::ops::{Deref, DerefMut};
    use std::string::ParseError;

    #[test]
    fn test() {
        {
            static MAX_TEMP: usize = 100;

            struct Cpu {
                tempreature: usize,
            }

            #[derive(PartialEq, Debug)]
            enum HttpStatus {
                Ok,
                ServerError,
            }

            let cpu = Cpu { tempreature: 100 };

            let status = if cpu.tempreature <= MAX_TEMP {
                HttpStatus::Ok
            } else {
                HttpStatus::ServerError // server melted
            };

            assert_eq!(status, HttpStatus::Ok);
        }

        // 6.2
        {
            // Expression Type | Example | Related Traits
            // ---|---|---
            // Array literals | `[1, 2, 3]` |
            // Repeating array literal | `[0; 50]` |
            // tuple | `(6, "crullers")` |
            // grouping | `(2 + 2)` |
            // block | `{ f(); g() }` |
            // control flow expression | `if ok { f() }` | `std::iter::IntoIterator`
            // \- | `if ok { 1 } else { 0 }` | \-
            // macro execution | `println!
            // path | `std::f64::consts::PI` `Point
            // structure literal | `Point { x: 0, y: 0 }` |
            // field access to tuples | `pair.0` | `Deref`, `DerefMut`
            // field access of structure | pointer.x | `Deref`, `DerefMut`
            // method call | `point.translate(50, 50)` | `Deref`, `DerefMut`
            // function call | `stdin()` | `Fn(Arg0, ...) -> T`, `FnMut(Arg0, ...) -> T`, `FnOnce(Arg0, ...) -> T`
            // index | `arr[0]` | `Index`, `IndexMut`, `Deref`, `DerefMut`
            // error check | `create dir("tmp")? ` | `!
            // logical/bitwise NOT | `!ok` | `Not`
            // sign-reversal |`-num` | `Neg`
            // reference resolution | `*ptr` | `Deref`, `DerefMut`
            // Borrowing | `&val` |Type cast | `x as u32
            // typecast | `x as u32` |
            // Multiplication | `n * 2` | `Mul`
            // Divide | `n / 2` | `Neg`
            // remainder (division) | `n % 2` | `Rem`
            // Addition | `n+2` | `Add
            // Subtract | `n - 1` | `Sub
            // Shift Left | `n << 1` | `Shl
            // Shift Right | `n >> 1` | `Shr` Bitwise AND
            // Bitwise AND | `n & 1` | `BitAnd`
            // Bitwise Exclusive OR | `n ^ 1` | `BigXor`
            // bitwise OR | `n | 1` | `BitOr`
            // Smaller | `n < 1` | `std::cmp::PartialOrd`
            // Small Equal | `n <= 1` | `std::cmp::PartialOrd`
            // equal | `n == 1` | `std::cmp::PartialEq`
            // logical AND | `x.ok && y.ok` |
            // logical OR | `x.ok || backup.ok` |
            // range not including the tail | `start . stop` |
            // range including the tail | `start . = stop` |
            // assignment | `x = val` | logical OR
            // assignment | `x = val` |
            // Compound assignment | `x *= 1` | `MulAssign
            // \- | `x /= 1` | `DivAssign`
            // \- | `x %= 1` | `RemAssign`
            // \- | `x += 1` | `AddAssign`
            // \- | `x -= 1` | `SubAssign`
            // \- | `x <<= 1` | `ShlAssign`
            // \- | `x>= 1` | `ShrAssi
            // Closure | `|x, y| x + y` |
        }

        // 6.3
        {
            struct Post {
                author: Option<Author>,
                client_address: String,
            }
            impl Post {
                fn author(&self) -> Option<&Author> {
                    self.author.as_ref()
                }

                fn get_network_metadata(&self) -> Result<NetworkMetadata, AddrParseError> {
                    let client_address = self.client_address.parse()?;
                    Ok(NetworkMetadata { client_address })
                }
            }

            struct Author {
                name: String,
            }
            impl Author {
                fn name(&self) -> String {
                    self.name.to_string()
                }
            }

            struct NetworkMetadata {
                client_address: IpAddr,
            }

            impl NetworkMetadata {
                fn client_address(&self) -> IpAddr {
                    self.client_address
                }
            }

            // --

            fn main(post: Post) -> Result<String, Box<dyn std::error::Error>> {
                let display_name = match post.author() {
                    Some(author) => author.name(),
                    None => {
                        let network_info = post.get_network_metadata()?;
                        let ip = network_info.client_address();
                        ip.to_string()
                    }
                };

                Ok(display_name)
            }

            let post = Post {
                author: Some(Author {
                    name: "John".to_string(),
                }),
                client_address: "192.0.2.0".to_string(),
            };
            assert_eq!(main(post).ok(), Some("John".to_string()));

            let post = Post {
                author: None,
                client_address: "192.0.2.0".to_string(),
            };
            assert_eq!(main(post).ok(), Some("192.0.2.0".to_string()));
        }

        // 6.4
        {
            // let name: type = expr;
            {
                struct User {
                    nickname: Option<String>,
                }
                impl User {
                    fn has_nickname(&self) -> bool {
                        self.nickname.is_some()
                    }
                    fn nickname(&self) -> String {
                        self.nickname.clone().unwrap()
                    }
                    fn register(&mut self, nickname: &str) {
                        self.nickname = Some(nickname.to_string());
                    }
                }
                fn generate_unique_name() -> String {
                    "Unique Name".to_string()
                }
                fn main(mut user: User) -> String {
                    let name;
                    if user.has_nickname() {
                        name = user.nickname();
                    } else {
                        name = generate_unique_name();
                        user.register(&name);
                    }
                    name
                }
                fn main2(mut user: User) -> String {
                    let name = if user.has_nickname() {
                        user.nickname()
                    } else {
                        let name = generate_unique_name();
                        user.register(&name);
                        name
                    };
                    name
                }
                let user = User { nickname: None };
                assert_eq!(main(user), "Unique Name".to_string());

                let user = User {
                    nickname: Some("John".to_string()),
                };
                assert_eq!(main(user), "John".to_string());

                let user = User { nickname: None };
                assert_eq!(main2(user), "Unique Name".to_string());

                let user = User {
                    nickname: Some("John".to_string()),
                };
                assert_eq!(main2(user), "John".to_string());
            }

            // {
            //     use std::cmp::Ordering;
            //     use std::io;
            //     use std::path::Path;

            //     fn show_files() -> io::Result<()> {
            //         let mut v = vec![];
            //         v.push(Path::new("/dev/null"));
            //         v.push(Path::new("/dev/stdout"));
            //         v.push(Path::new("/dev/stderr"));

            //         fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
            //             a.timestamp
            //                 .cmp(&b.timestamp)
            //                 .reverse()
            //                 .then(a.path.cmp(&b.path))
            //         }
            //         v.sort_by(cmp_by_timestamp_then_name);
            //         Ok(())
            //     }
            // }
        }

        // 6.6
        {
            // if let pattern = expr {
            //     block1
            // } else {
            //     block2
            // }
            {
                struct Cookie;
                struct Request {
                    session_cookie: Option<Cookie>,
                }
                fn restore_session(_cookie: Cookie) {}
                fn main(request: Request) -> Result<(), ()> {
                    if let Some(cookie) = request.session_cookie {
                        Ok(restore_session(cookie))
                    } else {
                        Err(())
                    }
                }
                fn main2(request: Request) -> Result<(), ()> {
                    match request.session_cookie {
                        Some(cookie) => Ok(restore_session(cookie)),
                        _ => Err(()),
                    }
                }
                let request = Request {
                    session_cookie: Some(Cookie {}),
                };
                assert!(main(request).is_ok());

                let request = Request {
                    session_cookie: None,
                };
                assert!(main(request).is_err());

                let request = Request {
                    session_cookie: Some(Cookie {}),
                };
                assert!(main2(request).is_ok());

                let request = Request {
                    session_cookie: None,
                };
                assert!(main2(request).is_err());
            }

            // if let Err(err) = show_cheesy_anti_robot_task() {
            //     log_robot_attempt(err);
            //     politely_accuse_user_of_being_a_robot();
            // } else {
            //     session.mark_as_human();
            // }
        }

        // 6.7
        {
            // while condition {
            //     block
            // }

            // while let pattern = expr {
            //     block
            // }

            // loop {
            //     block
            // }

            // for pattern in iterable {
            //     block
            // }
            for i in 0..20 {
                println!("{}", i);
            }

            fn error_messages() -> Vec<String> {
                vec!["Something wrong with tech".to_string()]
            }
            {
                let strings: Vec<String> = error_messages();
                for s in strings {
                    println!("{}", s);
                }
                // assert_eq!(strings.len(), 1); // error
                // println!("{} errors(s)", strings.len()); // error: use of moved value
                // note: `into_iter` takes ownership of the receiver `self`, which moves `strings`
                //    --> /Users/tamakiii/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:262:18
                //     |
                // 262 |     fn into_iter(self) -> Self::IntoIter;
                //     |                  ^^^^
                // help: consider iterating over a slice of the `Vec<String>`'s content to avoid moving into the `for` loop
                //     |
                // 323 |             for s in &strings {
                //     |                      +

                // For more information about this error, try `rustc --explain E0382`.
                // error: could not compile `expression` due to previous error
            }
            {
                let strings: Vec<String> = error_messages();
                for rs in &strings {
                    println!("String {:?} is at address {:p}.", *rs, rs);
                }
                // println!("{} errors(s)", strings.len());
                assert_eq!(strings.len(), 1);
            }
            {
                let mut strings: Vec<String> = error_messages();
                for rs in &mut strings {
                    // the type of rs is &mut String
                    rs.push('\n'); // add a newline to each string
                }
                assert_eq!(strings[0], "Something wrong with tech\n");
            }
        }

        // 6.8
        {
            // let answer = loop {
            //     if let Some(line) = next_line() {
            //         if line.starts_with("answer: ") {
            //             break line;
            //         }
            //     } else {
            //         break "answer: nothing";
            //     }
            // }

            // for line in input_lines {
            //     let trimmed = trim_comments_and_whitespace(line);
            //     if trimmed.is_empty() {
            //         // Jump back to the top of the loop and
            //         // move on to the next line of input.
            //         continue;
            //     }
            // }

            #[derive(Debug)]
            struct Room {
                spot: Vec<String>,
            }
            impl Room {
                fn hiding_spots(&self) -> &Vec<String> {
                    &self.spot
                }
            }

            let key = "spot";
            let apartment = vec![Room {
                spot: vec!["spot".to_string()],
            }];
            'search: for room in apartment {
                for spot in room.hiding_spots() {
                    if spot.contains(key) {
                        println!("Your keys are {} in the {:?}.", spot, room);
                        break 'search;
                    }
                }
            }
        }

        // 6.10
        {
            // "definite assignment"

            // fn wait_for_process(process: &mut Process) -> i32 {
            //     while true {
            //         if process.wait() {
            //             return process.exit_code();
            //         }
            //     }
            // } // error: mismatched types: expected i32, found()

            // std::process::exit()
            // pub fn exit(code: i32) -> !

            // fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
            //     socket.listen();
            //     loop {
            //         let s = socket.accept();
            //         handler.handle(s);
            //     }
            // }
        }

        // 6.11
        #[allow(unused_must_use)]
        {
            // Vec<i32>::with_capacity(1000);
            // error: comparison operators cannot be chained
            //    --> src/main.rs:431:16
            //     |
            // 431 |             Vec<i32>::with_capacity(1000);
            //     |                ^   ^
            //     |
            // help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
            //     |
            // 431 |             Vec::<i32>::with_capacity(1000);
            //     |                ++

            Vec::<i32>::with_capacity(1000);
        }

        // 6.14
        {
            let a: i32 = 0;
            let b = a.wrapping_add(1);
            assert_eq!(a, 0);
            assert_eq!(b, 1);

            assert_eq!(a.checked_div(1), Some(0));
            assert_eq!(a.checked_div(0), None);

            let hi: u8 = 0xe0;
            let lo = !hi;
            assert_eq!(lo, 0x1f);
        }

        // 6.16
        {
            assert_eq!(-1.99 as i32, -1);
            assert_eq!(1e6 as u8, 255);

            assert_eq!(std::char::from_u32(0x0030), Some('0'));
            assert_eq!(std::char::from_u32(0xd800), None);

            // "deref coercions"
            // * Values of type `&String` are automatically converted to type `&str` without casting.
            // * Values of type `&Vec<i32>` are automatically converted to `&[i32]`.
            // * Values of type `Box<Chessboard>` are automatically converted to `&Chessboard`.
        }

        // 6.17
        {
            let is_even = |x| x % 2 == 0;
            assert!(is_even(10));
            assert!(!is_even(11));

            let is_even = |x: u64| -> bool { x % 2 == 0 };
            assert!(is_even(10));
            assert!(!is_even(11));
        }
    }
}
