use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use crate::prelude::*;

use super::behaviors::{MouseBehavior, SelectionBehavior};

static CONTAINER: &'static str = "container";
static ITEMS_PANEL: &'static str = "items_panel";

type SelectedItem = Option<Entity>;

#[derive(Debug, Copy, Clone)]
enum Action {
    CheckMouseUpOutside { x: f64, y: f64 },
}

/// The `ComboBoxItemState` handles the interaction an selection of a `ComboBoxItem`.
#[derive(Default, AsAny)]
pub struct ComboBoxItemState {
    request_selection_toggle: Cell<bool>,
    index: usize,
    selected_container: Entity,
    combo_box: Entity,
    // ugly work around for item builder context clone, todo make it better 😉
    builder: Option<Arc<RefCell<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
}

impl ComboBoxItemState {
    fn toggle_selection(&self) {
        self.request_selection_toggle.set(true);
    }
}

impl State for ComboBoxItemState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let selected_index = ctx
            .get_widget(self.combo_box)
            .clone_or_default::<i32>("selected_index");
        let selected: bool = *ctx.widget().get("selected");

        if selected_index >= 0 && (selected_index as usize) == self.index && !selected {
            self.request_selection_toggle.set(true);
        }

        if !ctx.widget().get::<bool>("enabled") || !self.request_selection_toggle.get() {
            return;
        }
        self.request_selection_toggle.set(false);

        let entity = ctx.entity;

        // unselect previous selected item.
        if let Some(item) = ctx
            .get_widget(self.combo_box)
            .clone::<SelectedItem>("selected_item")
        {
            ctx.get_widget(item).set("selected", false);
            ctx.get_widget(item).update_theme_by_state(false);
        }

        ctx.widget().set("selected", true);
        ctx.widget().update_theme_by_state(false);
        ctx.get_widget(self.combo_box)
            .set("selected_index", self.index as i32);
        ctx.get_widget(self.combo_box)
            .set("selected_item", Some(entity));

        // Add selected content to combobox
        let index = self.index;
        let selected_container = self.selected_container;
        if let Some(builder) = &self.builder {
            ctx.clear_children_of(selected_container);
            let build_context = &mut ctx.build_context();
            let selected_content = builder.borrow()(build_context, index);
            build_context.append_child(selected_container, selected_content);
        }
    }
}

widget!(
    /// The `ComboBoxItem` describes an item inside of a `ComboBox`.
    ///
    /// **CSS element:** `combo_box_item``
    ComboBoxItem<ComboBoxItemState>: MouseHandler {
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
        selected: bool
    }
);

impl ComboBoxItem {
    fn selected_container(mut self, selected_container: impl Into<Entity>) -> Self {
        self.state_mut().selected_container = selected_container.into();
        self
    }

    fn index(mut self, index: usize) -> Self {
        self.state_mut().index = index;
        self
    }

    fn combo_box(mut self, combo_box: impl Into<Entity>) -> Self {
        self.state_mut().combo_box = combo_box.into();
        self
    }

    // Define the template build function for the selected content of the ComboBoxItems.
    fn items_builder(
        &mut self,
        builder: &Arc<RefCell<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>,
    ) {
        self.state_mut().builder = Some(builder.clone());
    }
}

impl Template for ComboBoxItem {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("ComboBoxItem")
            .min_width(64.0)
            .height(24.0)
            .selected(false)
            .pressed(false)
            .element("combo_box_item")
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(32.0)
            .font("Roboto Regular")
            .on_click(move |states, _| {
                states.get::<ComboBoxItemState>(id).toggle_selection();
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

/// The `ComboBoxState` is used to manipulate the position of the thumb of the slider widget.
#[derive(Default, AsAny)]
pub struct ComboBoxState {
    popup: Entity,
    action: Option<Action>,
    builder: Option<Arc<RefCell<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
    count: usize,
    items_panel: Entity,
    selected_container: Entity,
}

impl ComboBoxState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }

    // closes the popup on mouse up outside of the combobox and popup.
    fn close_popup(&mut self, ctx: &mut Context, x: f64, y: f64) {
        let combo_box_position = ctx.widget().clone::<Point>("position");
        let combo_box_bounds = ctx.widget().clone::<Rectangle>("bounds");

        let combo_box_global_bounds = Rectangle::new(
            combo_box_position.x,
            combo_box_position.y,
            combo_box_bounds.width,
            combo_box_bounds.height,
        );

        if !combo_box_global_bounds.contains((x, y)) {
            ctx.widget().set("selected", false);
            ctx.get_widget(self.popup)
                .set("visibility", Visibility::Collapsed);
            ctx.get_widget(self.popup).update_theme_by_state(false);
            ctx.widget().update_theme_by_state(false);
        }
    }
}

impl State for ComboBoxState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let count = ctx.widget().clone_or_default::<usize>("count");
        let entity = ctx.entity;

        // build the combobox items
        if count != self.count {
            if let Some(builder) = &self.builder {
                ctx.clear_children_of(self.items_panel);

                for i in 0..count {
                    let item = {
                        let build_context = &mut ctx.build_context();
                        let child = builder.borrow()(build_context, i);
                        let mut item = ComboBoxItem::create()
                            .index(i)
                            .combo_box(entity)
                            .selected_container(self.selected_container);

                        if let Some(builder) = &self.builder {
                            item.items_builder(builder);
                        }
                        let item = item.build(build_context);

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
        if self.action.is_none() || !(*ctx.widget().get::<bool>("selected")) {
            return;
        }

        if let Some(action) = self.action {
            match action {
                Action::CheckMouseUpOutside { x, y } => {
                    self.close_popup(ctx, x, y);
                }
            }
        }
    }
}

// todo use code of list view item, by create combobox item insert entity of popup container

widget!(
    /// The `ComboBox` represents an selection widget with a drop-down list.
    ///
    /// **CSS element:** `combo_box`
    ComboBox<ComboBoxState>: MouseHandler, ChangedHandler {
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

        /// Sets or shared the count.
        count: usize,

        /// Sets or shares the selected index. If the value is -1 no item is selected.
        selected_index: i32,

        /// The entity of the selected item.
        selected_item: SelectedItem,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the flag if the drop down is open.
        selected: bool
    }
);

impl ComboBox {
    /// Define the template build function for the content of the ComboBoxItems.
    pub fn items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static + Clone>(
        mut self,
        builder: F,
    ) -> Self {
        self.state_mut().builder = Some(Arc::new(RefCell::new(builder)));
        self
    }
}

impl Template for ComboBox {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let selected_container = Container::create().attach(Grid::column(0)).build(ctx);

        let container = Container::create()
            .id(CONTAINER)
            .background(id)
            .border_radius(id)
            .border_width(id)
            .border_brush(id)
            .padding(id)
            .child(
                Grid::create()
                    .columns(
                        Columns::create()
                            .column("*")
                            .column(4.0)
                            .column(14.0)
                            .build(),
                    )
                    .child(selected_container)
                    .child(
                        FontIconBlock::create()
                            .attach(Grid::column(2))
                            .vertical_alignment("center")
                            .icon(material_font_icons::ARROW_DOWN_ICON)
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .build(ctx);
        self.state_mut().selected_container = selected_container;

        let items_panel = Stack::create()
            .vertical_alignment("start")
            .id(ITEMS_PANEL)
            .orientation("vertical")
            .build(ctx);

        self.state_mut().items_panel = items_panel;
        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("disabled", "auto"))
            .child(items_panel)
            .build(ctx);

        let popup = Popup::create()
            .height(200.0)
            .open(("selected", id))
            .child(scroll_viewer)
            .child(
                ScrollIndicator::create()
                    .padding(2.0)
                    .content_id(items_panel.0)
                    .scroll_offset(scroll_viewer)
                    .opacity(id)
                    .build(ctx),
            )
            .target(container.0)
            .build(ctx);
        self.state_mut().popup = popup;

        let _ = ctx.append_child_to_overlay(popup);

        self.name("ComboBox")
            .element("combo_box")
            .height(32.0)
            .min_width(80.0)
            .selected(false)
            .selected_index(-1)
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .child(
                        SelectionBehavior::create()
                            .selected(id)
                            .enabled(id)
                            .target(id.0)
                            .child(container)
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_global_mouse_up(move |states, e| {
                states
                    .get_mut::<ComboBoxState>(id)
                    .action(Action::CheckMouseUpOutside { x: e.x, y: e.y })
            })
    }
}
