pub trait Renderer {
    fn size(&self) -> (i32, i32);

    fn mv(&mut self, x: i32, y: i32);
    fn mv_x(&mut self, x: i32);
    fn mv_y(&mut self, y: i32);

    fn render_str(&mut self, line: &str);

    fn render_str_ln(&mut self, line: &str);
}

pub trait Render {
    fn render(&self, renderer: &mut dyn Renderer);
}
