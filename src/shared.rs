pub enum InputTypes {
    Example,
    MyInput,
}

impl InputTypes {
    pub fn to_file_name(&self) -> &str {
        match self {
            InputTypes::Example => "example.txt",
            InputTypes::MyInput => "input.txt",
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Tile {
    pub coordinate: Coordinate,
    pub contents: char,
}

pub struct Space2D<T>(pub Vec<Vec<T>>);
