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

    let result = match op {
        Operator::Add | Operator::CatAdd => Some(number1 + number2),
        Operator::Sub => Some(number1 - number2),
        Operator::Mul => Some(number1 * number2),
        Operator::Div => {
            if number2 == 0.0 {
                println!("Ошибка: деление на ноль!");
                None
            } else {
                Some(number1 / number2)
            }
        }
    };

    if let Some(res) = result {
        let (op_char, suffix) = match op {
            Operator::Add => ('+', ""),
            Operator::Sub => ('-', ""),
            Operator::Mul => ('*', ""),
            Operator::Div => ('/', ""),
            Operator::CatAdd => ('+', " Лапок"),
        };
        println!("{} {} {} = {}{}", number1, op_char, number2, res, suffix);
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
