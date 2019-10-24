use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

use super::behaviors::MouseBehavior;
use crate::{prelude::*, utils::SelectionMode as SelMode};

#[derive(Default)]
pub struct ListViewState {
    builder: RefCell<Option<Box<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
    count: Cell<usize>,
    selected_entities: RefCell<HashSet<Entity>>,
}

impl State for ListViewState {
    fn update(&self, context: &mut Context<'_>) {
        let count = context.widget().clone_or_default::<Count>("count").0;
        // self.selected_index.set(context.widget().clone_or_default::<Index>().0);

        if count != self.count.get() {
            if let Some(builder) = &*self.builder.borrow() {
                if let Some(items_panel) = context.entity_of_child("items_panel") {
                    context.clear_children_of(items_panel);

                    for i in 0..count {
                        let mut build_context = context.build_context();

                        let item = {
                            let child = builder(&mut build_context, i);
                            let item = ListViewItem::create().build(&mut build_context);

                            let mouse_behavior = MouseBehavior::create().build(&mut build_context);
                            build_context
                                .register_shared_property::<Selector>("selector", mouse_behavior, item);
                            build_context.register_shared_property::<Pressed>("pressed", mouse_behavior, item);
                            build_context.append_child(item, mouse_behavior);

                            build_context.register_shared_property::<Brush>("foreground", child, item);
                            build_context.register_shared_property::<FontSize>("font_size", child, item);
                            build_context.append_child(items_panel, item);
                            build_context.append_child(mouse_behavior, child);

                            item
                        };
                        context.get_widget(item).update_properties_by_theme();
                    }
                }
            }

            self.count.set(count);
        }
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        for index in context
            .widget()
            .get::<SelectedEntities>("selected_entities")
            .0
            .clone()
            .symmetric_difference(&*self.selected_entities.borrow())
        {
            let mut widget = context.get_widget(*index);
            widget.set("selected", Selected(!widget.get::<Selected>("selected").0));

            widget.update_theme_by_state(false);
        }

        *self.selected_entities.borrow_mut() = context.widget().get::<SelectedEntities>("selected_entities").0.clone();
    }
}

#[derive(Default)]
pub struct ListViewItemState {
    request_selection_toggle: Cell<bool>,
}

impl ListViewItemState {
    fn toggle_selection(&self) {
        self.request_selection_toggle.set(true);
    }
}

impl State for ListViewItemState {
    fn update(&self, context: &mut Context<'_>) {
        if !context.widget().get::<Enabled>("enabled").0 || !self.request_selection_toggle.get() {
            return;
        }
        self.request_selection_toggle.set(false);

        let selected = context.widget().get::<Selected>("selected").0;

        let entity = context.entity;
        let index = context.index_as_child(entity).unwrap();

        if let Some(parent) = &mut context.parent_by_id("ListView") {
            let selection_mode = parent.get::<SelectionMode>("selection_mode").0;
            // deselect item
            if selected {
                parent.get_mut::<SelectedEntities>("selected_entities").0.remove(&entity);
                parent.get_mut::<SelectedIndices>("selected_indices").0.remove(&index);
                return;
            }

            if parent.get::<SelectedEntities>("selected_entities").0.contains(&entity)
                || selection_mode == SelMode::None
            {
                return;
            }

            if selection_mode == SelMode::Single {
                parent.get_mut::<SelectedEntities>("selected_entities").0.clear();
                parent.get_mut::<SelectedIndices>("selected_indices").0.clear();
            }

            parent.get_mut::<SelectedEntities>("selected_entities").0.insert(entity);
            parent.get_mut::<SelectedIndices>("selected_indices").0.insert(index);
        }
    }
}

widget!(
    ListViewItem<ListViewItemState>: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_width: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the pressed property. 
        pressed: Pressed,

        /// Sets or shares the selected property. 
        selected: Selected
    }
);

impl Template for ListViewItem {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("ListViewItem")
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
            .on_click(move |_| {
                state.toggle_selection();
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

// todo: selection mode.

widget!(
    /// The `ListView` is an items drawer widget with selectable items.
    ///
    /// **CSS element:** `items-widget`
    ListView<ListViewState> {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_width: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Sets or shared the count.
        count: Count,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the selection mode property.
        selection_mode: SelectionMode,

        /// Sets or shares the selected indices.
        selected_indices: SelectedIndices,

        /// Sets or shares the list of selected indices.
        selected_entities: SelectedEntities,

        /// Sets or shares the (wheel, scroll) delta property. 
        delta: Delta
    }
);

impl ListView {
    pub fn items_builder<F: Fn(&mut BuildContext, usize) -> Entity + 'static>(
        self,
        builder: F,
    ) -> Self {
        *self.clone_state().builder.borrow_mut() = Some(Box::new(builder));
        self
    }
}

impl Template for ListView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let items_panel = Stack::create()
            .vertical_alignment("Start")
            .selector(SelectorValue::default().clone().id("items_panel"))
            .orientation(id)
            .build(context);

        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("Disabled", "Auto"))
            .delta(id)
            .child(items_panel)
            .build(context);

        self.name("ListView")
            .selector(SelectorValue::from("list-view").id("ListView"))
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_width(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .selection_mode("Single")
            .selected_indices(HashSet::new())
            .selected_entities(HashSet::new())
            .delta(0.0)
            .orientation("Vertical")
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
                    .padding(id)
                    .child(scroll_viewer)
                    .child(
                        ScrollIndicator::create()
                            .padding(2.0)
                            .content_id(ContentId::from(items_panel.0))
                            .scroll_offset(scroll_viewer)
                            .build(context),
                    )
                    .build(context),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
