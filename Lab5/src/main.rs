use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use rand::Rng;
use num_bigint::{BigUint, RandBigInt};

type HmacSha256 = Hmac<Sha256>;

fn generate_parameters() -> (BigUint, BigUint) {
    let p = BigUint::parse_bytes(b"23", 10).unwrap();
    let g = BigUint::parse_bytes(b"5", 10).unwrap();
    (p, g)
}

fn generate_keys(p: &BigUint, g: &BigUint) -> (BigUint, BigUint) {
    let mut rng = rand::thread_rng();
    let private_key = rng.gen_biguint(128);
    let public_key = g.modpow(&private_key, p);
    (private_key, public_key)
}

fn compute_shared_secret(private_key: &BigUint, other_public_key: &BigUint, p: &BigUint) -> BigUint {
    other_public_key.modpow(private_key, p)
}

fn sign_message(key: &[u8], message: &[u8]) -> Vec<u8> {
    // Використовуємо Mac::new_from_slice замість застарілого NewMac
    let mut mac = HmacSha256::new_from_slice(key).expect("Неправильний ключ для HMAC");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

fn verify_signature(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let mut mac = HmacSha256::new_from_slice(key).expect("Неправильний ключ для HMAC");
    mac.update(message);
    mac.verify_slice(signature).is_ok()
}

fn main() {

    let (p, g) = generate_parameters();
    println!("Спільні параметри: p = {}, g = {}", p, g);

    let (alice_private, alice_public) = generate_keys(&p, &g);
    let (bob_private, bob_public) = generate_keys(&p, &g);
    println!("Аліса: приватний ключ = {}, публічний ключ = {}", alice_private, alice_public);
    println!("Боб: приватний ключ = {}, публічний ключ = {}", bob_private, bob_public);

    let alice_shared_secret = compute_shared_secret(&alice_private, &bob_public, &p);
    let bob_shared_secret = compute_shared_secret(&bob_private, &alice_public, &p);
    println!("Аліса обчислює спільний секрет: {}", alice_shared_secret);
    println!("Боб обчислює спільний секрет: {}", bob_shared_secret);

    assert_eq!(alice_shared_secret, bob_shared_secret);
    let shared_key = &alice_shared_secret.to_bytes_be();

    let message = "Любіть Україну всім серцем своїм...".as_bytes();
    let signature = sign_message(shared_key, message);
    println!("Підписане повідомлення: {:?}", signature);

    let is_valid = verify_signature(shared_key, message, &signature);
    println!("Підпис валідний: {}", is_valid);
}
