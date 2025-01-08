use engine::piston_window::clear;
use engine::window;

fn main() {
    engine::setup();

    let mut window = window::Window::default();
    window.set_size((600.0, 600.0));
    window.set_name(format!("my super cool game"));
    let mut piston_window = window.build();

    while let Some(e) = piston_window.next() {
        piston_window.draw_2d(&e, |_c, g, _d| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}
