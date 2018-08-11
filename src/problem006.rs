pub fn main() {
    let mut sum_of_square: u32 = 0;
    let mut square_of_sum: u32 = 0;
    let difference: u32;

    for i in 1..101 {
        sum_of_square += (i as u32).pow(2);
        square_of_sum += i;
    }

    square_of_sum = square_of_sum.pow(2);
    difference = square_of_sum - sum_of_square;

    println!("{}", difference)
}
