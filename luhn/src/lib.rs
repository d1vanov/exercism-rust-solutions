/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut candidate: String = code.split_whitespace().collect();
    if candidate.len() <= 1 {
        return false;
    }

    candidate = candidate.chars().rev().collect();
    return match checksum_remainder(candidate.as_str()) {
        Some(cs) => (cs == 0),
        None => false
    }
}

fn addends(candidate: &str) -> Option<Vec<u32>> {
    let mut result: Vec<u32> = Vec::new();
    result.reserve_exact(candidate.len());
    for (i, c) in candidate.chars().enumerate() {
        let mut d = c.to_digit(10)?;
        if i == 0 || i % 2 == 0 {
            result.push(d);
            continue;
        }

        d *= 2;
        if d >= 10 {
            d -= 9;
        }

        result.push(d);
    }
    return Some(result);
}

fn checksum_remainder(input: &str) -> Option<u32> {
    let digits = addends(input)?;
    let cs: u32 = digits.iter().sum();
    return Some(cs % 10);
}
