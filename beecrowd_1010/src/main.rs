use std::io;
fn main() {
    let mut input_one = String::new();
    let mut input_two = String::new();

    io::stdin().read_line(&mut input_one).unwrap();
    io::stdin().read_line(&mut input_two).unwrap();

    let values1: Vec<&str> = input_one.trim().split_whitespace().collect();
    let quantity1: i32 = values1[1].parse().unwrap();
    let price1: f64 = values1[2].parse().unwrap();

    let values2: Vec<&str> = input_two.trim().split_whitespace().collect();
    let quantity2: i32 = values2[1].parse().unwrap();
    let price2: f64 = values2[2].parse().unwrap();

    let total = (quantity1 as f64 * price1) + (quantity2 as f64 * price2);

    println!("VALOR A PAGAR: R$ {:.2}", total);
}