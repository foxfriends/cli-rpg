use pancurses::Window;
use rpg::Renderer;

pub(super) struct WindowRenderer<'a>(pub &'a Window);

impl Renderer for WindowRenderer<'_> {
    fn size(&self) -> (i32, i32) {
        let (h, w) = self.0.get_max_yx();
        (w, h)
    }

    fn mv(&mut self, x: i32, y: i32) {
        self.0.mv(y, x);
    }

    fn mv_x(&mut self, x: i32) {
        self.0.mv(self.0.get_cur_y(), x);
    }

    fn mv_y(&mut self, y: i32) {
        self.0.mv(y, self.0.get_cur_x());
    }

    fn render_str(&mut self, line: &str) {
        self.0.addstr(line);
    }

    fn render_str_ln(&mut self, line: &str) {
        let (y, x) = self.0.get_cur_yx();
        self.render_str(line);
        self.mv(x, y + 1);
    }
}
