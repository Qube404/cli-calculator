use std::str::FromStr;
use regex::Regex;

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
    equation: Vec<String>,
    pub result: f64,
}

impl Equation {
    // Returns new Equation with default values.
    pub fn new() -> Self {
        Equation {
            equation: Vec::new(),
            result: 0.0,
        }
    }

    // Does new and set in one function.
    pub fn from(equation: String) -> Result<Self, CalculationError> {
        let mut equ = Equation::new();
        equ.set(equation)?;
        Ok(equ)
    }

    // Turns the string into a vector of EquationOption enum variants.
    pub fn set(&mut self, calc: String) -> Result<(), CalculationError> {
        // Buffer for grouping a number together until an operator is reached.
        let mut buf = String::new();
        let mut flag = false;
        for char in calc.chars() {
            match char {
                '+' | '-' | '*' | '/' => self.push_op(char, &mut buf, &mut flag),
                _ => buf.push(char),
            }
        }
        self.equation.push(buf);
        self.calculate();
        println!("{:?}", self.equation);

        Ok(())
    }

    fn calculate(&mut self) {
        self.calc_mult_div();
        self.calc_plus_min();
    }

    fn calc_mult_div(&mut self) {
        self.equation = 
            self
            .equation
            .iter()
            .map(|str| {
                let mut buf: String = String::new();
                let mut vec: Vec<String> = Vec::new();

                let mut main_num: f64 = 0.0;
                let mut curr_op: String = String::from("+");

                if str.contains("*") || str.contains("/") {
                    for char in str.chars() {
                        match char {
                            '*' | '/' => {
                                vec.push(buf.to_string());
                                vec.push(char.to_string());
                                buf.clear();
                            }

                            _ => buf.push(char),
                        }
                    } 
                    vec.push(buf);

                    for op in vec {
                        match op.as_str() {
                            "*" | "/" => curr_op = op.clone(),
                            num => {
                                match curr_op.as_str() {
                                    "+" => main_num += num.parse::<f64>().unwrap(),
                                    "*" => main_num *= num.parse::<f64>().unwrap(),
                                    "/" => main_num /= num.parse::<f64>().unwrap(),
                                    _ => (),
                                }
                            }
                        }
                    }

                    return main_num.to_string();
                }
                str.to_string()
            }).collect::<Vec<String>>();
    }

    fn calc_plus_min(&mut self) {
    }

    fn push_op(&mut self, op: char, buf: &mut String, flag: &mut bool) {
        match op {
            '+' | '-' => {
                self.equation.push(buf.to_string());
                self.equation.push(op.to_string());
                buf.clear()
            }

            '*' | '/' => {
                buf.push(op);
                *flag = true;
            }

            _ => panic!("Invalid character passed to function."),
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _1_plus_1_equals_2() {
        let equ = Equation::from("1+1".to_string()).unwrap();
        assert_eq!(equ.result, 2.0);
    }

    #[test]
    fn _10_times_26_equals_260() {
        let equ = Equation::from("10*26".to_string()).unwrap();
        assert_eq!(equ.result, 260.0);
    }

    #[test]
    fn _1_plus_2_plus_3_equals_6() {
        let equ = Equation::from("1+2*3".to_string()).unwrap();
        assert_eq!(equ.result, 7.0);
    }

    #[test]
    fn _10_times_2_minus_3_plus_97_divided_by_12_equals_22() {
        let equ = Equation::from("10*2-3+25/5".to_string()).unwrap();
        assert_eq!(equ.result, 22.0);
    }
}
