use std::cell::Cell;

use crate::prelude::*;

#[derive(Default)]
pub struct ListViewState {
    builder: RefCell<Option<Box<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
    count: Cell<usize>,
    selected_index: Cell<i32>,
}

impl State for ListViewState {
    fn update(&self, context: &mut Context<'_>) {
        let count = context.widget().clone_or_default::<Count>().0;
        self.selected_index.set(context.widget().clone_or_default::<Index>().0);

        if count != self.count.get() {
            if let Some(builder) = &*self.builder.borrow() {
                if let Some(items_panel) = context.entity_of_child("items_panel") {
                    context.clear_children_of(items_panel);
                    let mut build_context = context.build_context();

                    for i in 0..count {
                        let child = builder(&mut build_context, i);
                        let item = ListViewItem::create().build(&mut build_context);
                        build_context.register_shared_property::<Foreground>(child, item);
                        build_context.register_shared_property::<FontSize>(child, item);
                        build_context.register_property(item, Index(i as i32));
                        build_context.append_child(items_panel, item);
                        build_context.append_child(item, child);
                    }
                }
            }

            self.count.set(count);
        }
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        let selected_index = context.widget().clone_or_default::<Index>().0;

        if selected_index < 0 {
            return;
        }

        if selected_index != self.selected_index.get() {
             if let Some(items_panel) = context.entity_of_child("items_panel") {
                 if let Some(old_selected_item) = &mut context.child_of_parent(items_panel, selected_index as usize) {
                     old_selected_item.set(Selected(false));
                 }
             }
        }
    }
}

#[derive(Default)]
pub struct ListViewItemState {}

impl State for ListViewItemState {
    fn update(&self, context: &mut Context<'_>) {
        self.update_pressed(&mut context.widget());

        if !context.widget().get::<Pressed>().0 {
            return;
        }

        context.widget().set(Selected(true));

        let index = context.widget().clone::<Index>();

        if let Some(parent) = &mut context.parent_by_id("ListView") {

            let blub = parent.has::<Index>();
            if *parent.get::<Index>() == index {
                return;
            }

            parent.set(index);
        }
    }

     fn update_post_layout(&self, context: &mut Context<'_>) {
         self.update_selected(&mut context.widget());
     }
}

impl PressedState for ListViewItemState {}
impl SelectedState for ListViewItemState {}

widget!(
    ListViewItem<ListViewItemState>: ClickHandler {
        // Sets or shares the index property.
        index: Index,

        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the foreground property.
        foreground: Foreground,

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
        self.name("ListViewItem")
            .index(0)
            .height(24.0)
            .selected(false)
            .pressed(false)
            .selector("list-view-item")
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_thickness(0.0)
            .border_brush("transparent")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(32.0)
            .font("Roboto Regular")
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(RectangleRenderObject))
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
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Sets or shared the items_count.
        items_count: Count,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the index of the selected item.
        selected_index: Index
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
        self.name("ListView")
            .selector(SelectorValue::from("list-view").id("ListView"))
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_thickness(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .selected_index(-1)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Stack::create()
                            .selector(SelectorValue::default().clone().id("items_panel"))
                            .orientation(id)
                            .build(context),
                    )
                    .build(context),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
