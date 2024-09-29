#[test]

fn main() {
    let n = 5;
    let k = 3;
    let combinations = comb(n, k);

    for combination in combinations {
        println!("{:?}", combination);
    }
}
fn comb(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut combination = (0..k).collect::<Vec<_>>();

    loop {
        result.push(combination.clone());

        let mut i = k;
        while i > 0 {
            i -= 1;
            if combination[i] < n - k + i {
                combination[i] += 1;
                for j in i + 1..k {
                    combination[j] = combination[j - 1] + 1;
                }
                break;
            }
        }

        if i == 0 && combination[0] == n - k {
            break;
        }
    }

    result
}


