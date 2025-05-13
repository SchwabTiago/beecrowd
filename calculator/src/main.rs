use colour::green_ln;
use std::io;

fn main() {
    loop {
        green_ln!("---- Calculator ----");
        println!("1- Sum");
        println!("2- Sub");
        println!("3- Mult");
        println!("4- Div");
        let mut menu_selected = String::new();

        io::stdin()
            .read_line(&mut menu_selected)
            .expect("Failed to read the line!!");

        println!("{}", menu_selected);
    }
}
