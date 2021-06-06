use legion::World;

pub(crate) trait Archetype<T> {
    const ID: &'static str;

    fn new(world: &mut World, data: T);
}

mod player;

pub use player::Player;
