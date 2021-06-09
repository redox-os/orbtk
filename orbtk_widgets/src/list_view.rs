use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

use super::behaviors::MouseBehavior;
use crate::{api::prelude::*, prelude::*, proc_macros::*, themes::theme_orbtk::*};

static ITEMS_PANEL: &str = "items_panel";

/// The `ListViewState` generates the list box items and handles the selected indices.
#[derive(Default, AsAny)]
pub struct ListViewState {
    builder: WidgetBuildContext,
    count: usize,
    selected_entities: RefCell<HashSet<Entity>>,
    items_panel: Entity,
}

impl ListViewState {
    fn generate_items(&mut self, ctx: &mut Context) {
        let count = ctx.widget().clone_or_default::<usize>("count");
        let entity = ctx.entity();

        if count != self.count || *ctx.widget().get::<bool>("request_update") {
            ctx.widget().set("request_update", false);
            if let Some(builder) = &self.builder {
                ctx.clear_children_of(self.items_panel);

                for i in 0..count {
                    let item = {
                        let build_context = &mut ctx.build_context();
                        let child = builder(build_context, i);
                        let item = ListViewItem::new().parent(entity.0).build(build_context);

                        let mouse_behavior =
                            MouseBehavior::new().target(item.0).build(build_context);
                        build_context.register_shared_property::<Selector>(
                            "selector",
                            mouse_behavior,
                            item,
                        );
                        build_context.register_shared_property::<bool>(
                            "pressed",
                            mouse_behavior,
                            item,
                        );
                        build_context.append_child(item, mouse_behavior);

                        build_context.register_shared_property::<Brush>("foreground", child, item);
                        build_context.register_shared_property::<f32>("opacity", item, entity);
                        build_context.register_shared_property::<f32>("opacity", child, entity);
                        build_context.register_shared_property::<f64>("font_size", child, item);
                        build_context.register_shared_property::<f64>("font", child, item);
                        build_context.append_child(self.items_panel, item);
                        build_context.append_child(mouse_behavior, child);

                        item
                    };
                    ctx.get_widget(item).update_widget(entity, false, false);
                }
            }

            self.count = count;
        }
    }
}

impl State for ListViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.items_panel = ctx
            .entity_of_child(ITEMS_PANEL)
            .expect("ListViewState.init: ItemsPanel child could not be found.");

        self.generate_items(ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.generate_items(ctx);
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        for index in ctx
            .widget()
            .get::<SelectedEntities>("selected_entities")
            .0
            .clone()
            .symmetric_difference(&*self.selected_entities.borrow())
        {
            let mut widget = ctx.get_widget(*index);

            if !widget.has::<bool>("selected") {
                continue;
            }

            let selected = !widget.get::<bool>("selected");
            widget.set("selected", selected);

            if selected {
                widget
                    .get_mut::<Selector>("selector")
                    .push_state("selected");
            } else {
                widget
                    .get_mut::<Selector>("selector")
                    .remove_state("selected");
            }

            widget.update(false);
        }

        *self.selected_entities.borrow_mut() = ctx
            .widget()
            .get::<SelectedEntities>("selected_entities")
            .0
            .clone();
    }
}

/// The `ListViewItemState` handles the interaction and selection of a `ListViewItem`.
#[derive(Default, AsAny)]
pub struct ListViewItemState {
    request_selection_toggle: Cell<bool>,
}

impl ListViewItemState {
    fn toggle_selection(&self) {
        self.request_selection_toggle.set(true);
    }
}

impl State for ListViewItemState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if !ctx.widget().get::<bool>("enabled") || !self.request_selection_toggle.get() {
            return;
        }
        self.request_selection_toggle.set(false);

        let selected = *ctx.widget().get::<bool>("selected");

        let entity = ctx.entity();
        let index = ctx.index_as_child(entity).unwrap();

        let parent_entity: Entity = (*ctx.widget().get::<u32>("parent")).into();

        let mut parent = ctx.get_widget(parent_entity);

        let selection_mode = *parent.get::<SelectionMode>("selection_mode");
        // deselect item
        if selected {
            parent
                .get_mut::<SelectedEntities>("selected_entities")
                .0
                .remove(&entity);
            parent
                .get_mut::<SelectedIndices>("selected_indices")
                .0
                .remove(&index);
            return;
        }

        if parent
            .get::<SelectedEntities>("selected_entities")
            .0
            .contains(&entity)
            || selection_mode == SelectionMode::None
        {
            return;
        }

        if selection_mode == SelectionMode::Single {
            parent
                .get_mut::<SelectedEntities>("selected_entities")
                .0
                .clear();
            parent
                .get_mut::<SelectedIndices>("selected_indices")
                .0
                .clear();
        }

        parent
            .get_mut::<SelectedEntities>("selected_entities")
            .0
            .insert(entity);
        parent
            .get_mut::<SelectedIndices>("selected_indices")
            .0
            .insert(index);

        let selected_indices: Vec<usize> = parent
            .get::<SelectedIndices>("selected_indices")
            .0
            .iter()
            .copied()
            .collect();

        ctx.event_adapter().push_event_direct(
            parent_entity,
            SelectionChangedEvent(parent_entity, selected_indices),
        );
    }
}

widget!(
    /// The `ListViewItem` describes an item inside of a `ListView`.
    ///
    /// **style:** `list-view``
    ListViewItem<ListViewItemState>: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool,

        /// Sets or shares the parent id.
        parent: u32,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool
    }
);

impl Template for ListViewItem {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ListViewItem")
            .style("list_view_item")
            .min_width(64.0)
            .height(24.0)
            .selected(false)
            .pressed(false)
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(32.0)
            .font("Roboto-Regular")
            .on_click(move |states, _| {
                states.get::<ListViewItemState>(id).toggle_selection();
                false
            })
            .child(
                MouseBehavior::new()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .build(ctx),
            )
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        PaddingLayout::new().into()
    }
}

widget!(
    /// The `ListView` is an items drawer widget with selectable items.
    ///
    /// **style:** `items-widget`
    ListView<ListViewState> : SelectionChangedHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Sets or shares the item count.
        count: usize,

        /// Sets or shares the selection mode property.
        selection_mode: SelectionMode,

        /// Sets or shares the selected indices.
        selected_indices: SelectedIndices,

        /// Sets or shares the list of selected indices.
        selected_entities: SelectedEntities,

        /// Use this flag to force the redrawing of the items.
        request_update: bool
    }
);

impl ListView {
    /// Define the template build function for the content of the ListViewItems.
    pub fn items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static>(
        mut self,
        builder: F,
    ) -> Self {
        self.state_mut().builder = Some(Box::new(builder));
        self
    }
}

impl Template for ListView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let items_panel = Stack::new()
            .v_align("start")
            .id(ITEMS_PANEL)
            .orientation(id)
            .build(ctx);

        let scroll_viewer = ScrollViewer::new()
            .mode(("disabled", "auto"))
            .child(items_panel)
            .build(ctx);

        self.name("ListView")
            .style("list_view")
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_width(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .selection_mode("single")
            .selected_indices(HashSet::new())
            .selected_entities(HashSet::new())
            .orientation("vertical")
            .child(
                Container::new()
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
                    .padding(id)
                    .opacity(id)
                    .child(scroll_viewer)
                    .child(
                        ScrollIndicator::new()
                            .padding(2.0)
                            .content_bounds(("bounds", items_panel))
                            .view_port_bounds(("bounds", scroll_viewer))
                            .scroll_padding(("padding", scroll_viewer))
                            .mode(scroll_viewer)
                            .opacity(id)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
