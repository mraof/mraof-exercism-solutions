pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut current = 3;
    while primes.len() <= n as usize {
        if !primes.iter().any(|p| (current % p) == 0) {
            primes.push(current);
        }
        current += 2;
    }
    primes[n as usize]
}
