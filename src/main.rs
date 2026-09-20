use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();

    stdin()
        .read_to_string(&mut input)
        .expect("> Failed to read the line");

    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse().expect("This is not a number"));

    let numbers: [i32; 2] = [
        iter.next().expect("First number not found"),
        iter.next().expect("Second number not found"),
    ];

    let total: i64 = numbers.iter().sum();

    println!("{}", total);
}
