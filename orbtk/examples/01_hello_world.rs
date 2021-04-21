pub use orbtk::*;

fn main() {
    if let Ok(mut window) = Window::create()
        .size(600, 480)
        .centered(true)
        .title("OrbTk: 01_hello_world")
        .build()
    {
        loop {
            let result = window.run();

            if result.is_err() || !result.unwrap() {
                break;
            }
        }
    }
}
