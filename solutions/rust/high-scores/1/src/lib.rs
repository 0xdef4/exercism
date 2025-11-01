#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.len() {
            1.. => {
                let mut highest_score = self.scores[0];
                for score in self.scores {
                    if highest_score < *score {
                        highest_score = *score
                    }
                }
                Some(highest_score)
            },
            0 => None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        match self.scores.len() {
            3.. => {
                let mut vec = self.scores.to_vec();
                vec.sort_by(|a,b| b.cmp(a));
                vec[0..3].to_vec()
            },
            2 => {
                let mut vec = self.scores.to_vec();
                vec.sort_by(|a,b| b.cmp(a));
                vec[0..2].to_vec()
            },
            0..2 => {
                let mut vec = self.scores.to_vec();
                vec.sort_by(|a,b| b.cmp(a));
                self.scores.to_vec()
            },
        }
    }
}
