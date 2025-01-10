use engine::app::App;
use engine::piston_window::{clear, color, rectangle, Event, Transformed};
use engine::window::WindowBuilder;

const GAME_NAME: &str = "My super cool game";
const GAME_WINDOW: (f64, f64) = (1000.0, 1000.0);

fn main() {
    engine::setup();

    let mut window = WindowBuilder::new()
        .set_size(&GAME_WINDOW)
        .set_name(GAME_NAME)
        .build();

    let rotation_angle = std::f64::consts::PI / 4.0;

    App::new(window).render(|window, e| {
        window.draw_2d(&e, |c, g, _d| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            let points: Vec<(f64, f64)> = vec![(0.0, 0.0), (100.0, 100.0), (200.0, 200.0)];

            for (x, y) in points {
                let transform = c
                    .transform
                    .trans(x, y)
                    .rot_rad(rotation_angle)
                    .trans(-50.0, -50.0);

                rectangle(
                    color::BLACK,
                    rectangle::square(0.0, 0.0, 100.0),
                    transform,
                    g,
                );
            }
        });
    });
}
