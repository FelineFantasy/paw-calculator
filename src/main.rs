use ask_input::input;
use std::io::{self, Write};

fn main() {
    let number1 = get_number("Введите первое число: ");
    let op = get_operator();
    let number2 = get_number("Введите второе число: ");

    let is_cat_op = op == 'ฅ';
    let clean_op = if is_cat_op { '+' } else { op };

    let result = match clean_op {
        '+' => number1 + number2,
        '-' => number1 - number2,
        '*' => number1 * number2,
        '/' => {
            if number2 == 0.0 {
                println!("Ошибка: деление на ноль!");
                return;
            }
            number1 / number2
        }
        _ => unreachable!(), // Оператор гарантированно валиден благодаря get_operator
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
    loop {
        print!("Введите оператор (+, -, *, /, ฅ): ");
        io::stdout().flush().unwrap();
        let op = input::<String>()
            .expect("Ошибка ввода оператора")
            .chars()
            .next()
            .unwrap_or(' ');

        if ['+', '-', '*', '/', 'ฅ'].contains(&op) {
            return op;
        }
        println!("Ошибка: неверный оператор. Попробуйте снова.");
    }
}
