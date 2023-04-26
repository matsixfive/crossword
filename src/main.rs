mod grid;
use crate::grid::*;

fn main() {
    let grid = Grid { width:10, height:10, letters: Vec::new() };
    println!("{}", grid.num_of_combinations("hello".len()))
}
