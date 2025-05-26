use std::io;

fn main() {
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).unwrap();
    let a: i32 = a_input.trim().parse().unwrap();

    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).unwrap();
    let b: i32 = b_input.trim().parse().unwrap();

    let soma = a + b;

    println!("SOMA = {}", soma);
}
