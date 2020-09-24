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
    SetMasterDetail(Entity, Entity),
    Expand,
    Collapse,
}

#[derive(Default, Clone, Debug, AsAny)]
pub struct MasterDetailState {
    content_container: Entity,
    master: Option<Entity>,
    detail: Option<Entity>,
    actions: VecDeque<MasterDetailAction>,
    expanded: bool,
    update: bool,
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

    // sets the master and detail widget (entity)
    fn set_master_detail(&mut self, ctx: &mut Context, master: Entity, detail: Entity) {
        ctx.clear_children_of(self.content_container);
        ctx.append_child_entity_to(master, self.content_container);
        ctx.build_context()
            .register_property::<usize>("column", master, 0);

        ctx.append_child_entity_to(detail, self.content_container);
        ctx.build_context()
            .register_property::<usize>("column", detail, 0);

        self.master = Some(master);
        self.detail = Some(detail);
    }

    // expands the widget (two column layout)
    fn expand(&mut self, ctx: &mut Context) {
        self.expanded = true;
        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Visible);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Visible);
            ctx.get_widget(detail).set::<usize>("column", 1);
        }

        Grid::columns_set(
            &mut ctx.get_widget(self.content_container),
            Columns::create().push(300).push("*").build(),
        );
    }

    // collapse the widget (one column layout)
    fn collapse(&mut self, ctx: &mut Context) {
        self.expanded = false;

        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Visible);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Collapsed);
            ctx.get_widget(detail).set::<usize>("column", 0);
        }
        Grid::columns_set(
            &mut ctx.get_widget(self.content_container),
            Columns::create().push("*").build(),
        );
    }
}

impl State for MasterDetailState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.update = true;
        self.content_container = ctx.child(CONTENT_CONTAINER).entity();
        self.update(registry, ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !self.update {
            return;
        }

        self.update = false;

        // handle state actions
        if let Some(action) = self.actions.pop_front() {
            match action {
                MasterDetailAction::ShowMaster => if !self.expanded {},
                MasterDetailAction::ShowDetail => if !self.expanded {},
                MasterDetailAction::SetMasterDetail(master, detail) => {
                    self.set_master_detail(ctx, master, detail)
                }
                MasterDetailAction::Expand => self.expand(ctx),
                MasterDetailAction::Collapse => self.collapse(ctx),
            }
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let width = ctx
            .get_widget(self.content_container)
            .get::<Rectangle>("bounds")
            .width();
        let break_point: f64 = *MasterDetail::break_point_ref(&ctx.widget());

        if self.expanded && width <= break_point {
            self.actions.push_front(MasterDetailAction::Collapse);

            // force update on next iteration
            ctx.send_window_request(WindowRequest::Redraw);
            ctx.push_event_strategy_by_entity(
                ActivateEvent(ctx.entity),
                ctx.entity,
                EventStrategy::Direct,
            );
        } else if !self.expanded && width > break_point {
            self.actions.push_front(MasterDetailAction::Expand);

            // force update on next iteration
            ctx.send_window_request(WindowRequest::Redraw);
            ctx.push_event_strategy_by_entity(
                ActivateEvent(ctx.entity),
                ctx.entity,
                EventStrategy::Direct,
            );
        }
    }
}

widget!(
    MasterDetail<MasterDetailState>: ActivateHandler {
        responsive: bool,

        break_point: f64
    }
);

impl MasterDetail {
    /// Register a master and a detail widget (entity).
    pub fn master_detail(mut self, master: Entity, detail: Entity) -> Self {
        self.state_mut()
            .actions
            .push_front(MasterDetailAction::SetMasterDetail(master, detail));
        self
    }
}

impl Template for MasterDetail {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MasterDetails")
            .child(Grid::new().id(CONTENT_CONTAINER).build(ctx))
            .on_activate(move |states, _| states.get_mut::<MasterDetailState>(id).update = true)
    }
}
