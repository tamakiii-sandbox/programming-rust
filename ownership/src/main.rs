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
    use crate::print_padovan;

    #[test]
    fn test_padovan() {
        print_padovan();
    }
}
