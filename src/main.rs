use std::io;

use cli_qalculator::Calculation;

fn main() {
    println!("Enter calculation: ");

    let stdin = io::stdin();
    let mut input = String::new(); 

    stdin
        .read_line(&mut input)
        .expect("Could not read line!");
    input.pop();

    let calc = Calculation::build(input);
}
