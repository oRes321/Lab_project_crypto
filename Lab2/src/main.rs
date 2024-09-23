use std::fs::File;
use std::io::{BufReader, Read};

fn caesar_cipher(text: &str, shift: i32) -> String {
    let mut encrypted_text = String::new();
    let alphabet = "абвгґдеєжзийіклмнопрстуфхцчшщьюя";

    for ch in text.chars() {
        if let Some(pos) = alphabet.find(ch) {
            let new_pos = (pos as i32 + shift) % alphabet.len() as i32;
            let new_pos = if new_pos < 0 { new_pos + alphabet.len() as i32 } else { new_pos };
            if let Some(new_char) = alphabet.chars().nth(new_pos as usize) {
                encrypted_text.push(new_char);
            } else {
                encrypted_text.push(ch);
            }
        } else {
            encrypted_text.push(ch);
        }
    }
    encrypted_text
}

fn main() {
    let file_path = rfd::FileDialog::new()
        .add_filter("Текстові файли", &["txt"])
        .pick_file()
        .expect("Не вдалося вибрати файл");

    let file_path_str = file_path.to_str().expect("Не вдалося перетворити шлях у строку");
    let shift = 3; // Зсув для шифрування

    let file = File::open(file_path_str).expect("Не вдалося відкрити файл");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Не вдалося прочитати файл");

    let encrypted = caesar_cipher(&contents, shift);
    println!("Зашифрований текст:\n{}", encrypted);
}