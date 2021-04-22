pub use orbtk::*;

fn main() -> Result<(), Error> {
    let width = 600;
    let height = 480;

    App::new()
        .window(
            Window::create()
                .title("OrbTk - 01_hello_world")
                .size(width, height)
                .centered(true)
                .ui(Ui::new("Hello World".to_string())),
        )?
        .start()?;

    Ok(())
}
