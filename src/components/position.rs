use super::Velocity;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, other: Velocity) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add<Velocity> for Position {
    type Output = Self;

    fn add(mut self, other: Velocity) -> Self {
        self += other;
        self
    }
}
