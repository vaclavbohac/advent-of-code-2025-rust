
#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Rotation {
    direction: Direction,
    diff: i32,
}

impl Rotation {
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn diff(&self) -> i32 {
        self.diff
    }
}

pub fn parse_rotation(rotation: &str) -> Rotation {
    let value = rotation[1..].parse::<i32>().unwrap();

    match rotation {
        x if x.starts_with("R") => Rotation {
            direction: Direction::Right,
            diff: value,
        },
        x if x.starts_with("L") => Rotation {
            direction: Direction::Left,
            diff: value,
        },
        _ => Rotation {
            direction: Direction::Left,
            diff: 0,
        },
    }
}
