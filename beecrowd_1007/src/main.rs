use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();
    let mut num4 = String::new();

    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();
    io::stdin().read_line(&mut num3).unwrap();
    io::stdin().read_line(&mut num4).unwrap();


    let num1:f32 = num1.trim().parse().unwrap();
    let num2:f32 = num2.trim().parse().unwrap();
    let num3:f32 = num3.trim().parse().unwrap();
    let num4:f32 = num4.trim().parse().unwrap();

    let formula = (num1*num2)-(num3*num4);
    println!("DIFERENCA = {}",formula);
}