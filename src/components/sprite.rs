use crate::{Render, Renderer};

#[derive(Clone, Debug)]
pub struct Sprite {
    text: Vec<String>,
}

impl<S> From<S> for Sprite
where
    String: From<S>,
{
    fn from(s: S) -> Self {
        Sprite {
            text: String::from(s).lines().map(|s| s.to_owned()).collect(),
        }
    }
}

impl Render for Sprite {
    fn render(&self, renderer: &mut dyn Renderer) {
        for line in &self.text {
            renderer.render_str_ln(line);
        }
    }
}
