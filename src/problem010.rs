pub fn main() {
    let mut primes: u64 = 0;

    for i in 2..2000001 {
        if is_prime(i) {
            primes += i;
        }
    }

    println!("Sum of the primes below 2 million is: {}", primes);
}

fn is_prime(number: u64) -> bool {
    for i in 2..((number as f32).sqrt() as u64 + 1) { 
        if number % i == 0 {
            return false;
        }
    }

    return true;
}