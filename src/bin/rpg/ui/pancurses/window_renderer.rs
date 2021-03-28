use pancurses::Window;
use rpg::Renderer;

pub(super) struct WindowRenderer<'a>(pub &'a Window);

impl Renderer for WindowRenderer<'_> {
    fn size(&self) -> (i64, i64) {
        let (h, w) = self.0.get_max_yx();
        (w as i64, h as i64)
    }

    fn mv(&mut self, x: i64, y: i64) {
        self.0.mv(y as i32, x as i32);
    }

    fn mv_x(&mut self, x: i64) {
        self.0.mv(self.0.get_cur_y(), x as i32);
    }

    fn mv_y(&mut self, y: i64) {
        self.0.mv(y as i32, self.0.get_cur_x());
    }

    fn render_str(&mut self, line: &str) {
        self.0.addstr(line);
    }

    fn render_str_ln(&mut self, line: &str) {
        let (y, x) = self.0.get_cur_yx();
        self.render_str(line);
        self.0.mv(y + 1, x);
    }
}
