use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::<&'a str>::new();
    let word_lower : String = word.to_string().to_lowercase();
    let word_lower_counter = get_counter(&word_lower);
    for candidate in possible_anagrams {
        let candidate_lower: String = candidate.to_string().to_lowercase();
        if word_lower == candidate_lower {
            continue;
        }

        let candidate_lower_counter = get_counter(&candidate_lower);
        if word_lower_counter == candidate_lower_counter {
            result.insert(candidate);
        }
    }
    result
}

fn get_counter(word_lower: &str) -> HashMap<char, i32> {
    let mut map = HashMap::<char, i32>::new();
    for c in word_lower.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}
