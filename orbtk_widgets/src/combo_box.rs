use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use super::behaviors::{MouseBehavior, SelectionBehavior};

use crate::{
    api::prelude::*, prelude::*, proc_macros::*, shell::prelude::*, themes::theme_orbtk::*,
};

type SelectedItem = Option<Entity>;

// /// The `ComboBoxItemAction` represent actions that can be sent to `ComboBoxItemState`.
// #[derive(Debug, Copy, Clone)]
// enum ComboBoxItemAction {
//     // react on MouseUp event, if triggered outside the ComboBox bounds
//     _CheckMouseUpOutside {position: Point},

//     // toggle the selection popup
//     _ToggleSelection {position: Point},
// }

/// The `ComboBoxItemState` handles the interaction and selection of a `ComboBoxItem`.
#[derive(AsAny, Default)]
pub struct ComboBoxItemState {
    //action: Option<ComboBoxItemAction>,
    request_selection_toggle: Cell<bool>,
    index: usize,
    combobox_selector: Entity,
    combo_box: Entity,
    // ugly work around for item builder context clone.
    // TODO: improve the used algo 😉
    builder: Option<Arc<RefCell<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
}

/// The `ComboBoxItemState` handles the selection of a `ComboBoxItem`
/// as well as the associated interation methods.
impl ComboBoxItemState {
    fn toggle_selection(&self) {
	self.request_selection_toggle.set(true);
    }
}

impl State for ComboBoxItemState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
	let selected_index = ctx
	    .get_widget(self.combo_box)
	    .clone_or_default::<i32>("selected_index");
	let selected: bool = *ctx.widget().get("selected");

	// check if status inside ComboBoxItems popup has changed
	if selected_index >= 0 && (selected_index as usize) == self.index && !selected {
	    self.request_selection_toggle.set(true);
	}

	// omit rendering, if already updated
	if !ctx.widget().get::<bool>("enabled") || !self.request_selection_toggle.get() {
	    return;
	}
	self.request_selection_toggle.set(false);

	let entity = ctx.entity();

	// update previous selected item status
	if let Some(item) = ctx
	    .get_widget(self.combo_box)
	    .clone::<SelectedItem>("selected_item")
	{
	    ctx.get_widget(item).set("selected", false);
	    ctx.get_widget(item)
		.get_mut::<Selector>("selector")
		.remove_state("selected");
	    ctx.get_widget(item).update(false);
	}

	// update new selected item status.
	ctx.widget().set("selected", true);
	ctx.widget()
	    .get_mut::<Selector>("selector")
	    .push_state("selected");
	ctx.widget().update(false);
	ctx.get_widget(self.combo_box)
	    .set("selected_index", self.index as i32);
	ctx.get_widget(self.combo_box)
	    .set("selected_item", Some(entity));

	// add selected content to combobox
	let index = self.index;
	let combobox_selector = self.combobox_selector;
	if let Some(builder) = &self.builder {
	    ctx.clear_children_of(combobox_selector);
	    let build_context = &mut ctx.build_context();
	    let selected_content = builder.borrow()(build_context, index);
	    build_context.register_shared_property::<Brush>(
		"foreground",
		selected_content,
		self.combo_box,
	    );
	    build_context.register_shared_property::<f32>(
		"opacity",
		selected_content,
		self.combo_box,
	    );
	    build_context.register_shared_property::<f64>(
		"font_size",
		selected_content,
		self.combo_box,
	    );
	    build_context.append_child(combobox_selector, selected_content);
	}
    }
}

widget!(
    /// The `ComboBoxItem` describes an item inside of a `ComboBox` popup.
    ///
    /// **style:** `combo_box_item``
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

	/// Indicates if the widget is hovered by the mouse cursor.
	hover: bool,

	/// Sets or shares the padding property.
	padding: Thickness,

	/// Sets or shares the pressed property.
	///
	/// The boolean indicates, if an item inside
	/// the item list was pressed.
	pressed: bool,

	/// Sets or shares the selected property.
	///
	/// The boolean indicates that the status
	/// of the selected item has changed.
	selected: bool
    }
);

/// Method definitions, that react on any given state change inside
/// the `ComboBoxItem` widget.
impl ComboBoxItem {
    fn combobox_selector(mut self, combobox_selector: impl Into<Entity>) -> Self {
	self.state_mut().combobox_selector = combobox_selector.into();
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
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
	self.name("ComboBoxItem")
	    .style("combo_box_item")
	    .background("transparent")
	    .border_brush("transparent")
	    .border_radius(0)
	    .border_width(0)
	    .foreground(colors::LINK_WATER_COLOR)
	    .font_size(32)
	    .font("Roboto-Regular")
	    .min_width(64)
	    .padding(0)
	    .pressed(false)
	    .selected(false)
	    .child(
		MouseBehavior::new()
		    .pressed(id)
		    .enabled(id)
		    .target(id.0)
		    .build(ctx),
	    )
	    .on_click(move |states, _| {
		states.get::<ComboBoxItemState>(id).toggle_selection();
		false
	    })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
	RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
	PaddingLayout::new().into()
    }
}

/// The `ComboBoxAction` represent actions that can be sent to `ComboBoxState`.
#[derive(Clone, Debug)]
enum ComboBoxAction {
    // close the selection popup, if MouseUp event is triggered
    // outside the ComboBox bounds
    CloseSelection { position: Point },

    // open the selection popup, if MouseDown event is triggered
    // inside the ComboBox bounds
    OpenSelection { position: Point },

    // handle selection of popup, if Keyboard event is triggered
    KeyPressed { key_event: KeyEvent },
}

/// The `ComboBoxState` is used to manipulate the thumb position
/// inside the popup widget. When user triggers the open event, the
/// ComboBox widget is annotated with a slider.
///
/// Note: The slider is rendered visible, if the number of selectable
/// items exceed the available size of the popup widget bounds.
#[derive(AsAny, Default)]
pub struct ComboBoxState {
    action: Option<ComboBoxAction>,
    builder: Option<Arc<RefCell<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>>,
    count: usize,
    items_popup: Entity,
    popup: Entity,
    combobox_selector: Entity,
}

/// Method definitions, that react on any given state change inside
/// the `ComboBox` widget.
impl ComboBoxState {
    // closes the popup if `mouse up` is raised outside of the
    // Combobox related bounds.
    fn close_selection(&mut self, ctx: &mut Context, p: Point) {
	let combo_box_position = ctx.widget().clone::<Point>("position");
	let combo_box_bounds = ctx.widget().clone::<Rectangle>("bounds");

	let combo_box_global_bounds = Rectangle::new(combo_box_position, combo_box_bounds.size());

	if !combo_box_global_bounds.contains(p) {
	    ctx.widget().set("selected", false);
	    ctx.widget()
		.get_mut::<Selector>("selector")
		.remove_state("selected");
	    ctx.get_widget(self.popup)
		.set("visibility", Visibility::Collapsed);
	    ctx.get_widget(self.popup).update(false);
	    ctx.widget().update(false);
	}
    }

    // opens a popup item box.
    fn open_selection(&mut self, ctx: &mut Context, p: Point) {
	// upper left point of our ComboBox selector
	let combo_box_position = ctx.widget().clone::<Point>("position");
	let combo_box_bounds = ctx.widget().clone::<Rectangle>("bounds");
        let combo_box_global_bounds = Rectangle::new(combo_box_position, combo_box_bounds.size());

	//let combo_box_placement = ctx.widget().clone::<Placement>("placement");

	// open the popup if `mouse down` point is inside of the ComboBox bounds.
	if combo_box_global_bounds.contains(p) {
	    ctx.widget().set("selected", true);
	    ctx.widget()
		.get_mut::<Selector>("selector")
		.push_state("selected");
	    ctx.get_widget(self.popup)
		.set("visibility", Visibility::Visible);
	    ctx.get_widget(self.popup).update(true);
	    ctx.widget().update(true);
	}
    }
}

impl State for ComboBoxState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
	// TODO: set it in a dynamic fashion
	// e.g consume the value afer measure was run
	let combo_box_width = 100.0;
	let combo_box_item_height = 100.0;

	ctx.widget()
	    .get_mut::<Constraint>("constraint")
	   .set_width(combo_box_width);

	ctx.get_widget(self.popup).set("open", false);
	ctx.get_widget(self.popup)
	    .set("visibility", Visibility::Collapsed);
	ctx.get_widget(self.popup)
	    .set("placement", Placement::Bottom);
	ctx.get_widget(self.popup)
	    .get_mut::<Constraint>("constraint")
	    .set_width(combo_box_width);
	ctx.get_widget(self.popup)
	    .get_mut::<Constraint>("constraint")
	    .set_height(combo_box_item_height);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
	let count = ctx.widget().clone_or_default::<usize>("count");
	let entity = ctx.entity();

	// build the ComboBoxItems
	if count != self.count {
	    if let Some(builder) = &self.builder {
		ctx.clear_children_of(self.items_popup);

		for i in 0..count {
		    let item = {
			let build_context = &mut ctx.build_context();
			let child = builder.borrow()(build_context, i);
			let mut item = ComboBoxItem::new()
			    .index(i)
			    .combo_box(entity)
			    .combobox_selector(self.combobox_selector);

			if let Some(builder) = &self.builder {
			    item.items_builder(builder);
			}
			let item = item.build(build_context);

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
			build_context.append_child(self.items_popup, item);
			build_context.append_child(mouse_behavior, child);

			item
		    };
		    ctx.get_widget(item).update_widget(entity, false, false);
		}
	    }

	    self.count = count;
	}
    }

    fn messages(
	&mut self,
	mut messages: MessageReader,
	_registry: &mut Registry,
	ctx: &mut Context,
    ) {
	for message in messages.read::<ComboBoxAction>() {
	    match message {
		ComboBoxAction::CloseSelection { position } => {
		    ComboBoxState::close_selection(self, ctx, position);
		}
		ComboBoxAction::OpenSelection { position } => {
		    ComboBoxState::open_selection(self, ctx, position);
		}
		ComboBoxAction::KeyPressed { key_event } => match key_event.key {
		    Key::Escape => {
			// TODO: get active position
			//ComboBoxState::close_selection(self, ctx, position);
			println!("combo_box: Key={:?}", key_event.key);
		    }
		    Key::Down | Key::NumpadAdd => {
			// TODO: get active position
			//ComboBoxState::open_selection(self, ctx, position);
			println!("combo_box: Key={:?}", key_event.key);
		    }
		    _ => {}
		},
	    }
	}
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
	// omit rendering, if there is no selection
	if self.action.is_none() || !(*ctx.widget().get::<bool>("selected")) {
	    return;
	}

	if let Some(ComboBoxAction::CloseSelection { position }) =  self.action {
	    ComboBoxState::close_selection(self, ctx, position);
	}
    }

    fn cleanup(&mut self, _: &mut Registry, ctx: &mut Context) {
	let _ = ctx.remove_child_from_overlay(self.popup);
    }
}

// TODO: use code of list view item, by creating a combobox item insert entity of popup container

widget!(
    /// The `ComboBox` represents a selection widget with a drop-down box.
    ///
    /// The selection box itself presents the active selected item.
    /// You may activate the drop-down popup by activating a handler
    /// (Keyboard, Mouse). Select a new item from the presented item
    /// list. Once the active selection changes, the index pointing
    /// to the selected item is updated and the item list is
    /// collapsed. The drop-down box is annotated with a slider.
    /// Note: The slider is rendered visible, if the number of selectable
    /// items exceed the available size of the popup widget bounds.
    ///
    /// **style:** `combo_box`
    ComboBox<ComboBoxState>: KeyDownHandler, MouseHandler {
	/// Sets or shares the background property.
	background: Brush,

	/// Sets or shares the border radius property.
	border_radius: f64,

	/// Sets or shares the border thickness property.
	border_width: Thickness,

	/// Sets or shares the border brush property.
	border_brush: Brush,

	/// Sets or shared the count.
	///
	/// Holds the number of the selectable items.
	count: usize,

	/// Sets or shares the foreground property.
	foreground: Brush,

	/// Sets or share the font size property.
	font_size: f64,

	/// Sets or shares the font property.
	font: String,

	/// Indicates if the widget is hovered by the mouse cursor.
	hover: bool,

	/// Sets or shares the icon property.
	icon: String,

	/// Sets or shares the icon brush property.
	icon_brush: Brush,

	/// Sets or share the icon font size property.
	icon_size: f64,

	/// Sets or shares the icon font property.
	icon_font: String,

	/// Sets or shares the padding property.
	padding: Thickness,

	/// Sets or shares the placement property.
	///
	/// The drop down popup offers selectable items and is placed
	/// relative to the position of the selection box. An optional
	/// distance attribute (float), defines the margin between the
	/// selection box and the drop down popup.
	///
	/// [`placement`]: ../orbtk_core/render_object/enum.Placement.html
	placement: Placement,

	/// Sets or shares the pressed property.
	///
	/// The boolean indicates, if the selection popup
	/// should be toggled.
	pressed: bool,

	/// Sets or shares the selected property.
	///
	/// The boolean indicates that the status
	/// inside the drop down popup has changed.
	selected: bool,

	/// Sets or shares the selected index.
	/// Value: "-1" -> no item is selected.
	selected_index: i32,

	/// The entity of the selected item.
	selected_item: SelectedItem
    }
);

impl ComboBox {
    /// Define the build function used to create the content of the ComboBoxItems.
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
	let combobox_selector = Container::new().attach(Grid::column(0)).build(ctx);

	let container = Container::new()
	    .background(id)
	    .border_radius(id)
	    .border_width(id)
	    .border_brush(id)
	    .padding(id)
	    .child(
		Grid::new()
		    .columns("*, 4, 14")
		    .child(combobox_selector)
		    .child(
			FontIconBlock::new()
			    .attach(Grid::column(2))
			    .v_align("center")
			    .icon_brush(id)
			    .icon_size(id)
			    .icon_font(id)
			    .icon(id)
			    .build(ctx),
		    )
		    .build(ctx),
	    )
	    .build(ctx);

	self.state_mut().combobox_selector = combobox_selector;

	let items_popup = Stack::new()
	    .v_align("start")
	    .orientation("vertical")
	    .build(ctx);

	self.state_mut().items_popup = items_popup;
	let scroll_viewer = ScrollViewer::new()
	    .mode(("disabled", "auto"))
	    .child(items_popup)
	    .build(ctx);

	let popup = Popup::new()
	    .min_height(40.0)
	    .child(scroll_viewer)
	    .child(
		ScrollIndicator::new()
		    .padding(2.0)
		    .content_bounds(("bounds", items_popup))
		    .view_port_bounds(("bounds", scroll_viewer))
		    .scroll_padding(("padding", scroll_viewer))
		    .mode(scroll_viewer)
		    .opacity(id)
		    .build(ctx),
	    )
	    .target(container)
	    .build(ctx);
	self.state_mut().popup = popup;

	let _ = ctx.append_child_to_overlay(popup);

	self.name("ComboBox")
	    .style("combo_box")
	    .icon(material_icons_font::MD_ARROW_DROP_DOWN)
	    .icon_font("MaterialIcons-Regular")
	    .icon_size(orbtk_fonts::ICON_FONT_SIZE_12)
	    .icon_brush(colors::LINK_WATER_COLOR)
	    .height(32.0)
	    .min_width(40.0)
	    .selected(false)
	    .selected_index(-1)
	    .child(
		MouseBehavior::new()
		    .pressed(id)
		    .enabled(id)
		    .target(id.0)
		    .child(
			SelectionBehavior::new()
			    .selected(id)
			    .enabled(id)
			    .target(id.0)
			    .child(container)
			    .build(ctx),
		    )
		    .build(ctx),
	    )
	    .on_global_mouse_up(move |states, event| {
		let position: Point = event.position;
		states.send_message(ComboBoxAction::CloseSelection { position }, id);
	    })
	    .on_key_down(move |states, key_event: KeyEvent| -> bool {
		let key_event: KeyEvent = key_event;
		states.send_message(ComboBoxAction::KeyPressed { key_event }, id);
		true
	    })
	    .on_mouse_down(move |states, event| {
		let position: Point = event.position;
		states.send_message(ComboBoxAction::OpenSelection { position }, id);
		true
	    })
    }
}
