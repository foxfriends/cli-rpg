use crate::components::*;
use crate::{Render, Renderer};

#[derive(Clone, Debug)]
pub struct GameEntity {
    pub(crate) archetype: &'static str,
    pub(crate) position: Position,
    pub(crate) sprite: Sprite,
}

impl Render for GameEntity {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.mv(self.position.x, self.position.y);
        self.sprite.render(renderer);
    }
}
