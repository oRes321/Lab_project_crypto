use std::collections::HashSet;
use std::io;
#[test]

fn main() {

    println!("Множина — це набір унікальних елементів. Операції над множинами включають:");
    println!("1. Об'єднання (union): всі елементи з обох множин.");
    println!("2. Перетин (intersection): елементи, які є в обох множинах.");
    println!("3. Різниця (difference): елементи, що є лише в одній множині.");
    println!("4. Симетрична різниця (symmetric difference): елементи, які є в одній множині, але не в обох.");

    println!("Введіть елементи першої множини через кому:");
    let set1 = read_set();
    println!("Введіть елементи другої множини через кому:");
    let set2 = read_set();

    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();

    println!("Об'єднання: {:?}", union);
    println!("Перетин: {:?}", intersection);
    println!("Різниця (перша множина без другої): {:?}", difference);
    println!("Симетрична різниця: {:?}", symmetric_difference);
}

fn read_set() -> HashSet<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка зчитування");
    input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}