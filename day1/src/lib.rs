use std::num::ParseIntError;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Right(isize),
    Left(isize),
}
pub use Movement::*;

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_at(1);
        Ok(match left {
            "R" => Right(right.parse::<isize>()?),
            "L" => Left(right.parse::<isize>()?),
            _ => panic!("unexpected string"),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn distance_from(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}
pub use Direction::*;

impl Default for Direction {
    fn default() -> Self {
        North
    }
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" | "n" => North,
            "W" | "w" => West,
            "S" | "s" => South,
            "E" | "e" => East,
            _ => panic!("unexpected string"),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Bot {
    coord: Coord,
    facing: Direction,
}

impl Bot {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn distance_from_base(&self) -> usize {
        self.coord.distance_from(&Coord::new())
    }
}

impl Add<Movement> for Bot {
    type Output = Self;

    fn add(mut self, other: Movement) -> Self {
        match (other, self.facing) {
            (Right(n), North) | (Left(n), South) => {
                self.facing = East;
                self.coord.x += n;
            }
            (Left(n), North) | (Right(n), South) => {
                self.facing = West;
                self.coord.x -= n;
            }
            (Left(n), East) | (Right(n), West) => {
                self.facing = North;
                self.coord.y += n;
            }
            (Left(n), West) | (Right(n), East) => {
                self.facing = South;
                self.coord.y -= n;
            }
        }
        self
    }
}

impl AddAssign<Movement> for Bot {
    fn add_assign(&mut self, other: Movement) {
        *self = self.clone() + other;
    }
}
