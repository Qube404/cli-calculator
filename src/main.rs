use std::io;

use cli_qalculator::{Equation, Calculator};

fn main() {
    println!("Enter calculation: ");

    loop {
        let stdin = io::stdin();
        let mut input = String::new(); 

        stdin
            .read_line(&mut input)
            .expect("Could not read line!");

        // Removes trailing \n
        input.pop();

        // Special Commands
        match input.to_lowercase().as_str() {
            "quit" => break,

            "clear" => {
                print!("\x1b[2J\x1b[1;1H");
                println!("Enter calculation: ");
                continue
            }

            "help" => {
                println!("\n-------------------------------------");
                println!("Special Commands (CASE INSENSITIVE): ");
                println!("Quit: Exits the program.");
                println!("Clear: Clears the terminal.");
                println!("Help: Displays this help message.");
                println!("-------------------------------------\n");

                println!("Enter calculation: ");
                continue
            }

            _ => ()
        }

        let equ = match Equation::from(input) {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid symbols in equation!");
                continue
            }
        };

        let calc = Calculator::new(equ);
        let result = match calc.calculate() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid symbols in operator list!");
                continue
            }
        };

        println!("Result: {}", result);
    }
}
