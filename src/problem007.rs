pub fn main() {
    let mut counter: u32 = 1;
    let mut primes: u32 = 0;

    loop {
        if primes == 10001 {
            break;
        }

        counter += 1;

        if is_prime(counter) {
            primes += 1;
        }
    }

    println!("The 10001st prime is {}", counter);
}

fn is_prime(number: u32) -> bool {
    for i in 2..((number as f32).sqrt() as u32 + 1) { 
        if number % i == 0 {
            return false;
        }
    }

    return true;
}