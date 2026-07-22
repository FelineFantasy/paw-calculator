use ask_input::input;

fn main() {
    println!("Введите первое число: ");
    let number1: f64 = input().expect("Ошибка ввода числа");

    println!("Введите оператор: ");
    let op: String = input().expect("Ошибка ввода оператора");

    println!("Введите второе число: ");
    let number2: f64 = input().expect("Ошибка ввода числа");

    let is_cat_op = op.starts_with('ฅ');

    let clean_op = if is_cat_op { &op[3..] } else { op.as_str() };

    if clean_op == "/" && number2 == 0.0 {
        println!("Ошибка: деление на ноль!");
        return;
    }

    let result = match clean_op {
        "+" => number1 + number2,
        "-" => number1 - number2,
        "*" => number1 * number2,
        "/" => number1 / number2,
        _ => {
            println!("Неверный оператор!");
            return;
        }
    };

    let suffix = if is_cat_op { " Лапок" } else { "" };
    println!("{} {} {} = {}{}", number1, clean_op, number2, result, suffix);
}
