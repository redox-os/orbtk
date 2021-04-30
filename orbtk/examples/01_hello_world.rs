pub use orbtk::*;

fn view(state: &mut String) -> Node {
    TextBlock::new()
        .font_family("Roboto Regular")
        .font_size(12)
        .text(state.clone())
        .into()
}

fn main() -> Result<(), Error> {
    let width = 600;
    let height = 480;

    App::new()
        .register_fonts(|font_loader| {
            font_loader
                .load_font_from_bytes("Roboto Regular", orbtk::fonts::ROBOTO_REGULAR_FONT)?;
            font_loader.load_font_from_bytes("Roboto Medium", orbtk::fonts::ROBOTO_MEDIUM_FONT)?;
            Ok(())
        })?
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
