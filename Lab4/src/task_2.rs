fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x1, y1) = extended_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (d, x, y)
    }
}
#[test]
fn main() {
    let a = 1263;
    let b = 204;
    let (d, x, y) = extended_gcd(a, b);
    println!("НСД({}, {}) = {}", a, b, d);
    println!("Розв'язок рівняння: {} * {} + {} * {} = {}", a, x, b, y, d);
}
