use itertools::Itertools;

fn main()
{
    let digits = vec![1,2,3,4,5,];
    let mut count = 0;

    for perm in  digits.iter().permutations(5)
    {
        let  number = perm.iter().fold(0, |acc , &&x | acc * 10 + x);
        if perm[4] != &5
        {
            count +=1;
        }
    }
    println!("Number of numbers that are not multiples 5: {}", count);

}