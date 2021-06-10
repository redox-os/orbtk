use super::behaviors::MouseBehavior;

use crate::{api::prelude::*, prelude::*, proc_macros::*, themes::theme_orbtk::*};

// --- KEYS --
const TAB_HEADER_CONTAINER: &str = "tab_header_container";
const HEADER_CONTAINER: &str = "header_container";
const HEADER_BAR: &str = "header_bar";
const BODY_CONTAINER: &str = "body_container";
// --- KEYS --

/**
The `TabHeaderState` is used to store some callbacks that will be applied during template function.
Once the template function is called, they are no more used.
*/
#[derive(Default, AsAny)]
pub struct TabHeaderState {
    //Callback called when user click on the header of the tab (normally used to switch to the clicked tab). Only used during initialization
    on_header_mouse_down_callback: Option<Box<dyn 'static + Fn(&mut StatesContext, Mouse) -> bool>>,
    //Callback called when user click on the close button near the tab header (normally used to close the tab). Only used during initialization
    on_close_click_callback: Option<Box<dyn 'static + Fn(&mut StatesContext, Point) -> bool>>,

    header_bar: Entity,
    // tab_header_container: Entity,
}

impl State for TabHeaderState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.header_bar = ctx.child(HEADER_BAR).entity();
        // self.tab_header_container = ctx.child(TAB_HEADER_CONTAINER).entity();
        self.update(registry, ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        // set visibility of selection indicator bar
        // should be refactored after property converter are implemented
        // ```rust
        // Container::new().style(HEADER_BAR).visibility(("selected", id, |sel| {
        //    if sel Visibility::Visible else Visibility::Collapsed
        // }))
        // ```

        if *TabHeader::close_button_ref(&ctx.widget()) == Visibility::Collapsed {
            TabHeader::spacing_set(&mut ctx.widget(), 0.);
        }

        let selected = *ctx.widget().get::<bool>("selected");
        let vis = *ctx
            .get_widget(self.header_bar)
            .get::<Visibility>("visibility");
        if selected && vis != Visibility::Visible {
            ctx.get_widget(self.header_bar)
                .set("visibility", Visibility::Visible);
        } else if !selected && vis == Visibility::Visible {
            ctx.get_widget(self.header_bar)
                .set("visibility", Visibility::Collapsed);
        }
    }
}

widget!(
    /// The `TabHeader` widget is used internally to managed tabs headers. Not meant for other uses.
    TabHeader<TabHeaderState> {
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

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the text property.
        text: String,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the icon property.
        icon: String,

        /// Sets or shares the icon brush property.
        icon_brush: Brush,

        /// Sets or share the icon font size property.
        icon_size: f64,

        /// Sets or shares the icon font property.
        icon_font: String,

        /// Sets or shares the selected property.
        selected: bool,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the spacing between icon and text.
        spacing: f64,

        /// Sets or shares the close button visibility.
        close_button: Visibility,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool
    }
);

impl TabHeader {
    ///Set the callback that is called when user click on the header (generally used to switch tab)
    pub fn on_header_mouse_down<T: 'static + Fn(&mut StatesContext, Mouse) -> bool>(
        mut self,
        callback: T,
    ) -> Self {
        self.state.on_header_mouse_down_callback = Some(Box::new(callback));
        self
    }

    ///Set the callback that is called when user click on the close button near the header (generally used to close the tab)
    pub fn on_close_click<T: 'static + Fn(&mut StatesContext, Point) -> bool>(
        mut self,
        callback: T,
    ) -> Self {
        self.state.on_close_click_callback = Some(Box::new(callback));
        self
    }
}

impl Template for TabHeader {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let mut button = Button::new()
            .style("tab_icon_only")
            .icon(material_icons_font::MD_CLOSE)
            .visibility(("close_button", id));

        if let Some(callback) = self.state.on_close_click_callback.take() {
            button = button.on_click(callback);
        }
        //if self.close_button() == false {button = button.visibility(Visibility::Collapsed);}

        let mut mouse_behavior = MouseBehavior::new().enabled(id).target(id.0).pressed(id);

        if let Some(callback) = self.state.on_header_mouse_down_callback.take() {
            mouse_behavior = mouse_behavior.on_mouse_down(callback);
        }

        self.name("TabHeader")
            .style("tab_header")
            .selected(false)
            .min_width(64)
            .height(36)
            .background(colors::LYNCH_COLOR)
            .border_radius(4)
            .border_width(0)
            .border_brush("transparent")
            .padding((16, 0, 16, 0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("Unnamed Tab")
            .font_size(orbtk_fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .icon("")
            .icon_font("MaterialIcons-Regular")
            .icon_size(orbtk_fonts::ICON_FONT_SIZE_12)
            .icon_brush(colors::LINK_WATER_COLOR)
            .spacing(4)
            .close_button(Visibility::Visible)
            .child(mouse_behavior.build(ctx))
            .child(
                Container::new()
                    .padding(id)
                    .child(
                        Stack::new()
                            .id(TAB_HEADER_CONTAINER)
                            .spacing(id)
                            .orientation("horizontal")
                            .child(
                                TextBlock::new()
                                    .text(id)
                                    .v_align("center")
                                    .h_align("start")
                                    .font(id)
                                    .font_size(id)
                                    .foreground(id)
                                    .build(ctx),
                            )
                            .child(button.v_align("center").build(ctx))
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .child(
                Container::new()
                    .id(HEADER_BAR)
                    .v_align("start")
                    .visibility("collapsed")
                    .style("tab_header_bar")
                    .build(ctx),
            )
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    // fn layout(&self) -> Box<dyn Layout> {
    //     PaddingLayout::new().into()
    // }
}

///Used to internally
enum TabWidgetAction {
    SelectByIndex(usize),
    SelectByBody(Entity),
    Add(String, Entity),
    Remove(Entity),
    SetCloseButtonVisibility(bool),
}
/**
Through the TabWidgetState it is possible to control the behaviour of TabWidget.
Nearly every called function on TabWidgetState will not be executed immediatly, but they are instead stored and executed during the update phase,
in the same order they are submitted.
*/
#[derive(Default, AsAny)]
pub struct TabWidgetState {
    ///Store all the pending actions. During the update call they will be resolved
    actions: Vec<TabWidgetAction>,

    header_container: Entity,
    body_container: Entity,

    tabs: Vec<(Entity, Entity)>, //Header , Body
    selected: usize,

    close_button_visibility: bool,
}

impl TabWidgetState {
    /**
    Switch the selected tab with one with the passed index.
    If the index is greater than the tab count, the last one will be selected.
    */
    pub fn select_by_index(&mut self, index: usize) {
        self.actions.push(TabWidgetAction::SelectByIndex(index));
    }
    /**
    Switch the selected tab with one with the passed body.
    If the no such body is found among the present tabs, nothing happen.
    */
    pub fn select_by_body(&mut self, entity: Entity) {
        self.actions.push(TabWidgetAction::SelectByBody(entity));
    }

    /**
    Remove the tab with the passed body.
    If the no such body is found among the present tabs, nothing happen.
    */
    pub fn remove_by_body(&mut self, entity: Entity) {
        self.actions.push(TabWidgetAction::Remove(entity));
    }

    ///Add a new tab to the widget.
    pub fn add_tab<T: Into<String>>(&mut self, header: T, body: Entity) {
        self.actions.push(TabWidgetAction::Add(header.into(), body));

        //At this point the tab has not been added yet, so we can check if len is == 0
        if self.tabs.is_empty() {
            self.select_by_index(0);
        }
    }

    ///Set the close button visibility of all the tabs.
    pub fn set_close_button_visibility(&mut self, value: bool) {
        self.actions
            .push(TabWidgetAction::SetCloseButtonVisibility(value));
    }
    ///Get the close button visibility status
    pub fn get_close_button_visibility(&self) -> bool {
        self.close_button_visibility
    }

    /**
    Get the tab index associated with the passed body.
    If it is not found, None is returned.
    */
    pub fn get_index(&self, tab_body: Entity) -> Option<usize> {
        for i in 0..self.tabs.len() {
            let (_, body) = self.tabs[i];
            if body == tab_body {
                return Some(i);
            }
        }

        None
    }

    /**
    Visually refresh the current tab. This is used when, for example, the selected tab is removed,
    so the tab that take it's place need to be visually updated.
    */
    fn refresh_selected_tab(&mut self, ctx: &mut Context) {
        let tab = self.tabs[self.selected];
        ctx.get_widget(tab.0).set("selected", true);
        toggle_flag("selected", &mut ctx.get_widget(tab.0));
        ctx.get_widget(tab.0).update(false);
        ctx.get_widget(tab.1).set("visibility", Visibility::Visible);
    }

    /**
    Change the selected tab by index. Unlike the public "select_by_index", this happen immediately.
    */
    fn select_by_index_internal(&mut self, ctx: &mut Context, mut index: usize) {
        //No tabs could be selected if there are no one, so return immediately
        if self.tabs.is_empty() {
            return;
        }

        //If the passed index is greater than tab count, select the last one
        if index >= self.tabs.len() && index != 0 {
            index = self.tabs.len() - 1;
        }

        if self.selected != index {
            let current_tab = self.tabs[self.selected];
            let new_tab = self.tabs[index];

            //Toggle current button, the new button is toggled by user click
            ctx.get_widget(current_tab.0).set("selected", false);
            toggle_flag("selected", &mut ctx.get_widget(current_tab.0));
            ctx.get_widget(current_tab.0).update(true);

            //Hide current body
            ctx.get_widget(current_tab.1)
                .set("visibility", Visibility::Collapsed);

            ctx.get_widget(new_tab.0).set("selected", true);
            toggle_flag("selected", &mut ctx.get_widget(new_tab.0));
            ctx.get_widget(new_tab.0).update(false);

            //Show new body
            ctx.get_widget(new_tab.1)
                .set("visibility", Visibility::Visible);

            self.selected = index;
        }
    }

    ///Add a new tab to the widget. Unlike the public "add_tab", this happen immediatly.
    fn add_tab_internal(&mut self, ctx: &mut Context, header_text: String, body: Entity) {
        //Create the new tab
        let header = self.create_tab_header(ctx, header_text, body);

        //Set tab body collapsed
        ctx.get_widget(body)
            .set("visibility", Visibility::Collapsed);

        //Push button to the header container
        ctx.append_child_entity_to(header, self.header_container);

        //Push the body to the body container
        ctx.append_child_entity_to(body, self.body_container);

        //Push the new tab to the list
        self.tabs.push((header, body));

        //If the added tab is the first
        if self.tabs.len() == 1 {
            //Select the tab just inserted
            self.selected = 0;
            self.refresh_selected_tab(ctx);
        }
    }

    ///Remove a tab from the widget. Unlike the public "remove_tab", this happen immediatly.
    fn remove_tab_internal(&mut self, ctx: &mut Context, body: Entity) {
        if let Some(index) = self.get_index(body) {
            //Pop the index tab out of the stored tabs
            let (header, body) = self.tabs.remove(index);

            //Push button to the header container
            ctx.remove_child_from(header, self.header_container);

            //Push the body to the body container
            ctx.remove_child_from(body, self.body_container);

            //If there is at least one tab
            if !self.tabs.is_empty() {
                //If selected is greater than tab count, select the last one
                if self.selected >= self.tabs.len() {
                    self.selected = self.tabs.len() - 1;
                }

                //Only update current selection if the removed tab is lesser than the selected.
                //If it is greater, there is no need to update, but simply remove the target tab
                if index <= self.selected {
                    self.refresh_selected_tab(ctx);
                }
            }
        }
    }

    /**
    Set the visibility of the close button on all tabs. Unlike the public "set_close_button_visibility", this happen immediatly.
    If the passed "value" is equal to "self.close_button_visibility", so the requested visibility is already present, nothing happen.
    */
    fn set_close_button_visibility_internal(&mut self, ctx: &mut Context, value: bool) {
        if self.close_button_visibility != value {
            self.close_button_visibility = value;
            let new_visibility = if self.close_button_visibility {
                Visibility::Visible
            } else {
                Visibility::Collapsed
            };
            for tab in &self.tabs {
                ctx.get_widget(tab.0).set("close_button", new_visibility);
            }
        }
    }

    //Create a new TabHeader entity and return it. For internal use.
    fn create_tab_header(&self, ctx: &mut Context, text: String, body: Entity) -> Entity {
        let cloned_entity = ctx.entity();
        TabHeader::new()
            .close_button(if self.close_button_visibility {
                Visibility::Visible
            } else {
                Visibility::Collapsed
            })
            .text(text)
            .on_header_mouse_down(move |states, _| {
                states
                    .get_mut::<TabWidgetState>(cloned_entity)
                    .select_by_body(body);
                true
            })
            .on_close_click(move |states, _| {
                states
                    .get_mut::<TabWidgetState>(cloned_entity)
                    .remove_by_body(body);
                true
            })
            .build(&mut ctx.build_context())
    }
}

impl State for TabWidgetState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.header_container = ctx.child(HEADER_CONTAINER).entity();
        self.body_container = ctx.child(BODY_CONTAINER).entity();
        self.close_button_visibility = true;
        self.update(registry, ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let actions: Vec<TabWidgetAction> = self.actions.drain(..).collect();
        for action in actions {
            match action {
                TabWidgetAction::SelectByIndex(index) => {
                    self.select_by_index_internal(ctx, index);
                }
                TabWidgetAction::SelectByBody(body) => {
                    if let Some(index) = self.get_index(body) {
                        self.select_by_index_internal(ctx, index)
                    }
                }
                TabWidgetAction::Add(header_text, body) => {
                    self.add_tab_internal(ctx, header_text, body);
                }
                TabWidgetAction::Remove(body) => {
                    self.remove_tab_internal(ctx, body);
                }
                TabWidgetAction::SetCloseButtonVisibility(value) => {
                    self.set_close_button_visibility_internal(ctx, value);
                }
            }
        }
    }
}

widget!(
    /**
    The `TabWidget` widget can store and control multiple tabs with arbitrary content. Only the selected tab will show it's content.

    This example creates a TabWidget:
    ```rust
    TabWidget::new()
    .tab("Tab header 1",TextBlock::new().text("Tab content 1").build(ctx))
    .tab("Tab header 2",TextBlock::new().text("Tab content 2").build(ctx))
    .tab("Tab header 3",TextBlock::new().text("Tab content 3").build(ctx))
    .build(ctx)
     ```
     */
    TabWidget<TabWidgetState> {
        /// Sets or shares the spacing between tabs.
        spacing: f64,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness
    }
);

impl TabWidget {
    ///Set the close button visibility
    pub fn close_button(mut self, value: bool) -> Self {
        self.state
            .actions
            .push(TabWidgetAction::SetCloseButtonVisibility(value));
        self
    }
    ///Add a tab the widget
    pub fn tab<T: Into<String>>(mut self, header: T, body: Entity) -> Self {
        self.state
            .actions
            .push(TabWidgetAction::Add(header.into(), body));
        self
    }
}

impl Template for TabWidget {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TabWidget").style("tab_widget").child(
            Grid::new()
                .rows("34, *")
                .child(
                    Stack::new()
                        .id(HEADER_CONTAINER)
                        .orientation("horizontal")
                        .spacing(id)
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .id(BODY_CONTAINER)
                        .background(id)
                        .border_brush(id)
                        .border_width(id)
                        .border_radius(id)
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
