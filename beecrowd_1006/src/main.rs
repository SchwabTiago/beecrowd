use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();


    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();
    io::stdin().read_line(&mut num3).unwrap();


    let num1:f64 = num1.trim().parse().unwrap();
    let num2:f64 = num2.trim().parse().unwrap();
    let num3:f64 = num3.trim().parse().unwrap();

    let formula = ((num1*2.0)+(num2*3.0)+(num3*5.0))/10.0;
    println!("MEDIA = {:.1}",formula);
}