#[derive(Debug)]
pub struct HighScores<'a>{
    //vec_scores: Vec<u32>,
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        // let mut sc = vec![0;scores.len()];
        // sc.copy_from_slice(scores);

        // Self {
        //     vec_scores : sc,
        // }
        Self{scores}
    }

    pub fn scores(&self) -> &[u32] {
        //&self.vec_scores
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        //self.vec_scores.last().copied()
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        //self.vec_scores.iter().max().copied()
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // let mut vec = vec![0;self.vec_scores.len()];
        // vec.copy_from_slice(&self.vec_scores);
        // vec.sort();
        // vec.reverse();
        // let mut len = 3;
        // if vec.len() < 3 {
        //     len = vec.len();
        // }
        // let mut ret = vec![0;len];
        // ret.copy_from_slice(&vec[0..len]);
        // ret
        let mut ret = self.scores.to_vec();
        ret.sort_unstable();
        ret.into_iter().rev().take(3).collect::<Vec<u32>>()
    }
}
