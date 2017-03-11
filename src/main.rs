extern crate piston_window;
mod material;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [200, 100]
    )

    .exit_on_esc(true)
    .build()
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
