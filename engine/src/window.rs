use piston_window::{PistonWindow, Size as PistonSize, WindowSettings};

type Size = (f64, f64);

pub struct Window {
    size: PistonSize,
    name: String,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            size: PistonSize {
                width: 0.0,
                height: 0.0,
            },
            name: format!(""),
        }
    }
}

impl Window {
    pub fn set_size(&mut self, size: Size) -> &Self {
        self.size = PistonSize {
            width: size.0,
            height: size.1,
        };
        self
    }

    pub fn set_name(&mut self, name: &str) -> &Self {
        self.name = name.to_string();
        self
    }

    pub fn build(&mut self) -> PistonWindow {
        WindowSettings::new(&self.name, self.size)
            .exit_on_esc(true)
            .build()
            .expect("Window building process failed")
    }
}
