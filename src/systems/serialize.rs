use super::*;
use crate::state::{GameEntity, GameState};
use legion::world::SubWorld;
use legion::{maybe_changed, Entity, IntoQuery};
use std::collections::HashSet;

#[system]
#[read_component(Entity)]
#[read_component(Position)]
#[read_component(Sprite)]
pub fn serialize(world: &SubWorld, #[resource] state: &mut GameState) {
    let entities = state.entities_mut();
    let entities_alive: HashSet<_> = <(&Entity, &Position, &Sprite)>::query()
        .iter(world)
        .map(|(id, ..)| id)
        .collect();
    entities.retain(|id, _| entities_alive.contains(id));

    let mut query = <(&Entity, &Position, &Sprite)>::query()
        .filter(maybe_changed::<Position>() | maybe_changed::<Sprite>());
    for (id, position, sprite) in query.iter(world) {
        entities.insert(
            *id,
            GameEntity {
                sprite: sprite.clone(),
                position: *position,
            },
        );
    }
}
