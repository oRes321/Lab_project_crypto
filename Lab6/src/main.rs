use rand::Rng;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn mod_exp(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

fn generate_keys() -> (i64, i64, i64, i64) {
    let p = 61;
    let q = 53;
    let n = p * q;
    let phi = (p - 1) * (q - 1);

    let mut e = 3;
    while gcd(e, phi) != 1 {
        e += 1;
    }

    let mut k = 1;
    while (1 + k * phi) % e != 0 {
        k += 1;
    }
    let d = (1 + k * phi) / e;

    println!("Прості числа p: {}, q: {}", p, q);
    println!("Відкритий ключ (n, e): ({}, {})", n, e);
    println!("Закритий ключ (n, d): ({}, {})", n, d);

    (n, e, d, phi)
}

fn encrypt(message: i64, e: i64, n: i64) -> i64 {
    mod_exp(message, e, n)
}

fn decrypt(cipher: i64, d: i64, n: i64) -> i64 {
    mod_exp(cipher, d, n)
}

fn main() {
    let (n, e, d, _) = generate_keys();

    let mut input = String::new();
    println!("Введіть повідомлення (ціле число): ");
    std::io::stdin().read_line(&mut input).expect("Помилка при читанні введення");
    let message: i64 = input.trim().parse().expect("Введіть коректне ціле число");

    let cipher = encrypt(message, e, n);
    println!("Зашифроване повідомлення: {}", cipher);

    let decrypted_message = decrypt(cipher, d, n);
    println!("Розшифроване повідомлення: {}", decrypted_message);
}
