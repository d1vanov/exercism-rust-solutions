use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    let mut once_added_numbers : HashSet<u32> = HashSet::new();

    let mut number: u32 = 1;
    while number < limit {
        if once_added_numbers.contains(&number) {
            number += 1;
            continue;
        }

        for factor in factors {
            if *factor == 0 {
                continue;
            }

            if number % factor == 0 {
                sum += number;
                once_added_numbers.insert(number);
                break;
            }
        }

        number += 1;
    }

    return sum;
}
