use std::collections::VecDeque;

use crate::{api::prelude::*, proc_macros::*, shell::WindowRequest, Grid};

// --- KEYS --
static CONTENT_CONTAINER: &str = "id_content_container";
// --- KEYS --

// Describes operations on the master detail state.
#[derive(Debug, Clone, PartialEq)]
enum MasterDetailAction {
    ShowMaster,
    ShowDetail,
}

#[derive(Default, Clone, Debug, AsAny)]
pub struct MasterDetailState {
    content_container: Entity,
    master: Option<Entity>,
    detail: Option<Entity>,
    actions: VecDeque<MasterDetailAction>,
    expanded: bool,
}

impl MasterDetailState {
    /// Shows the master widget. If the master widget is visible nothing will happen.
    pub fn show_master(&mut self) {
        self.actions.push_front(MasterDetailAction::ShowMaster);
    }

    /// Shows the detail widget. If the detail widget is visible nothing will happen.
    pub fn show_detail(&mut self) {
        self.actions.push_front(MasterDetailAction::ShowDetail);
    }
}

impl State for MasterDetailState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.content_container = ctx.child(CONTENT_CONTAINER).entity();
        self.update(registry, ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        // register new master and detail widget
        if self.master.is_some() || self.detail.is_some() {
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

        // handle state actions
        if let Some(action) = self.actions.pop_front() {
            match action {
                MasterDetailAction::ShowMaster => if !self.expanded {},
                MasterDetailAction::ShowDetail => if !self.expanded {},
            }
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let width = ctx
            .get_widget(self.content_container)
            .get::<Rectangle>("bounds")
            .width();
        let break_point: f64 = *MasterDetail::break_point_ref(&ctx.widget());

        if self.expanded && width < break_point {
            // todo collapse
            // set master to visible and detail to collapsed
            println!("collapse");
            self.expanded = false;
            ctx.send_window_request(WindowRequest::Redraw);
        } else if !self.expanded && width > break_point {
            // todo expand
            // todo set both to visible
            println!("expand");
            self.expanded = true;
            ctx.send_window_request(WindowRequest::Redraw);
        }
    }
}

widget!(
    MasterDetail<MasterDetailState> {
        responsive: bool,

        break_point: f64
    }
);

impl MasterDetail {
    /// Register a master widget (entity) on the master detail widget.
    /// Only one master widget can be set. Only the last set widget will
    /// be the master widget on the master detail widget.
    pub fn master(mut self, master: Entity) -> Self {
        self.state_mut().master = Some(master);
        self
    }

    /// Register a detail widget (entity) on the master detail widget.
    /// Only one detail widget can be set. Only the last set widget will
    /// be the detail widget on the master detail widget.
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
