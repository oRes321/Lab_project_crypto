
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[test]

fn main() {

    let file_path = rfd::FileDialog::new()
        .add_filter("Текстові файли", &["txt"])
        .pick_file()
        .expect("Не вдалося вибрати файл");
    let file_path_str = file_path.to_str().expect("Не вдалося перетворити шлях у строку");

    let frequencies = count_char_frequency_from_file(file_path_str);

    let mut freq_vec: Vec<(&char, &f32)> = frequencies.iter().collect();

    freq_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (char, freq) in &freq_vec {
        println!("{}: {:.4}", char, freq);
    }
}


fn count_char_frequency_from_file(file_path: &str) -> HashMap<char, f32> {
    let file = File::open(file_path).expect("Не вдалося відкрити файл");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Не вдалося прочитати файл");

    let mut frequency: HashMap<char, f32> = HashMap::new();
    let mut total_letters = 0;

    for ch in contents.chars() {
        if ch.is_alphabetic() && ch.is_lowercase() {
            *frequency.entry(ch).or_insert(0.0) += 1.0;
            total_letters += 1;
        }
    }

    for value in frequency.values_mut() {
        *value /= total_letters as f32;
    }

    frequency
}