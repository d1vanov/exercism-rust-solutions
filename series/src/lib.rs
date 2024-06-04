pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        panic!("Substring of zero length cannot be returned");
    }

    let chars = digits.chars().collect::<Vec<_>>();
    if chars.len() < len {
        return vec![];
    }

    let mut result: Vec<String> = vec![];
    for i in 0..chars.len() - len + 1 {
        let mut current_series = vec![];
        for j in i..len + i {
            current_series.push(chars[j])
        }
        result.push(current_series.iter().collect::<String>());
    }

    result
}
