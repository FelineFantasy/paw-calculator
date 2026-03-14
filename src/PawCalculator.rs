use ask_input::{float_input, str_input};

fn main() {
    println!("Введите первое число: ");
    let number1 = float_input();

    println!("Введите оператор: ");
    let op = str_input();

    println!("Введите второе число: ");
    let number2 = float_input();

    if op == "+" {
        println!("{} + {} = {}", number1, number2, number1 + number2);
    } else if op == "-" {
        println!("{} - {} = {}", number1, number2, number1 - number2);
    } else if op == "*" {
        println!("{} * {} = {}", number1, number2, number1 * number2);
    } else if op == "/" {
        println!("{} / {} = {}", number1, number2, number1 / number2);
    } 
    
    else if op == "ฅ+" {
        println!("{} + {} = {} Лапок", number1, number2, number1 + number2);
    } else if op == "ฅ-" {
        println!("{} - {} = {} Лапок", number1, number2, number1 - number2);
    } else if op == "ฅ*" {
        println!("{} * {} = {} Лапок", number1, number2, number1 * number2);
    } else if op == "ฅ/" {
        println!("{} / {} = {} Лапок", number1, number2, number1 / number2);
    } 
    
    else {
        println!("Неверный оператор!")
    }
}