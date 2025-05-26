use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();

    let num1:f64 = num1.trim().parse().unwrap();
    let num2:f64 = num2.trim().parse().unwrap();

    let formula = ((num1*3.5)+(num2*7.5))/11.0;
    println!("MEDIA = {:.5}",formula);
}
