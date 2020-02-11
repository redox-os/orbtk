use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

use super::behaviors::MouseBehavior;
use crate::{prelude::*, utils::SelectionMode as SelMode};

static ITEMS_PANEL: &'static str = "items_panel";
pub static LIST_VIEW: &'static str = "list_view";

#[derive(Default, AsAny)]
pub struct ListViewState {
    builder: WidgetBuildContext,
    count: usize,
    selected_entities: RefCell<HashSet<Entity>>,
    items_panel: Entity,
}

impl State for ListViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.items_panel = ctx
            .entity_of_child(ITEMS_PANEL)
            .expect("ListViewState.init: ItemsPanel child could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let count = ctx.widget().clone_or_default::<usize>("count");
        let entity = ctx.entity;

        if count != self.count {
            if let Some(builder) = &self.builder {
                ctx.clear_children_of(self.items_panel);

                for i in 0..count {
                    let item = {
                        let build_context = &mut ctx.build_context();
                        let child = builder(build_context, i);
                        let item = ListViewItem::create().build(build_context);

                        let mouse_behavior = MouseBehavior::create().build(build_context);
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
                        build_context.append_child(self.items_panel, item);
                        build_context.append_child(mouse_behavior, child);

                        item
                    };
                    ctx.get_widget(item).update_properties_by_theme();
                }
            }

            self.count = count;
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        for index in ctx
            .widget()
            .get::<SelectedEntities>("selected_entities")
            .0
            .clone()
            .symmetric_difference(&*self.selected_entities.borrow())
        {
            let mut widget = ctx.get_widget(*index);
            widget.set("selected", !widget.get::<bool>("selected"));

            widget.update_theme_by_state(false);
        }

        *self.selected_entities.borrow_mut() = ctx
            .widget()
            .get::<SelectedEntities>("selected_entities")
            .0
            .clone();
    }
}

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
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled") || !self.request_selection_toggle.get() {
            return;
        }
        self.request_selection_toggle.set(false);

        let selected = *ctx.widget().get::<bool>("selected");

        let entity = ctx.entity;
        let index = ctx.index_as_child(entity).unwrap();

        if let Some(parent) = &mut ctx.try_parent_from_id(LIST_VIEW) {
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
                || selection_mode == SelMode::None
            {
                return;
            }

            if selection_mode == SelMode::Single {
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
        }
    }
}

widget!(
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

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool
    }
);

impl Template for ListViewItem {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("ListViewItem")
            .min_width(64.0)
            .height(24.0)
            .selected(false)
            .pressed(false)
            .selector("list-view-item")
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(32.0)
            .font("Roboto Regular")
            .on_click(move |states, _| {
                states.get::<ListViewItemState>(id).toggle_selection();
                false
            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}

widget!(
    /// The `ListView` is an items drawer widget with selectable items.
    ///
    /// **CSS element:** `items-widget`
    ListView<ListViewState> {
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

        /// Sets or shared the count.
        count: usize,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the selection mode property.
        selection_mode: SelectionMode,

        /// Sets or shares the selected indices.
        selected_indices: SelectedIndices,

        /// Sets or shares the list of selected indices.
        selected_entities: SelectedEntities,

        /// Sets or shares the (wheel, scroll) delta property.
        delta: Point
    }
);

impl ListView {
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
        let items_panel = Stack::create()
            .vertical_alignment("start")
            .selector(Selector::default().id(ITEMS_PANEL))
            .orientation(id)
            .build(ctx);

        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("disabled", "auto"))
            .delta(id)
            .child(items_panel)
            .build(ctx);

        self.name("ListView")
            .selector(Selector::from("list-view").id(LIST_VIEW))
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_width(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .selection_mode("single")
            .selected_indices(HashSet::new())
            .selected_entities(HashSet::new())
            .delta(0.0)
            .orientation("vertical")
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
                    .padding(id)
                    .opacity(id)
                    .child(scroll_viewer)
                    .child(
                        ScrollIndicator::create()
                            .padding(2.0)
                            .content_id(items_panel.0)
                            .scroll_offset(scroll_viewer)
                            .opacity(id)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
