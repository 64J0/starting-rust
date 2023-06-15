fn main() {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);
    println!("Top 3: {:?}", high_scores.personal_top_three());
}

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|n| *n)
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted = self.scores.to_vec();
        sorted.sort();
        sorted.last().map(|n| *n)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.to_vec();
        sorted.sort();
        sorted.reverse();

        if sorted.len() >= 3 {
            let (left, _) = sorted.split_at(3);
            left.to_vec()
        } else {
            sorted
        }
    }
}
