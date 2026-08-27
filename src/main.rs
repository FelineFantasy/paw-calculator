use ask_input::input;

const ZERO_THRESHOLD: f64 = 1e-12;

fn main() {
    let number1 = get_number("Введите первое число: ");
    let op = get_operator();
    let number2 = get_number("Введите второе число: ");

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

fn get_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    input().expect("Ошибка ввода числа").parse().expect("Ошибка: введите число")
}

fn get_operator() -> char {
    println!("Введите оператор: ");
    input()
        .expect("Ошибка ввода оператора")
        .chars()
        .next()
        .unwrap_or(' ')
}
