use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

use crate::{prelude::*, utils::SelectionMode as SelMode};

use super::behaviors::{MouseBehavior, SelectionBehavior};

static CONTAINER: &'static str = "container";
static ITEMS_PANEL: &'static str = "items_panel";

type SelectedItem = Option<Entity>;

#[derive(Default, AsAny)]
pub struct ComboBoxItemState {
    request_selection_toggle: Cell<bool>,
    selected_container: Entity,
    combo_box: Entity,
    builder: WidgetBuildContext,
}

impl ComboBoxItemState {
    fn toggle_selection(&self) {
        self.request_selection_toggle.set(true);
    }
}

impl State for ComboBoxItemState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled") || !self.request_selection_toggle.get() {
            return;
        }
        self.request_selection_toggle.set(false);

        // let selected = *ctx.widget().get::<bool>("selected");

        let entity = ctx.entity;
        let index: u32 = *ctx.widget().get("index");

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
            .set("selected_index", index as i32);
        ctx.get_widget(self.combo_box)
            .set("selected_item", Some(entity));
        // let index = ctx.index_as_child(entity).unwrap();

        // if let Some(parent) = &mut ctx.get_widget(self.combo_box) {
        //     // let selection_mode = *parent.get::<SelectionMode>("selection_mode");
        //     // deselect item
        //     if selected {
        //         // parent
        //         //     .get_mut::<SelectedEntities>("selected_entities")
        //         //     .0
        //         //     .remove(&entity);
        //         // parent
        //         //     .get_mut::<SelectedIndices>("selected_indices")
        //         //     .0
        //         //     .remove(&index);
        //         return;
        //     }

        //     // if parent
        //     //     .get::<SelectedEntities>("selected_entities")
        //     //     .0
        //     //     .contains(&entity)
        //     //     || selection_mode == SelMode::None
        //     // {
        //     //     return;
        //     // }

        //     // if selection_mode == SelMode::Single {
        //     //     parent
        //     //         .get_mut::<SelectedEntities>("selected_entities")
        //     //         .0
        //     //         .clear();
        //     //     parent
        //     //         .get_mut::<SelectedIndices>("selected_indices")
        //     //         .0
        //     //         .clear();
        //     // }

        //     // parent
        //     //     .get_mut::<SelectedEntities>("selected_entities")
        //     //     .0
        //     //     .insert(entity);
        //     // parent
        //     //     .get_mut::<SelectedIndices>("selected_indices")
        //     //     .0
        //     //     .insert(index);
        // }
    }
}

widget!(
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

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool,

        /// Sets or shares the index inside of the combobox item collection.
        index: u32
    }
);

impl ComboBoxItem {
    fn selected_container(mut self, selected_container: impl Into<Entity>) -> Self {
        self.state_mut().selected_container = selected_container.into();
        self
    }

    fn combo_box(mut self, combo_box: impl Into<Entity>) -> Self {
        self.state_mut().combo_box = combo_box.into();
        self
    }

    // Define the template build function for the selected content of the ComboBoxItems.
    fn items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static + Clone>(
        mut self,
        builder: F,
    ) -> Self {
        self.state_mut().builder = Some(Box::new(builder));
        self
    }
}

impl Template for ComboBoxItem {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("ComboBoxItem")
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
    builder: WidgetBuildContext,
    count: usize,
    items_panel: Entity,
    selected_container: Entity,
}

impl State for ComboBoxState {
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
                        let item = ComboBoxItem::create()
                            .index(i as u32)
                            .combo_box(entity)
                            .selected_container(self.selected_container);

                        // if let Some(builder) = self.builder {
                        //     item.items_builder((*builder).clone());
                        // }
                        
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
        // for index in ctx
        //     .widget()
        //     .get::<SelectedEntities>("selected_entities")
        //     .0
        //     .clone()
        //     .symmetric_difference(&*self.selected_entities.borrow())
        // {
        //     let mut widget = ctx.get_widget(*index);
        //     widget.set("selected", !widget.get::<bool>("selected"));

        //     widget.update_theme_by_state(false);
        // }

        // *self.selected_entities.borrow_mut() = ctx
        //     .widget()
        //     .get::<SelectedEntities>("selected_entities")
        //     .0
        //     .clone();
    }
}

// todo use code of list view item, by create combobox item insert entity of popup container

widget!(
    /// The `ComboBox` represents an selection widget with a drop-down list.
    ///
    /// **CSS element:** `ComboBox`
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

        /// Sets or shares the css selector property.
        selector: Selector,

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
        self.state_mut().builder = Some(Box::new(builder));
        self
    }
}

impl Template for ComboBox {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let container = Container::create()
            .selector(Selector::default().id(CONTAINER))
            .background(id)
            .border_radius(id)
            .border_width(id)
            .border_brush(id)
            .padding(id)
            .build(ctx);
        self.state_mut().selected_container = container;

        let items_panel = Stack::create()
            .vertical_alignment("start")
            .selector(Selector::default().id(ITEMS_PANEL))
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

        let _ = ctx.append_child_to_overlay(popup);

        self.name("ComboBox")
            .selector("combo_box")
            .height(32.0)
            .min_width(80.0)
            .selected(false)
            .selected_index(-1)
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .selector(id)
                    .child(
                        SelectionBehavior::create()
                            .selected(id)
                            .enabled(id)
                            .selector(id)
                            .parent(id.0)
                            .child(container)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
