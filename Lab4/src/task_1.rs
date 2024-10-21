fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
#[test]
fn main() {
    let a = 10946;
    let b = 2584;
    let result = gcd(a, b);
    println!("НСД({}, {}) = {}", a, b, result);
}