use piston_window::{Event, PistonWindow};

pub struct App {
    pub window: PistonWindow,
}

impl App {
    pub fn new(window: PistonWindow) -> Self {
        Self { window }
    }

    pub fn render<F>(&mut self, mut render: F)
    where
        F: FnMut(&mut PistonWindow, Event),
    {
        while let Some(e) = self.window.next() {
            render(&mut self.window, e);
        }
    }
}
