pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut first_char_of_current_word: Option<char> = None;
    let mut in_word = false;

    let chars: Vec<char> = phrase.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        let split_word_char = *c != '\'' && (c.is_whitespace() || c.is_ascii_punctuation());
        if in_word {
            if split_word_char {
                result.push(first_char_of_current_word.unwrap());
                first_char_of_current_word = None;
                in_word = false;
            } else {
                let mut capitalized_symbol_with_lower_left_neighbor = false;
                if i != 0 {
                    let previous_c = chars[i - 1];
                    capitalized_symbol_with_lower_left_neighbor =
                        c.is_uppercase() && !previous_c.is_uppercase();
                }

                if capitalized_symbol_with_lower_left_neighbor {
                    result.push(first_char_of_current_word.unwrap());
                    first_char_of_current_word = Some(*c);
                }
            }
        } else if !split_word_char {
            first_char_of_current_word = Some(c.to_ascii_uppercase());
            in_word = true;
        }
    }

    if let Some(c) = first_char_of_current_word {
        result.push(c);
    }

    result
}
