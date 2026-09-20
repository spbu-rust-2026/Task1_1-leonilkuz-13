use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("> Failed to read the line");

    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse().expect("This is not a number"));

    let numbers: [i32; 2] = [
        iter.next().expect("First number not found"),
        iter.next().expect("Second number not found"),
    ];

    let total: i32 = numbers.iter().sum();

    println!("{}", total);
}
