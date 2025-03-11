use std::io::{self, Write};
use rand::prelude::*;

fn main() {

    // Get an RNG 
    let mut rng = rand::rng();

    // Generate random number 
    let num: u8 = rng.random_range(..11);
    println!("Welcome to guessing number game!");
    let convert_input = loop {
        let mut input = String::new();
        print!("Please input the number between 0 - 10: ");
        let _ = io::stdout().flush();
    
        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim and parse to u8 
        match input.trim().parse::<u8>() {
            Ok(num) => break num,
            Err(_) => println!("Invalid Number! Please enter a valid value"),
        }

    };
    // let convertInput: u8 = input.parse().expect("Invalid Number!");
    println!("You guess: {}", convert_input);
    println!("I guess: {}", num);
    if num == convert_input {
        println!("You win the game!");
    } else {
        println!("You lose!");
    }
}
