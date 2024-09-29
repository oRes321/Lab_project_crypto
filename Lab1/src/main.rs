mod task_1_3;
mod task_1_5;
mod task_2;
mod task_3;

use std::io;



fn main() {
    let mut input = String::new();

    println!("Введіть значення n:");
    io::stdin().read_line(&mut input).expect("Помилка при введенні.");
    let n: u32 = input.trim().parse().expect("Невірний формат числа.");

    input.clear();
    println!("Введіть значення k:");
    io::stdin().read_line(&mut input).expect("Помилка при введенні.");
    let k: u32 = input.trim().parse().expect("Невірний формат числа.");

    println!("Перестановка P({}) = {}", n, permutation(n));
    println!("Сполучення C({}, {}) = {}", n, k, combination(n, k));
    println!("Розміщення A({}, {}) = {}", n, k, placement(n, k));
}
fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

fn permutation(n: u32) -> u32 {
    factorial(n)
}

fn combination(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn placement(n: u32, k: u32) -> u32 {
    factorial(n) / factorial(n - k)
}
