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
    num_ops: Vec<EquationOptions>,
}

impl Equation {
    pub fn new() -> Self {
        Equation {
            num_ops: Vec::new(),
        }
    }

    pub fn from(equation: String) -> Result<Self, CalculationError> {
        let mut equ = Equation::new();
        equ.set(equation)?;
        Ok(equ)
    }

    pub fn set(&mut self, calc: String) -> Result<(), CalculationError> {
        // Buffer for grouping a number together until an operator is reached.
        let mut buf = String::new();
        let mut num_ops: Vec<EquationOptions> = Vec::new();
        
        for char in calc.chars() {
            match char {
                '+' => self.pusher(EquationOptions::Plus, &mut buf, &mut num_ops)?,
                '-' => self.pusher(EquationOptions::Minus, &mut buf, &mut num_ops)?,
                '*' => self.pusher(EquationOptions::Multiply, &mut buf, &mut num_ops)?,
                '/' => self.pusher(EquationOptions::Divide, &mut buf, &mut num_ops)?,
                num => buf.push(num),
            }
        }
        // Required to push remaining contents of buf to num_ops.
        self.push_num(&mut buf, &mut num_ops)?;

        self.num_ops = num_ops;
        Ok(())
    }

    // Made for reduction of repetition in match statement in self.set()
    fn pusher(&self, calc_option: EquationOptions, str: &mut String, vec: &mut Vec<EquationOptions>) -> Result<(), CalculationError> {
        self.push_num(str, vec)?;
        self.push_op(calc_option, vec);
        str.clear();
        Ok(())
    }

    // Seperated from self.pusher() so that the functionality of pushing numbers and operators
    // doesn't rely on another one existing to use.
    fn push_num(&self, str: &mut String, vec: &mut Vec<EquationOptions>) -> Result<(), CalculationError> {
        vec.push(EquationOptions::Number(str.parse::<f64>()?));
        Ok(())
    }

    // Seperated from self.pusher for the same reason as self.push_num(). I think this is better
    // for future scaling.
    fn push_op(&self, calc_option: EquationOptions, vec: &mut Vec<EquationOptions>) {
        vec.push(calc_option);
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

    pub fn calculate(&self) -> Result<f64, ()> {
        let mut curr_op: EquationOptions = EquationOptions::Plus;
        let mut main_num: f64 = 0.0;

        for op in self.equation.num_ops.iter() {
            match op {
                EquationOptions::Number(num) => {
                    match curr_op {
                        EquationOptions::Plus => main_num += num,
                        EquationOptions::Minus => main_num -= num,
                        EquationOptions::Multiply => main_num *= num,
                        EquationOptions::Divide => main_num /= num,
                        EquationOptions::Number(_) => return Err(()),
                    }   
                }
                op => curr_op = op.clone(),
            } 
        }
        Ok(main_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_equals_two() {
        let equ = Equation::from("1+1".to_string()).unwrap();
        let calc = Calculator::new(equ);

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
