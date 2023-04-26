mod grid;
mod word;
use crate::grid::*;

fn main() {
    let mut grid = Grid::new(20, 4);
    println!("{}", grid.num_of_combinations("hello".len()));
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "yea".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "hello".to_string(), x: 3, y: 2, direction: word::Direction::East });
    grid.add_word(word::Word { word: "yap".to_string(), x: 4, y: 1, direction: word::Direction::South });
    println!("{}", grid);

}
