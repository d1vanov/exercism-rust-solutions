pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    if n == 1 {
        return Some(0);
    }

    let mut steps = 0;
    let mut n = n;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        }
        else {
            n = n * 3 + 1;
        }
        steps += 1;
    }

    return Some(steps);
}
