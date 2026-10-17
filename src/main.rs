use ask_input::input;
use std::io::{self, Write};

fn main() {
    let number1 = get_number("Введите первое число: ");
    let op = get_operator();
    let number2 = get_number("Введите второе число: ");

    let is_cat_op = op == 'ฅ';
    let clean_op = if is_cat_op { '+' } else { op };

    if clean_op == '/' && number2 == 0.0 {
        println!("Ошибка: деление на ноль!");
        return;
    }

    let result = match clean_op {
        '+' => number1 + number2,
        '-' => number1 - number2,
        '*' => number1 * number2,
        '/' => number1 / number2,
        _ => {
            println!("Неверный оператор!");
            return;
        }
    };

    let suffix = if is_cat_op { " Лапок" } else { "" };
    println!("{} {} {} = {}{}", number1, clean_op, number2, result, suffix);
}

fn get_number(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    input::<f64>().expect("Ошибка: введите число")
}

fn get_operator() -> char {
    print!("Введите оператор: ");
    io::stdout().flush().unwrap();
    input::<String>()
        .expect("Ошибка ввода оператора")
        .chars()
        .next()
        .expect("Ошибка: оператор не введён")
}
