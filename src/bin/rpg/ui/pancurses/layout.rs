use pancurses::Window;
use std::collections::HashMap;

pub(super) struct Layout {
    windows: HashMap<&'static str, Window>,
}

impl Layout {
    pub fn debug(window: &Window) -> Self {
        let mut windows = HashMap::new();
        let h = window.get_max_y();
        windows.insert("main", window.derwin(h - 1, 0, 0, 0).unwrap());
        windows.insert("debug", window.derwin(1, 0, h - 1, 0).unwrap());
        Self { windows }
    }

    #[allow(dead_code)]
    pub fn standard(window: &Window) -> Self {
        let mut windows = HashMap::new();
        let (h, w) = window.get_max_yx();
        windows.insert("main", window.subwin(0, 0, h, w).unwrap());
        Self { windows }
    }

    pub fn get(&self, window: &'static str) -> Option<&Window> {
        self.windows.get(&window)
    }
}
