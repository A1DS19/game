use piston_window::prelude::*;
use piston_window::{PistonWindow, Size as PistonSize, WindowSettings};

type Size = (f64, f64);

pub struct Window {
    size: PistonSize,
    name: String,
}

pub struct WindowBuilder {
    pub window: Window,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            window: Window {
                size: PistonSize {
                    height: 0.0,
                    width: 0.0,
                },
                name: "".to_string(),
            },
        }
    }

    pub fn set_size(mut self, size: &Size) -> Self {
        self.window.size = PistonSize {
            width: size.0,
            height: size.1,
        };
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.window.name = name.to_string();
        self
    }

    pub fn build(self) -> PistonWindow {
        let opengl = OpenGL::V4_5;

        WindowSettings::new(&self.window.name, self.window.size)
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .expect("Window building process failed")
    }
}
