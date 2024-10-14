use std::io;



fn main() {

    println!("Введіть перше число:");
    let num1 = read_input();


    println!("Введіть оператор (+, -, *, /, & | ^):");
    let operator = read_operator();


    println!("Введіть друге число:");
    let num2 = read_input();


    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '&' => num1 & num2,
        '|' => num1 | num2,
        '^' => num1 ^ num2,
        _ => {
            println!("Невідомий оператор!");
            return;
        }
    };


    println!("Результат: {}", result);
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні");
    input.trim().parse().expect("Введіть ціле число!")
}

fn read_operator() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка при читанні");
    input.trim().chars().next().expect("Введіть правильний оператор")
}