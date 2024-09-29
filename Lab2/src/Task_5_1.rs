use std::fs;
use std::io::{self, Write};
use rfd::FileDialog;

#[test]

fn main() {

    let file_path = FileDialog::new()
        .add_filter("Текстові файли", &["txt"])
        .pick_file()
        .expect("Не вдалося вибрати файл");


    let text = fs::read_to_string(file_path).expect("Не вдалося прочитати файл");


    print!("Введіть величину зсуву для дешифрування: ");
    io::stdout().flush().unwrap();

    let mut shift_input = String::new();
    io::stdin()
        .read_line(&mut shift_input)
        .expect("Помилка при введенні зсуву");

    let shift: i32 = shift_input.trim().parse().expect("Неправильний формат зсуву");


    let decrypted_message = decrypt_caesar(&text, shift);


    println!("Дешифроване повідомлення: {}", decrypted_message);
}

fn decrypt_caesar(text: &str, shift: i32) -> String {
    let alphabet = "абвгґдеєжзийіїклмнопрстуфхцчшщьюя";
    let alphabet_len = alphabet.chars().count() as i32;

    text.chars()
        .map(|c| {
            if let Some(pos) = alphabet.chars().position(|x| x == c) {
                let shifted_pos = (pos as i32 - shift).rem_euclid(alphabet_len);
                alphabet.chars().nth(shifted_pos as usize).unwrap()
            } else {
                c
            }
        })
        .collect()
}
