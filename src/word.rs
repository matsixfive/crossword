#[allow(unused)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct Word {
    pub word: String,
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}
