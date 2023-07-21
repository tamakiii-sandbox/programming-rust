use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many_madrigals".to_string(),
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

    show(table);

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

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_collections_hashmap() {}
}
