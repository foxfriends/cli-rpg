use super::*;
use crate::components::*;

pub struct Player;

impl Archetype<Position> for Player {
    const ID: &'static str = "Player";

    fn new(world: &mut World, position: Position) {
        world.push((ArchetypeId::new(Self::ID), position, Velocity::default()));
    }
}
