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
        }
    }
}
