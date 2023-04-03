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

        if input.to_lowercase() == "quit" {break}
        let equ = Equation::from(input);
        let calc = Calculator::new(equ);
        let result = calc
            .calculate()
            .expect("Failed to calculate");

        println!("Result: {}", result);
    }
}
