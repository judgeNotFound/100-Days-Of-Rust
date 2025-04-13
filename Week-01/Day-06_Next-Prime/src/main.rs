use std::io;
use primes::{Sieve, PrimeSet};


fn calc_next_prime(num: u64) -> u64 {
    let mut sieve = Sieve::new();
    let (_idx, next_prime) = sieve.find(num);

    next_prime
}

fn main() {
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Could not read input.");

        let num: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let next_prime = calc_next_prime(num);

        println!("Next Prime: {}", next_prime);
    }
}
