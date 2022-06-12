extern crate itertools;
extern crate permutohedron;

use itertools::Itertools;
use permutohedron::Heap;
use std::collections::HashMap;

struct Alphametics {
    pattern_lhs: Vec<String>,
    pattern_rhs: String,
    characters: Vec<char>,
    leading_characters: Vec<char>,
    char_to_digit_map: HashMap<char, u8>,
}

impl Alphametics {
    pub fn new(input: &str) -> Self {
        let mut pattern: Vec<String> = input
            .split("==")
            .map(|str| str.trim().to_string())
            .collect();
        let pattern_rhs = pattern.pop().unwrap().to_string();
        let pattern_lhs = pattern[0]
            .split('+')
            .map(|str| str.trim().to_string())
            .collect::<Vec<_>>();
        let mut characters: Vec<char> = input.chars().filter(|chr| chr.is_alphabetic()).collect();
        let leading_characters: Vec<char> = pattern_lhs
            .iter()
            .map(|word| word.chars().nth(0).unwrap())
            .collect();
        characters.sort();
        characters.dedup();
        let mut char_to_digit_map = HashMap::new();
        for &chr in &characters {
            char_to_digit_map.insert(chr, 0u8);
        }

        Alphametics {
            pattern_lhs,
            pattern_rhs,
            characters,
            leading_characters,
            char_to_digit_map,
        }
    }
    fn match_found(&self) -> bool {
        let rhs = self.evaluate(&self.pattern_rhs);

        let solution_length = rhs.to_string().len();
        let expected_solution_length = self.pattern_rhs.len();
        if solution_length != expected_solution_length {
            return false;
        }

        let lhs_result = self.evaluate_list(&self.pattern_lhs);
        lhs_result == rhs
    }
    fn update_char_to_digit_map(&mut self, digits: &[usize]) {
        let characters: Vec<char> = self.characters.clone();
        characters
            .into_iter()
            .zip(digits.into_iter())
            .for_each(|(key, digit)| {
                let _ = self.char_to_digit_map.insert(key, *digit as u8);
            });
    }
    fn has_zero_leading_character(&self) -> bool {
        self.leading_characters
            .iter()
            .any(|chr| &self.char_to_digit_map[chr] == &0)
    }
    fn evaluate(&self, text: &str) -> u64 {
        let digits: Vec<String> = text
            .chars()
            .map(|chr| {
                let digit = *&self.char_to_digit_map[&chr];
                digit.to_string()
            })
            .collect();
        digits.join("").parse::<u64>().unwrap()
    }
    fn evaluate_list(&self, variables: &[String]) -> u64 {
        variables
            .iter()
            .map(|variable| self.evaluate(variable))
            .sum()
    }
    fn find_match(&mut self) -> Option<HashMap<char, u8>> {
        let character_count = self.characters.len();
        for mut digits in (0..10).combinations(character_count) {
            let heap = Heap::new(&mut digits);
            for permutation in heap {
                self.update_char_to_digit_map(&permutation);
                if self.has_zero_leading_character() {
                    continue;
                }
                if self.match_found() {
                    let characters_iter = self.characters.clone().into_iter();
                    let pairs: HashMap<char, u8> = characters_iter
                        .zip(permutation.iter())
                        .map(|(chr, digit)| (chr, *digit as u8))
                        .collect();
                    return Some(pairs);
                }
            }
        }
        None
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametics = Alphametics::new(input);
    alphametics.find_match()
}
