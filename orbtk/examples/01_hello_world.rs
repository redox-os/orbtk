pub use orbtk::*;

fn view(state: &mut String) -> Box<dyn Widget + 'static> {
    Box::new(TextBlock::new().text(state.clone()))
}

fn main() -> Result<(), Error> {
    let width = 600;
    let height = 480;

    App::new()
        .window(
            Window::create("Hello World".to_string())
                .title("OrbTk - 01_hello_world")
                .size(width, height)
                .centered(true)
                .view(view),
        )?
        .start()?;

    Ok(())
}
