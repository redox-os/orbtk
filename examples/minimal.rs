use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MyWidgetView::create().build(ctx))
                .build(ctx)
        })
        .run();

    #[derive(Default, AsAny)]
    pub struct MyWidgetState {}

    widget!(
        MyWidgetView<MyWidgetState> {
        }
    );

    impl Template for MyWidgetView {
        fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
            self.name("MyWidgetView")
                .child(Container::create().build(ctx))
        }
    }

    impl State for MyWidgetState {
        fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
            let maybe_child_entity = ctx.try_child_from_index(0);
            if maybe_child_entity.is_none() {
                return;
            }
            let child_entity = maybe_child_entity.unwrap().entity();

            dbg!("IN UPDATE: ", child_entity);
        }

        fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
            let maybe_child_entity = ctx.try_child_from_index(0);
            if maybe_child_entity.is_none() {
                dbg!("Not in ");
                return;
            }
            let child_entity = maybe_child_entity.unwrap().entity();

            dbg!("IN UPDATE_POST_LAYOUT: ", child_entity);
            ctx.remove_child(child_entity);
        }
    }
}
