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

impl Coordinate {
    pub fn get_next_in_direction(&self, direction: &Direction) -> Self {
        let coordinate = match direction {
            Direction::Up => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
        };
        return coordinate;
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Coordinate3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate3D {
    pub fn distance_to_point(&self, other: &Coordinate3D) -> f64 {
        let delta_x = (self.x - other.x) as f64;
        let delta_y = (self.y - other.y) as f64;
        let delta_z = (self.z - other.z) as f64;

        let sum_of_squares = delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2);

        return sum_of_squares.sqrt();
    }
}

#[derive(Debug)]
pub struct Tile {
    pub coordinate: Coordinate,
    pub contents: char,
}

pub struct Space2D<T>(pub Vec<Vec<T>>);

impl<T> Space2D<T> {
    pub fn get_coordinate(&self, coordinate: &Coordinate) -> Option<&T> {
        if let Some(row) = self.0.get(coordinate.y as usize) {
            return row.get(coordinate.x as usize);
        }
        return None;
    }

    pub fn get_direction_from_coordinate(
        &self,
        coordinate: &Coordinate,
        direction: &Direction,
    ) -> Option<&T> {
        return self.get_coordinate(&coordinate.get_next_in_direction(direction));
    }
}
