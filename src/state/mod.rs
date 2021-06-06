use legion::Entity;
use std::collections::HashMap;

mod entity;

pub use entity::GameEntity;

/// Serialization of game state that is relevant for the frontend to process.
#[derive(Clone, Default, Debug)]
pub struct GameState {
    entities: HashMap<Entity, GameEntity>,
}

impl GameState {
    pub fn entities(&self) -> &HashMap<Entity, GameEntity> {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut HashMap<Entity, GameEntity> {
        &mut self.entities
    }
}
