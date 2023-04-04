use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum EquationOptions {
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(f64),
}

type CalculationError = <f64 as FromStr>::Err;

#[derive(Debug)]
pub struct Equation {
    equation: Vec<EquationOptions>,
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
        for char in calc.chars() {
            match char {
                '+' => self.pusher('+', &mut buf),
                '-' => self.pusher('-', &mut buf),
                '*' => self.pusher('*', &mut buf),
                '/' => self.pusher('/', &mut buf),
                _ => buf.push(char),
            }
        }
        self.push_num(&mut buf);
        self.calculate();

        Ok(())
    }

    pub fn calculate(&mut self) {
        self.result += 10.0;
    }

    fn pusher(&mut self, op: char, buf: &mut String) {
        self.push_num(buf);
        self.push_op(&op);
        buf.clear();
    }

    fn push_num(&mut self, buf: &mut String) {
        let num = buf.parse::<f64>().unwrap();
        self.equation.push(EquationOptions::Number(num));
        buf.clear();
    }

    fn push_op(&mut self, op: &char) {
        match op {
            '+' => self.equation.push(EquationOptions::Plus),
            '-' => self.equation.push(EquationOptions::Minus),
            '*' => self.equation.push(EquationOptions::Multiply),
            '/' => self.equation.push(EquationOptions::Divide),
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
        let equ = Equation::from("1+2+3".to_string()).unwrap();
        assert_eq!(equ.result, 6.0);
    }

    #[test]
    fn _10_times_2_minus_3_plus_97_divided_by_12_equals_9_point_5() {
        let equ = Equation::from("10*2-3+97/12".to_string()).unwrap();
        assert_eq!(equ.result, 9.5);
    }
}
