use wgpu;

use orbtk::prelude::*;

#[derive(Default, AsAny)]
struct MainState {

}

impl State for MainState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {

        // let _surface = wgpu::Surface::create(ctx);
   
    }
}

widget!(MainView<MainState>);

impl Template for MainView {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("MainView")
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - wgpu-rs example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}
