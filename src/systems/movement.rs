use super::*;

#[system(par_for_each)]
pub fn movement(position: &mut Position, velocity: &Velocity) {
    *position += *velocity;
}
