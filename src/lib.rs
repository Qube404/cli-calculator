use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum EquationOptions {
    Plus(f64),
    Minus(f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

type CalculationError = <f64 as FromStr>::Err;

#[derive(Debug)]
pub struct Equation {
    num_ops: Vec<EquationOptions>,
    order_flag: bool,
}

impl Equation {
    pub fn new() -> Self {
        Equation {
            num_ops: Vec::new(),
            order_flag: false,
        }
    }

    pub fn from(equation: String) -> Result<Self, CalculationError> {
        let mut equ = Equation::new();
        equ.set(equation)?;
        Ok(equ)
    }

    // Future qube notes
    //
    // Refactoring all this bullshit to work with the new enum.
    // Multiplications and divisions need to be immediately evaluated.
    // Maybe remove Calculator type?
    pub fn set(&mut self, calc: String) -> Result<(), CalculationError> {
        // Buffer for grouping a number together until an operator is reached.
        let mut buf = String::new();
        let mut groups: Vec<String> = Vec::new();

        // Pretty regex I don't want to have to figure out again if I decide to use it in the
        // future.
        // let re = Regex::new(r"^[0-9]+(.[0-9]+)([-+*/]{1}[0-9]+(.[0-9+]+))*$").unwrap();
        
        // Seperates string into groups with included operator delimiter.
        Ok(())
    }
} 

#[derive(Debug)]
pub struct Calculator {
    equation: Equation,
}

impl Calculator {
    pub fn new(equation: Equation) -> Calculator {
        Calculator {
            equation,
        }
    }

    // Also gotta refactor this bullass shit to work with new set function.
    // Might just remove this if the new set function does all the mathematical operations
    // immediately. Might use this actually to do all the calculations for set, reverse the
    // dependence of equation so that it holds a calculator instead of calculator holding an
    // equation.
    pub fn calculate(&mut self) -> Result<f64, ()> {
        let mut main_num: f64 = 0.0;
        let mut filtered: Vec<EquationOptions> = Vec::new();
        let filter = &mut self
            .equation
            .num_ops;

        /* match filter.remove(0) {
            EquationOptions::Number(num) => main_num += num,
            _ => panic!("No number found!"),
        } */

        println!("{:?}", filtered);
        Ok(main_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_equals_two() {
        let equ = Equation::from("1+1".to_string()).unwrap();
        let mut calc = Calculator::new(equ);

        let result = calc
            .calculate()
            .unwrap();

        assert_eq!(result, 2.0);
    }

    #[test]
    fn ten_times_twenty_six_equals_two_hundred_and_sixty() {
        let equ = Equation::from("10*26".to_string()).unwrap();
        let calc = Calculator::new(equ);

        let result = calc
            .calculate()
            .unwrap();

        assert_eq!(result, 260.0);
    }

    #[test]
    fn one_plus_two_plus_three_equals_six() {
        let equ = Equation::from("1+2+3".to_string()).unwrap();
        let calc = Calculator::new(equ);

        let result = calc
            .calculate()
            .unwrap();

        assert_eq!(result, 6.0);
    }

    #[test]
    fn ten_times_two_minus_three_plus_ninety_seven_divided_by_twelve_equals_nine_point_five() {
        let equ = Equation::from("10*2-3+97/12".to_string()).unwrap();
        let calc = Calculator::new(equ);

        let result = calc
            .calculate()
            .unwrap();

        assert_eq!(result, 9.5);
    }
}
