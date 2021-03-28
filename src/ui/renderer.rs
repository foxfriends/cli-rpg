pub trait Renderer {
    fn size(&self) -> (i64, i64);

    fn mv(&mut self, x: i64, y: i64);
    fn mv_x(&mut self, x: i64);
    fn mv_y(&mut self, y: i64);

    fn render_str(&mut self, line: &str);

    fn render_str_ln(&mut self, line: &str);
}

pub trait Render {
    fn render(&self, renderer: &mut dyn Renderer);
}
