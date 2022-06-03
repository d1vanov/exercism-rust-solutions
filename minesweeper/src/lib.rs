pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_count = minefield.len();
    if row_count == 0 {
        return vec![];
    }

    let column_count = minefield[0].len();
    if column_count == 0 {
        return vec![String::new()];
    }

    let mut counts = Vec::with_capacity(row_count + 2);
    for _ in 0..row_count + 2 {
        let mut counts_row = Vec::with_capacity(column_count + 2);
        for _ in 0..column_count + 2 {
            counts_row.push(0);
        }
        counts.push(counts_row);
    }

    for (x, s) in minefield.iter().enumerate() {
        for (y, c) in s.chars().enumerate() {
            match c {
                '*' => {
                    for i in 0..3 {
                        for j in 0..3 {
                            counts[x + i][y + j] += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let mut result = Vec::with_capacity(row_count);

    for (x, s) in minefield.iter().enumerate() {
        let mut row_chars = Vec::with_capacity(s.len());
        for (y, c) in s.chars().enumerate() {
            row_chars.push(match c {
                '*' => '*',
                ' ' if counts[x + 1][y + 1] != 0 => {
                    counts[x + 1][y + 1].to_string().chars().nth(0).unwrap()
                }
                _ => ' ',
            });
        }
        result.push(row_chars.into_iter().collect::<String>());
    }

    return result;
}
