use ask_input::{float_input, str_input};

fn main() {
    println!("Введите первое число: ");
    let number1 = float_input();

    println!("Введите оператор: ");
    let op = str_input();

    println!("Введите второе число: ");
    let number2 = float_input();

    match op.as_str() {
        "+" => println!("{} + {} = {}", number1, number2, number1 + number2),
        "-" => println!("{} - {} = {}", number1, number2, number1 - number2),
        "*" => println!("{} * {} = {}", number1, number2, number1 * number2),
        "/" => {
            if number2 == 0.0 {
                println!("Ошибка: деление на ноль!");
            } else {
                println!("{} / {} = {}", number1, number2, number1 / number2);
            }
        }
        "ฅ+" => println!("{} + {} = {} Лапок", number1, number2, number1 + number2),
        "ฅ-" => println!("{} - {} = {} Лапок", number1, number2, number1 - number2),
        "ฅ*" => println!("{} * {} = {} Лапок", number1, number2, number1 * number2),
        "ฅ/" => {
            if number2 == 0.0 {
                println!("Ошибка: деление на ноль!");
            } else {
                println!("{} / {} = {} Лапок", number1, number2, number1 / number2);
            }
        }
        _ => println!("Неверный оператор!"),
    }
}