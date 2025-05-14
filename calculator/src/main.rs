use colour::{green_ln, yellow_ln};
use std::io;

fn main() {
    loop {
        yellow_ln!("--- Calculadora ---");
        print!("\n");
        print!("\n");
        green_ln!("---- Escolha uma opção ----");
        println!("1 - Soma");
        println!("2 - Subtração");
        println!("3 - Multiplicação");
        println!("4 - Divisão");

        let mut option_input = String::new();
        io::stdin()
            .read_line(&mut option_input)
            .expect("Erro ao ler a opção");

        let option: i32 = option_input.trim().parse().expect("Opção inválida");

        println!("Digite o primeiro número:");
        let mut num1_input = String::new();
        io::stdin()
            .read_line(&mut num1_input)
            .expect("Erro ao ler o primeiro número");
        let num1: f64 = num1_input.trim().parse().expect("Número inválido");

        println!("Digite o segundo número:");
        let mut num2_input = String::new();
        io::stdin()
            .read_line(&mut num2_input)
            .expect("Erro ao ler o segundo número");
        let num2: f64 = num2_input.trim().parse().expect("Número inválido");

        match option {
            1 => println!("Resultado: {}", num1 + num2),
            2 => println!("Resultado: {}", num1 - num2),
            3 => println!("Resultado: {}", num1 * num2),
            4 => {
                if num2 == 0.0 {
                    println!("Erro: divisão por zero!");
                } else {
                    println!("Resultado: {}", num1 / num2);
                }
            }
            _ => println!("Opção inválida."),
        }
    }
}
