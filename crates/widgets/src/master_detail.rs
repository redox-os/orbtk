use crate::{api::prelude::*, proc_macros::*, Grid};

#[derive(Clone, Debug)]
pub enum MasterDetailAction {
    Master(Entity),
    Detail(Entity),
}

#[derive(Default, Clone, Debug, AsAny)]
pub struct MasterDetailState {
    // todo vec deref
    action: Option<MasterDetailAction>,
}

impl State for MasterDetailState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        // register master and detail as child
    }

    fn cleanup(&mut self, _registry: &mut Registry, _ctx: &mut Context) {}

    fn update(&mut self, _registry: &mut Registry, _ctx: &mut Context) {
        // if let Some(action) = self.action {
        //     match action {
        //         MasterDetailAction::Master(_) => {}
        //         MasterDetailAction::Detail(_) => {}
        //     }
        // }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {}
}

widget!(
    MasterDetail<MasterDetailState> {
        responsive: bool,

        break_point: f32
    }
);

impl MasterDetail {
    pub fn master(mut self, master: Entity) -> Self {
        // self.state_mut().master = Some(master);
        self
    }

    pub fn detail(mut self, detail: Entity) -> Self {
        // self.state_mut().detail = Some(detail);
        self
    }
}

impl Template for MasterDetail {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MasterDetails")
    }
}
