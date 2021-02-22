use pancurses::Window;

pub(super) trait Render {
    fn render(&self, window: &Window);
}
