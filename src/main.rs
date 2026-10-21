use ask_input::input;
use std::io::{self, Write};

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    CatAdd,
}

fn main() {
    let number1 = get_number("Введите первое число: ");
    let op = get_operator();
    let number2 = get_number("Введите второе число: ");

    match calculate(number1, number2, &op) {
        Ok(res) => {
            let (op_char, suffix) = match op {
                Operator::Add => ('+', ""),
                Operator::Sub => ('-', ""),
                Operator::Mul => ('*', ""),
                Operator::Div => ('/', ""),
                Operator::CatAdd => ('+', " Лапок"),
            };
            println!("{} {} {} = {}{}", number1, op_char, number2, res, suffix);
        }
        Err(err) => println!("Ошибка: {}", err),
    }
}

fn calculate(num1: f64, num2: f64, op: &Operator) -> Result<f64, &'static str> {
    match op {
        Operator::Add | Operator::CatAdd => Ok(num1 + num2),
        Operator::Sub => Ok(num1 - num2),
        Operator::Mul => Ok(num1 * num2),
        Operator::Div => {
            if num2 == 0.0 {
                Err("деление на ноль!")
            } else {
                Ok(num1 / num2)
            }
        }
    }
}

fn get_number(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    input::<f64>().expect("Ошибка: введите число")
}

fn get_operator() -> Operator {
    loop {
        print!("Введите оператор (+, -, *, /, ฅ): ");
        io::stdout().flush().unwrap();
        let op_str = input::<String>().expect("Ошибка ввода оператора");

        match op_str.trim() {
            "+" => return Operator::Add,
            "-" => return Operator::Sub,
            "*" => return Operator::Mul,
            "/" => return Operator::Div,
            "ฅ" => return Operator::CatAdd,
            _ => println!("Ошибка: неверный оператор. Попробуйте снова."),
        }
    }
}
