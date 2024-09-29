use std::collections::HashMap;
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

    let text_frequencies = analyze_text_frequencies(&text);

    let lang_frequencies = get_ukrainian_letter_frequencies();

    let mut best_shift = 0;
    let mut best_score = f64::MAX;

    for shift in 0..32 {
        let decrypted_text = decrypt_caesar(&text, shift);
        let decrypted_frequencies = analyze_text_frequencies(&decrypted_text);

        let score = compare_frequencies(&decrypted_frequencies, &lang_frequencies);

        if score < best_score {
            best_score = score;
            best_shift = shift;
        }
    }

    let decrypted_message = decrypt_caesar(&text, best_shift);
    println!("Знайдений зсув: {}", best_shift);
    println!("Дешифроване повідомлення: {}", decrypted_message);
}

fn get_ukrainian_letter_frequencies() -> HashMap<char, f64> {
    let mut frequencies = HashMap::new();
    frequencies.insert('о', 0.1043);
    frequencies.insert('а', 0.0762);
    frequencies.insert('і', 0.0704);
    frequencies.insert('н', 0.0597);
    frequencies.insert('е', 0.0539);
    frequencies.insert('с', 0.0517);
    frequencies.insert('и', 0.0490);
    frequencies.insert('т', 0.0481);
    frequencies.insert('к', 0.0450);
    frequencies.insert('р', 0.0388);
    frequencies.insert('л', 0.0383);
    frequencies.insert('м', 0.0357);
    frequencies.insert('у', 0.0316);
    frequencies.insert('я', 0.0294);
    frequencies.insert('в', 0.0290);
    frequencies.insert('д', 0.0272);
    frequencies.insert('п', 0.0245);
    frequencies.insert('ф', 0.0218);
    frequencies.insert('ь', 0.0187);
    frequencies.insert('є', 0.0169);
    frequencies.insert('з', 0.0169);
    frequencies.insert('ю', 0.0156);
    frequencies.insert('г', 0.0125);
    frequencies.insert('ї', 0.0120);
    frequencies.insert('х', 0.0120);
    frequencies.insert('ц', 0.0111);
    frequencies.insert('б', 0.0102);
    frequencies.insert('ш', 0.0098);
    frequencies.insert('ж', 0.0085);
    frequencies.insert('й', 0.0080);
    frequencies.insert('ч', 0.0067);
    frequencies.insert('щ', 0.0062);
    frequencies
}

fn analyze_text_frequencies(text: &str) -> HashMap<char, f64> {
    let mut frequency_map = HashMap::new();
    let mut total_letters = 0;


    for c in text.chars().filter(|c| c.is_alphabetic()) {
        let c = c.to_lowercase().next().unwrap();
        *frequency_map.entry(c).or_insert(0.0) += 1.0;
        total_letters += 1;
    }

    for value in frequency_map.values_mut() {
        *value = *value / total_letters as f64;
    }

    frequency_map
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

fn compare_frequencies(text_frequencies: &HashMap<char, f64>, lang_frequencies: &HashMap<char, f64>) -> f64 {
    let mut score = 0.0;

    for (letter, text_freq) in text_frequencies {
        if let Some(lang_freq) = lang_frequencies.get(letter) {
            score += (text_freq - lang_freq).abs();
        } else {
            score += *text_freq;
        }
    }

    score
}


