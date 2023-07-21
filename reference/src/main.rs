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
