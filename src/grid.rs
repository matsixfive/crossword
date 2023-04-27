use crate::{Error, Result};
use std::{cmp, fmt::Display};

// mod word;
use crate::word::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub letters: Vec<char>,
    pub words: Vec<Word>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let letters = vec![' '; width * height];

        Grid {
            width,
            height,
            letters,
            words: Vec::new(),
        }
    }
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
            + horizontal_positions * vertical_positions * 2) // diagonals
            * 2;
        // * 2 for 4wards and backwards

        all_combinations
    }

    pub fn add_word(&mut self, word: Word) -> Result<()> {
        if self.test_word(&word) {
            let word_chars = &word.word.chars().collect::<Vec<char>>();
            for i in 0..word_chars.len() {
                match word.direction {
                    Direction::North => {
                        self.letters[(word.y - i) * self.height + word.x] = word_chars[i];
                    }
                    Direction::NorthEast => {
                        self.letters[(word.y - i) * self.height + word.x + i] = word_chars[i];
                    }
                    Direction::East => {
                        self.letters[word.y * self.height + word.x + i] = word_chars[i];
                    }
                    Direction::SouthEast => {
                        self.letters[(word.y + i) * self.height + word.x + i] = word_chars[i];
                    }
                    Direction::South => {
                        self.letters[(word.y + i) * self.height + word.x] = word_chars[i];
                    }
                    Direction::SouthWest => {
                        self.letters[(word.y + i) * self.height + word.x - i] = word_chars[i];
                    }
                    Direction::West => {
                        self.letters[word.y * self.height + word.x - i] = word_chars[i];
                    }
                    Direction::NorthWest => {
                        self.letters[(word.y - i) * self.height + word.x - i] = word_chars[i];
                    }
                }
            }
            self.words.push(word);
            return Ok(());
        } else {
            return Err(Error::from("word does not fit"));
        }
    }

    pub fn test_word(&self, word: &Word) -> bool {
        let word_chars = word.word.chars().collect::<Vec<char>>();

        match word.direction {
            Direction::North => {
                todo!()
            }
            Direction::NorthEast => {
                todo!()
            }
            Direction::East => {
                for (char_index, char) in word_chars.iter().enumerate() {
                    if self.letters[word.y * self.height + word.x + char_index] == ' ' {
                        continue;
                    }

                    if self.letters[word.y * self.height + word.x + char_index] != *char {
                        return false;
                    }
                }
            }
            Direction::SouthEast => todo!(),
            Direction::South => todo!(),
            Direction::SouthWest => todo!(),
            Direction::West => {
                for (char_index, char) in word_chars.iter().rev().enumerate() {
                    if self.letters[word.y * self.height + word.x + char_index] == ' ' {
                        continue;
                    }

                    if self.letters[word.y * self.height + word.x + char_index] != *char {
                        return false;
                    }
                }
            }
            Direction::NorthWest => todo!(),
        }

        true
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut characters: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.height {
            characters.push(vec!['a'; self.width]);
        }

        for word in &self.words {
            let word_chars = word.word.chars().collect::<Vec<char>>();
            for i in 0..word_chars.len() {
                let x = word.x;
                let y = word.y;
                match word.direction {
                    Direction::North => characters[y - i][x] = word_chars[i],
                    Direction::NorthEast => characters[y - i][x + i] = word_chars[i],
                    Direction::East => characters[y][x + i] = word_chars[i],
                    Direction::SouthEast => characters[y + i][x + i] = word_chars[i],
                    Direction::South => characters[y + i][x] = word_chars[i],
                    Direction::SouthWest => characters[y + i][x - i] = word_chars[i],
                    Direction::West => characters[y][x - i] = word_chars[i],
                    Direction::NorthWest => characters[y - i][x - i] = word_chars[i],
                }
            }
        }

        let word_list = self
            .words
            .iter()
            .map(|w| w.word.clone())
            .collect::<Vec<String>>();
        let word_columns_amount = word_list.len() / self.height
            + if word_list.len() % self.height == 0 {
                0
            } else {
                1
            };
        let mut word_columns: Vec<Vec<&str>> = Vec::new();
        for i in 0..word_columns_amount {
            word_columns.push(Vec::from(
                word_list[i * self.height..cmp::min((i + 1) * self.height, word_list.len())]
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
            ));
        }


        let mut top_str = String::new();
        top_str.push('┏');
        // top_str.push('╭');
        for _ in 0..(self.width * 2 + 1) {
            // top_str.push('─');
            top_str.push('━');
        }
        top_str.push('┱');
        // top_str.push('╮');
        println!("{}", top_str);

        for (line_num, line) in characters.iter().enumerate() {
            let mut line_string = String::new();

            line_string.push_str("┃ ");
            // line_string.push_str("│ ");

            for c in line {
                // write!(f, "{} ", c)?;
                line_string.push(*c);
                line_string.push(' ');
            }

            // write words
            for i in 0..word_columns_amount {
                if i == 0 {
                    line_string.push_str("┃  ");
                    // line_string.push_str("│  ");
                }

                if word_columns[i].len() > line_num {
                    line_string.push_str(word_columns[i][line_num]);
                } else {
                    continue;
                }

                let max_word_length = word_columns[i].iter().map(|c| c.len()).max().unwrap_or(0);
                // println!("{}", max_word_length);
                for _ in 0..((max_word_length - word_columns[i][line_num].len()) + 2) {
                    line_string.push(' ');
                }
            }

            writeln!(f, "{}", line_string);
        }

        let mut bot_str = String::new();
        bot_str.push('┗');
        // bot_str.push('╰');
        for _ in 0..(self.width * 2 + 1) {
            bot_str.push('━');
            // bot_str.push('─');
        }
        // bot_str.push('┛');
        bot_str.push('┹');
        // bot_str.push('╯');
        println!("{}", bot_str);

        write!(f, "")
    }
}
