#[derive(Debug)]
pub struct HighScores<'a> {
    values: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores{
            values: &scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.values
    }

    pub fn latest(&self) -> Option<u32> {
        if self.values.is_empty() {
            return None;
        }

        Some(self.values[self.values.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if let Some(it) = self.values.iter().max() {
            return Some(*it);
        }

        return None;
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let values = {
            let mut values = self.values.to_vec();
            values.sort();
            values
        };

        let item_count = {
            let mut item_count = 3;
            if values.len() < item_count {
                item_count = values.len();
            }
            item_count
        };


        return {
            let mut result: Vec<u32> = Vec::new();
            result.reserve_exact(item_count);
            for i in 0..item_count {
                result.push(values[values.len() - 1 - i]);
            }
            result
        }
    }
}
