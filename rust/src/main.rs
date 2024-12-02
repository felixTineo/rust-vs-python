use std::time::Instant;

fn find_primes(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for num in 2..=limit {
        let is_prime = (2..=((num as f64).sqrt() as u64)).all(|i| num % i != 0);
        if is_prime {
            primes.push(num);
        }
    }
    primes
}

fn main() {
    let start = Instant::now();
    let primes = find_primes(10_000_000);
    let duration = start.elapsed();

    println!(
        "Encontrados {} primos en {:.2?} segundos.",
        primes.len(),
        duration
    );
}
