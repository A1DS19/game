use engine::piston_window::{clear, color, rectangle};
use engine::window::WindowBuilder;

const GAME_NAME: &str = "My super cool game";
const GAME_WINDOW: (f64, f64) = (1000.0, 1000.0);

fn main() {
    engine::setup();

    let mut window = WindowBuilder::new()
        .set_size(&GAME_WINDOW)
        .set_name(GAME_NAME)
        .build();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle(
                color::BLACK,
                rectangle::square(0.0, 0.0, 100.0),
                c.transform,
                g,
            );
        });
    }
}
