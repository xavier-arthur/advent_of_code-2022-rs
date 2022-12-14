#[derive(Debug)]
pub struct Score {
    pub range: [char; 52]
}

impl Score {
    pub fn new() -> Self {
        let mut score_chars: [char; 52] = [0 as char; 52];

        for (i, val) in ('a'..='z').enumerate() {
            score_chars[i] = val;
        }

        for (i, val) in ('A'..='Z').enumerate() {
            score_chars[i + 26] = val;
        }
        
        Self { range: score_chars }
    }

    pub fn get_points(&self, ch: char) -> i32 {
        get_index_of(&ch, &self.range).unwrap() as i32 + 1
    }
}

fn contains(needle: &char, haysack: &[char]) -> bool {
    for i in 0..haysack.len() {
        if haysack[i] == *needle {
            return true;
        }
    }
    false
}

fn get_index_of(needle: &char, haysack: &[char]) -> Option<usize> {
    for i in 0..haysack.len() {
        if haysack[i] == *needle {
            return Some(i);
        }
    }
    None
}

pub fn get_duplicates(vec: &mut Vec<char>) -> i32 {
    let score = Score::new();
    let mut accounted_duplicates: Vec<char> = vec![];
    let mut total_score: i32 = 0;

    let rucksack_one: Vec<char> = vec.splice(0..vec.len() / 2, None).collect();

    for value in rucksack_one {
        if contains(&value, vec) && !accounted_duplicates.contains(&value) {
            let points = get_index_of(&value, &score.range).unwrap() + 1;
            accounted_duplicates.push(value.clone());
            total_score += points as i32;
        }
    }

    total_score
}