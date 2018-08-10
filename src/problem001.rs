extern crate num_integer;

pub fn main() {
    let mut multiples: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    
    for num in 1..1000  {
        let (_div3, _rem3) = num_integer::div_rem(num, 3);
        let (_div5, _rem5) = num_integer::div_rem(num, 5);

        if _rem3 == 0 || _rem5 == 0 {
            multiples.push(num);
            sum += num;
        }
    }

    // for x in &multiples {
    //     println!("{}", x);
    // }

    println!("Sum of the multiples of 3 and 5 in the range 1 to 1000 = {}", sum);
}