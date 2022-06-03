use std::collections::HashMap;
use std::sync::mpsc;
use threadpool::ThreadPool;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    if input.is_empty() {
        return result;
    }

    let pool = ThreadPool::new(worker_count);

    let (tx, rx) = mpsc::channel();
    for i in 0..input.len() {
        let tx = tx.clone();
        let input_part = input[i].to_string();
        pool.execute(move|| {
            let mut partial_result = HashMap::new();
            for mut c in input_part.chars() {
                if c.is_ascii_punctuation() || c.is_ascii_digit() {
                    continue;
                }
                if c.is_ascii() {
                    c = c.to_ascii_lowercase();
                }
                let count = partial_result.entry(c).or_insert(0);
                *count += 1;
            }
            tx.send(partial_result).expect("Channel closed unexpectedly");
        });
    }

    for partial_result in rx.iter().take(input.len()) {
        for (c, partial_count) in partial_result {
            let count = result.entry(c).or_insert(0);
            *count += partial_count;
        }
    }

    result
}
