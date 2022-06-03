pub fn nth(n: u32) -> u32 {
    let primes = generate_primes(1000005);
    primes[n as usize]
}

fn generate_primes(max_value: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    let mut is_prime: Vec<bool> = Vec::new();
    for _ in 0..max_value {
        is_prime.push(true);
    }
    let mut p: u32 = 2;
    while p * p < max_value {
        if is_prime[p as usize] {
            let mut i = p * p;
            while i < max_value {
                is_prime[i as usize] = false;
                i += p;
            }
        }

        p += 1;
    }
    p = 2;
    while p < max_value {
        if is_prime[p as usize] {
            primes.push(p);
        }

        p += 1;
    }
    primes
}
