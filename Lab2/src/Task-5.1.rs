use std::fs::File;
use std::io::{BufReader, Read};

fn caesar_decrypt(text: &str, shift: i32) -> String {
    let mut decrypted_text = String::new();
    let alphabet: &str = "абвгґдеєжзийіклмнопрстуфхцчшщьюя ";
    for ch in text.chars() {
        if let Some(pos) = alphabet.find(ch) {
            let new_pos = (pos as i32 - shift).rem_euclid(alphabet.len() as i32) as usize;
            if let Some(new_char) = alphabet.chars().nth(new_pos) {
                decrypted_text.push(new_char);
            } else {
                decrypted_text.push(ch);
            }
        } else {
            decrypted_text.push(ch);
        }
    }
    decrypted_text
}

fn main() {

    let file_path = rfd::FileDialog::new()
        .add_filter("Текстові файли", &["txt"])
        .pick_file()
        .expect("Не вдалося вибрати файл");

    let file_path_str = file_path.to_str().expect("Не вдалося перетворити шлях у строку");
    let shift = 3;


    let file = File::open(file_path_str).expect("Не вдалося відкрити файл");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Не вдалося прочитати файл");


    let decrypted = caesar_decrypt(&contents, shift);
    println!("Розшифрований текст:\n{}", decrypted);
}
