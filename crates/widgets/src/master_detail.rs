use crate::{api::prelude::*, proc_macros::*, Grid};

// --- KEYS --
static CONTENT_CONTAINER: &str = "id_content_container";
// --- KEYS --

#[derive(Default, Clone, Debug, AsAny)]
pub struct MasterDetailState {
    content_container: Entity,
    master: Option<Entity>,
    detail: Option<Entity>,
}

impl State for MasterDetailState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.content_container = ctx.child(CONTENT_CONTAINER).entity();
        self.update(registry, ctx);
    }

    fn cleanup(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if self.master.is_none() && self.detail.is_none() {
            return;
        }

        ctx.clear_children_of(self.content_container);

        if let Some(master) = self.master {
            ctx.append_child_entity_to(master, self.content_container);
            self.master = None;
        }

        if let Some(detail) = self.detail {
            ctx.append_child_entity_to(detail, self.content_container);
            self.detail = None;
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {}
}

widget!(
    MasterDetail<MasterDetailState> {
        responsive: bool,

        break_point: f64
    }
);

impl MasterDetail {
    pub fn master(mut self, master: Entity) -> Self {
        self.state_mut().master = Some(master);
        self
    }

    pub fn detail(mut self, detail: Entity) -> Self {
        self.state_mut().detail = Some(detail);
        self
    }
}

impl Template for MasterDetail {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MasterDetails")
            .child(Grid::new().id(CONTENT_CONTAINER).build(ctx))
    }
}
