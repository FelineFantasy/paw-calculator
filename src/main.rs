use ask_input::input;

const ZERO_THRESHOLD: f64 = 1e-12;

fn main() {
    println!("Введите первое число: ");
    let number1: f64 = input().expect("Ошибка ввода числа");

    println!("Введите оператор: ");
    let op: char = input()
        .expect("Ошибка ввода оператора")
        .chars()
        .next()
        .unwrap_or(' ');

    println!("Введите второе число: ");
    let number2: f64 = input().expect("Ошибка ввода числа");

    let clean_op = if op == 'ฅ' { '+' } else { op };
    let is_cat_op = op == 'ฅ';

    if clean_op == '/' && number2.abs() < ZERO_THRESHOLD {
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
