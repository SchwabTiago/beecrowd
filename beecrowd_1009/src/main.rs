use std::io;
fn main() {
    let mut name = String::new();
    let mut fixed_salary = String::new();
    let mut total_sales = String::new();

    io::stdin().read_line(&mut name).unwrap();
    io::stdin().read_line(&mut fixed_salary).unwrap();
    io::stdin().read_line(&mut total_sales).unwrap();

    let fixed_salary: f64 = fixed_salary.trim().parse().unwrap();
    let total_sales: f64 = total_sales.trim().parse().unwrap();

    let result = fixed_salary + (total_sales * 0.15);

    println!("TOTAL = R$ {:.2}", result);
}