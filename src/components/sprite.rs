use crate::{Render, Renderer};

#[derive(Clone, Debug)]
pub struct Sprite {
    text: Vec<String>,
}

impl Render for Sprite {
    fn render(&self, renderer: &mut dyn Renderer) {
        for line in &self.text {
            renderer.render_str_ln(line);
        }
    }
}
