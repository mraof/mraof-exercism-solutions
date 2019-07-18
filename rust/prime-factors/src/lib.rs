use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    if n < 2 {
        return factors;
    }
    let mut primes = vec![2];
    let mut divisor = 2;
    let mut n = n;

    while n > divisor {
        if (n % divisor) == 0 {
            n /= divisor;
            factors.push(divisor)
        } else if let Some(d) = (3.max(divisor)..=n)
            .step_by(2)
            .find(|num| !primes.par_iter().any(|p| (num % p) == 0))
        {
            primes.push(d);
            divisor = d
        }
    }
    factors.push(n);
    factors
}
