use itertools::Itertools;

#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.descending().next()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.descending().take(3).collect()
    }

    fn descending(&self) -> impl Iterator<Item = u32> {
        self.0.iter().copied().sorted().rev()
    }
}
