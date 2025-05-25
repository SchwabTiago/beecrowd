use std::io;

fn main() {
    let mut num1_input = String::new();
    let mut num2_input = String::new();

    io::stdin()
        .read_line(&mut num1_input)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut num2_input)
        .expect("Failed to read line");

    let num1: i32 = num1_input
        .trim()
        .parse()
        .expect("Please enter a valid number");
    let num2: i32 = num2_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let sum_numbers = num1 + num2;

    println!("X = {}", sum_numbers);
}
