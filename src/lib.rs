use regex::Regex;

#[derive(Debug)]
pub struct Calculation<'a> {
    main_num: f64,
    num_op: Vec<&'a str>,
}

impl Calculation<'_> {
    pub fn build(calc: String) -> Self {
        assert_eq!(vec!["-"], Regex::new("[0-9.]").unwrap().split("1-1").collect::<Vec<&str>>());
        let re = Regex::new("[+-/*]").unwrap();
        let nums: Vec<&str> = re
            .split(&calc)
            .collect();
        let nums: Vec<CalculationOptions> = nums
            .iter()
            .map(|num| CalculationOptions::Number(num.parse::<f64>().expect("Expected a number.")))
            .collect();

        let re = Regex::new("[0-9]").unwrap();
        let ops: Vec<&str> = re
            .split(&calc)
            .collect();
        println!("{:?} {:?}", nums, ops);
        let ops: Vec<CalculationOptions> = ops
            .iter()
            .map(|op| match *op {
                "+" => Some(CalculationOptions::Plus),
                "-" => Some(CalculationOptions::Minus),
                "/" => Some(CalculationOptions::Divide),
                "*" => Some(CalculationOptions::Multiply),
                _ => None,
            }.expect("Expected a mathematical operator!"))
            .collect();
        println!("{:?} {:?}", nums, ops);


        Calculation {
            main_num: 0.0,
            num_op: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum CalculationOptions {
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(f64),
}
