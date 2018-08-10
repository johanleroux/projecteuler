pub fn main() {
    let mut number: u32 = 1;
    let mut divible: bool;

    loop {
        divible = true;

        for i in 1..21 {
            if number % i != 0 {
                divible = false;
                break;
            }
        }

        if divible {    
            println!("Smallest number evenly divisible through 1-20: {}", number);
            break;
        }

        number += 1;
    }
}
