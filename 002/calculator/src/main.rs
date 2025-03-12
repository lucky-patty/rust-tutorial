use std::io::{self, Write};

fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();
    let result: i32;

    
    println!("========== Basic Calculator ==========");
    print!("Enter the first number: ");

    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut x)
        .expect("Invalid Input");

    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Parse!");
            return;
        }
    };

    print!("Enter the second number: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut y)
        .expect("Invalid Input");
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Parse!");
            return;
        }
    };

    println!("--------------------");
    println!("List of operators: ");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("--------------------");
    print!("Please select operation number 1 - 4: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut op)
        .expect("Invalid Input");

    let op: i32 = match op.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Parse");
            return;
        }
    };

    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => {
            println!("Invalid Selection");
            return;
        }
    }

    println!("The result is: {}", result);
}
