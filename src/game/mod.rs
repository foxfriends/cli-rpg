use crate::archetypes::*;
use crate::components::*;
use legion::World;

pub fn new_game(world: &mut World) {
    Player::new(world, Position { x: 0, y: 0 });
}
