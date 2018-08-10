pub fn main() {
    let mut multiples: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    
    for num in 1..1000  {
        if num % 3 == 0 || num % 5 == 0 {
            multiples.push(num);
            sum += num;
        }
    }
    
    //println!("{:?}", multiples);
    println!("Sum of the multiples of 3 and 5 in the range 1 to 1000 = {}", sum);
}