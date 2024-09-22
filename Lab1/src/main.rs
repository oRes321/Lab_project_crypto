
fn main()
{

    let digits = [1,2,3,4,5,6];
    let mut count_with_repeats = 0;
    for &tens in &digits{
        for &ones in &digits{
            count_with_repeats += 1;
        }
    }

    println!("Numbers with repeating digits: {} ", count_with_repeats);


    let  mut count_without_repeats = 0;
    for &tens in &digits{
        for &ones in &digits{
            if tens != ones{
                count_without_repeats += 1;
            }
        }
    }

    println!("Numbers without repeating digits: {}", count_without_repeats);

}