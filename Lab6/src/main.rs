use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::{thread_rng, Rng};

// Функція для обчислення НСД
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if *b == BigInt::zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

// Розширений алгоритм Евкліда для знаходження оберненого значення
fn mod_inverse(e: &BigInt, phi: &BigInt) -> Option<BigInt> {
    let (mut t, mut new_t) = (BigInt::zero(), BigInt::one());
    let (mut r, mut new_r) = (phi.clone(), e.clone());

    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;

        // Обновлення t та r
        let temp_t = new_t.clone();
        new_t = &t - &quotient * &new_t;
        t = temp_t;

        let temp_r = new_r.clone();
        new_r = &r - &quotient * &new_r;
        r = temp_r;
    }

    if r > BigInt::one() {
        None // Не існує оберненого значення
    } else if t < BigInt::zero() {
        Some(t + phi)
    } else {
        Some(t)
    }
}

// Функція для генерації ключів RSA
fn generate_rsa_keys(p: BigInt, q: BigInt) -> Option<(BigInt, BigInt, BigInt)> {
    // Обчислення модуля n = p * q
    let n = &p * &q;

    // Обчислення φ(n) = (p - 1) * (q - 1)
    let phi = (&p - BigInt::one()) * (&q - BigInt::one());

    // Вибір значення e, яке є взаємно простим з φ(n)
    let mut rng = thread_rng();
    let mut e: BigInt;

    loop {
        // Генерація випадкового числа в межах [2, φ(n))
        let rand_num: u64 = rng.gen_range(2..=u64::MAX);
        e = rand_num.to_bigint().unwrap();

        // Переконатися, що e < φ(n)
        if &e >= &phi {
            continue;
        }

        // Переконуємось, що e взаємно просте з φ(n)
        if gcd(&e, &phi) == BigInt::one() {
            break;
        }
    }

    // Обчислення приватного ключа d
    let d = mod_inverse(&e, &phi)?;

    Some((e, d, n))
}

fn main() {
    // Встановлення значень p та q (мають бути великими простими числами)
    let p = 61.to_bigint().unwrap();
    let q = 53.to_bigint().unwrap();

    // Генерація ключів
    match generate_rsa_keys(p, q) {
        Some((e, d, n)) => {
            println!("Відкритий ключ (e, n): ({}, {})", e, n);
            println!("Приватний ключ (d, n): ({}, {})", d, n);
        }
        None => println!("Не вдалося знайти обернене значення."),
    }
}
