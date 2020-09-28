use std::collections::VecDeque;

use crate::{api::prelude::*, proc_macros::*, Grid};

// --- KEYS --
static CONTENT_GRID: &str = "id_content_grid";
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

/// Handles request and layout changed of the `MasterDetail` widget.
#[derive(Default, Clone, Debug, AsAny)]
pub struct MasterDetailState {
    content_grid: Entity,
    master: Option<Entity>,
    detail: Option<Entity>,
    actions: VecDeque<MasterDetailAction>,
    expanded: bool,
    update: bool,
    event_adapter: EventAdapter,
}

impl MasterDetailState {
    /// Shows the master widget. If the master widget is visible nothing will happen.
    pub fn show_master(&mut self) {
        self.update = true;
        self.actions.push_front(MasterDetailAction::ShowMaster);
    }

    /// Shows the detail widget. If the detail widget is visible nothing will happen.
    pub fn show_detail(&mut self) {
        self.update = true;
        self.actions.push_front(MasterDetailAction::ShowDetail);
    }

    // sets the master and detail widget (entity)
    fn set_master_detail(&mut self, ctx: &mut Context, master: Entity, detail: Entity) {
        ctx.clear_children_of(self.content_grid);
        ctx.append_child_entity_to(master, self.content_grid);
        ctx.build_context()
            .register_property::<usize>("column", master, 0);

        ctx.append_child_entity_to(detail, self.content_grid);
        ctx.build_context()
            .register_property::<usize>("column", detail, 0);
        ctx.get_widget(detail)
            .set("visibility", Visibility::Collapsed);

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

        let master_width = *MasterDetail::master_width_ref(&ctx.widget());

        Grid::columns_set(
            &mut ctx.get_widget(self.content_grid),
            Columns::create().push(master_width).push("*").build(),
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
            &mut ctx.get_widget(self.content_grid),
            Columns::create().push("*").build(),
        );
    }

    fn int_show_master(&self, ctx: &mut Context) {
        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Visible);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Collapsed);
        }
    }

    fn int_show_detail(&self, ctx: &mut Context) {
        if let Some(master) = self.master {
            ctx.get_widget(master)
                .set("visibility", Visibility::Collapsed);
        }

        if let Some(detail) = self.detail {
            ctx.get_widget(detail)
                .set("visibility", Visibility::Visible);
        }
    }

    // force update on next iteration.
    fn force_next_update(&self, ctx: &mut Context) {
        self.event_adapter
            .push_event_direct(ctx.entity, ActivateEvent(ctx.entity));
    }
}

impl State for MasterDetailState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.update = true;
        self.content_grid = ctx.child(CONTENT_GRID).entity();
        self.event_adapter = ctx.event_adapter();
        self.update(registry, ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !self.update {
            return;
        }

        self.update = false;

        // handle state actions
        if let Some(action) = self.actions.pop_front() {
            let responsive = *MasterDetail::responsive_ref(&ctx.widget());

            match action {
                MasterDetailAction::ShowMaster => {
                    if !self.expanded || !responsive {
                        self.int_show_master(ctx);
                    }
                }
                MasterDetailAction::ShowDetail => {
                    if !self.expanded || !responsive {
                        self.int_show_detail(ctx);
                    }
                }
                MasterDetailAction::SetMasterDetail(master, detail) => {
                    self.set_master_detail(ctx, master, detail)
                }
                MasterDetailAction::Expand => self.expand(ctx),
                MasterDetailAction::Collapse => self.collapse(ctx),
            }
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !*MasterDetail::responsive_ref(&ctx.widget()) {
            return;
        }

        let width = ctx
            .get_widget(self.content_grid)
            .get::<Rectangle>("bounds")
            .width();
        let break_point: f64 = *MasterDetail::break_point_ref(&ctx.widget());

        if self.expanded && width <= break_point {
            self.actions.push_front(MasterDetailAction::Collapse);
            MasterDetail::navigation_visibility_set(&mut ctx.widget(), Visibility::Visible);
            self.force_next_update(ctx);
        } else if !self.expanded && width > break_point {
            self.actions.push_front(MasterDetailAction::Expand);
            MasterDetail::navigation_visibility_set(&mut ctx.widget(), Visibility::Hidden);
            self.force_next_update(ctx);
        }
    }
}

widget!(
    /// `MasterDetail` is a responsive navigation widget with a master child and a detail child.
    ///
    /// If `responsive` property is set to `true` if the width of the `MasterDetail` widget crosses the given `break_point` the widget switch between a one column
    /// and two column layout. On on column layout or if `responsive` is set to `false` navigation between master and details is possible.
    ///
    /// # Example
    ///
    /// ```rust
    /// MasterDetail::new()
    ///     .responsive(true)
    ///     .break_point(300)
    ///     .master_detail(TextBlock::new().text("Master").build(ctx), TextBlock::new().text("Detail").build(ctx))
    ///     .build(ctx);
    /// ```
    MasterDetail<MasterDetailState>: ActivateHandler, ShowMasterHandler, ShowDetailHandler {
        /// Describes if the change between a one and two column layout on the `break_point`.
        responsive: bool,

        /// Describes the switch point between the one and two column layout.
        break_point: f64,

        /// Describes the width of the master widget on `expanded` state.
        master_width: f64,

        /// Read the visibility of navigation. If `expanded` is `false` or `responsive` is false it's `Visible` otherwise `Hidden`.
        navigation_visibility: Visibility
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

    /// Shows the master widget. If the master widget is visible nothing will happen.
    pub fn show_master(ctx: &mut Context, entity: Entity) {
        MasterDetail::panics_on_wrong_type(&ctx.get_widget(entity));
        ctx.event_adapter()
            .push_event_direct(entity, ShowMasterEvent(entity));
    }

    /// Shows the detail widget. If the detail widget is visible nothing will happen.
    pub fn show_detail(ctx: &mut Context, entity: Entity) {
        MasterDetail::panics_on_wrong_type(&ctx.get_widget(entity));
        ctx.event_adapter()
            .push_event_direct(entity, ShowDetailEvent(entity));
    }
}

impl Template for MasterDetail {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MasterDetails")
            .master_width(374)
            .child(Grid::new().id(CONTENT_GRID).build(ctx))
            // used to force an update on the next iteration after post layout
            .on_activate(move |states, _| states.get_mut::<MasterDetailState>(id).update = true)
            .on_show_master(move |states, _| states.get_mut::<MasterDetailState>(id).show_master())
            .on_show_detail(move |states, _| states.get_mut::<MasterDetailState>(id).show_detail())
    }
}

orbtk_api::trigger_event!(
    ShowMasterEvent,
    ShowMasterEventHandler,
    ShowMasterHandler,
    on_show_master
);

orbtk_api::trigger_event!(
    ShowDetailEvent,
    ShowDetailEventHandler,
    ShowDetailHandler,
    on_show_detail
);
