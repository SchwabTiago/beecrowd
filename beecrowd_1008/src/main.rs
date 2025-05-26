use std::io;
fn main() {
    let mut employee_number = String::new();
    let mut hours_worked = String::new();
    let mut hourly_rate = String::new();

    io::stdin().read_line(&mut employee_number).unwrap();
    io::stdin().read_line(&mut hours_worked).unwrap();
    io::stdin().read_line(&mut hourly_rate).unwrap();

    let employee_number:f32 = employee_number.trim().parse().unwrap();
    let hours_worked:f64 = hours_worked.trim().parse().unwrap();
    let hourly_rate:f64 = hourly_rate.trim().parse().unwrap();

    let calculated_salary = hourly_rate*hours_worked;

    println!("NUMBER = {}",employee_number);
    println!("SALARY = U$ {:.2}", calculated_salary);

}
