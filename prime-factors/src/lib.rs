pub fn factors(mut n: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    if n == 1 {
        return divisors;
    }

    let mut divisor: u64 = 2;
    while divisor <= n {
        let residual = n % divisor;
        if residual == 0 {
            divisors.push(divisor);
            if divisor == n {
                break;
            }
            n = n / divisor;
        }
        else {
            divisor += 1;
        }
    }
    return divisors;
}
