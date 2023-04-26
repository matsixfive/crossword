pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub letters: Vec<char>,
}

impl Grid {
    /// Returns the number of possible distinct places a word of length n can have in the grid
    pub fn num_of_combinations(&self, word_len: usize) -> usize {
        // possible positions the word can have in one line
        let horizontal_positions = if self.width < word_len {
            0
        } else {
            self.width - word_len + 1
        };
        // in the whole grid
        let horizontal_combinations = horizontal_positions * self.height;

        // possible positions the word can have in one column
        let vertical_positions = if self.height < word_len {
            0
        } else {
            self.height - word_len + 1
        };
        // in the whole grid
        let vertical_combinations = vertical_positions * self.height;

        let all_combinations = (horizontal_combinations
            + vertical_combinations
            + horizontal_positions * vertical_positions * 2)
            * 2;
        // * 2 for 4wards and backwards

        all_combinations
    }
}
