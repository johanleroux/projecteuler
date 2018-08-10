extern crate num_integer;

pub fn main() {
    let factors: Vec<u64> = factors(600851475143);
    println!("Factors: {:?}", factors);

    let primes: Vec<u64> = primes(factors);
    println!("Prime Factors: {:?}", primes);
    
    println!("Highest Prime Factors: {}", primes[primes.len()-1]);
}

fn factors(number: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
 
    for i in 1..((number as f32).sqrt() as u64 + 1) { 
        if number % i == 0 {
            factors.push(i);
            factors.push(number/i);
        }
    }
    factors.sort();
    return factors;
}

fn primes(numbers: Vec<u64>) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();

    for &x in numbers.iter() {
        if is_prime(x) {
            primes.push(x);
        }
    }

    return primes;
}

fn is_prime(number: u64) -> bool {
    for i in 2..((number as f32).sqrt() as u64 + 1) { 
        if number % i == 0 {
            return false;
        }
    }

    return true;
}