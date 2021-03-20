use super::*;

#[system(par_for_each)]
#[filter(component::<Controllable>())]
pub fn acceleration(#[state] dx: &i64, #[state] dy: &i64, velocity: &mut Velocity) {
    velocity.x += dx;
    velocity.y += dy;
}
