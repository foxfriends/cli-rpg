#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Velocity {
    pub x: i64,
    pub y: i64,
}

impl Velocity {
    pub fn stop(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}
