use crate::prelude::*;

// --- KEYS --
const HEADER_CONTAINER: &str = "header_container";
const BODY_CONTAINER: &str = "body_container";
// --- KEYS --

/**
The `TabHeaderState` is used to store some callbacks that will be applied during template function.
Once the template function is called, they are no more used.
*/
#[derive(Default, AsAny)]
pub struct TabHeaderState {
    //Callback called when user click on the header of the tab (normally used to switch to the clicked tab). Only used during initialization
    on_header_click_callback: Option<Box<dyn 'static + Fn(&mut StatesContext, Point) -> bool>>,
    //Callback called when user click on the close button near the tab header (normally used to close the tab). Only used during initialization
    on_close_click_callback: Option<Box<dyn 'static + Fn(&mut StatesContext, Point) -> bool>>,
}

impl State for TabHeaderState {}

widget!(
    /// The `TabHeader` widget is used internally to managed tabs headers. Not meant for other uses.
    TabHeader<TabHeaderState>: ChangedHandler
    {
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
        text: String16,

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

        /// Sets or shares the spacing between icon and text.
        spacing: f64,

        /// Sets or shares the close button visibility.
        close_button: Visibility
    }
);

impl TabHeader {
    ///Set the callback that is called when user click on the header (generally used to switch tab)
    pub fn on_header_click<T: 'static + Fn(&mut StatesContext, Point) -> bool>(
        mut self,
        callback: T,
    ) -> Self {
        self.state.on_header_click_callback = Some(Box::new(callback));
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
        let mut toggle_button = ToggleButton::new()
            .background(id)
            .border_radius(id)
            .border_width(id)
            .border_brush(id)
            .padding(id)
            .foreground(id)
            .text(id)
            .font_size(id)
            .font(id)
            .icon(id)
            .icon_brush(id)
            .icon_size(id)
            .icon_font(id)
            .selected(id)
            .spacing(id);

        if let Some(callback) = self.state.on_header_click_callback.take() {
            toggle_button = toggle_button.on_click(callback);
        }

        let mut button = Button::new()
            .background(id)
            .border_radius(id)
            .border_width(id)
            .border_brush(id)
            .padding(id)
            .foreground(id)
            .font_size(id)
            .font(id)
            .icon(id)
            .icon_brush(id)
            .icon_size(id)
            .icon_font(id)
            .spacing(id)
            .text("X")
            .width(2)
            .visibility(("close_button", id));

        if let Some(callback) = self.state.on_close_click_callback.take() {
            button = button.on_click(callback);
        }
        //if self.close_button() == false {button = button.visibility(Visibility::Collapsed);}

        self.name("TabWidget")
            .selected(false)
            .height(36)
            .min_width(64)
            .background(colors::LYNCH_COLOR)
            .border_radius(4)
            .border_width(0)
            .border_brush("transparent")
            .padding((16, 0, 16, 0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("Unnamed Tab")
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .icon("")
            .icon_font("Material Icons")
            .icon_size(fonts::ICON_FONT_SIZE_12)
            .icon_brush(colors::LINK_WATER_COLOR)
            .spacing(8)
            .close_button(Visibility::Visible)
            .child(
                Stack::new()
                    .orientation("horizontal")
                    .child(toggle_button.build(ctx))
                    .child(button.build(ctx))
                    .build(ctx),
            )
    }
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
        if self.tabs.len() == 0 {
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
        return None;
    }

    /**
    Visually refresh the current tab. This is used when, for example, the selected tab is removed,
    so the tab that take it's place need to be visually updated.
    */
    fn refresh_selected_tab(&mut self, ctx: &mut Context) {
        let tab = self.tabs[self.selected];
        ctx.get_widget(tab.0).set("selected", true);
        ctx.get_widget(tab.1).set("visibility", Visibility::Visible);
    }

    /**
    Change the selected tab by index. Unlike the public "select_by_index", this happen immediatly.
    */
    fn select_by_index_internal(&mut self, ctx: &mut Context, mut index: usize) {
        //No tabs could be selected if there are no one, so return immediatly
        if self.tabs.len() == 0 {
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

            //Hide current body
            ctx.get_widget(current_tab.1)
                .set("visibility", Visibility::Hidden);

            //The new tab is not "selected" because it is already done by the user click

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

        //Set tab body hidden
        ctx.get_widget(body).set("visibility", Visibility::Hidden);

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
        match self.get_index(body) {
            Some(index) => {
                //Pop the index tab out of the stored tabs
                let (header, body) = self.tabs.remove(index);

                //Push button to the header container
                ctx.remove_child_from(header, self.header_container);

                //Push the body to the body container
                ctx.remove_child_from(body, self.body_container);

                //If there is at least one tab
                if self.tabs.len() != 0 {
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
            None => (),
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
        let cloned_entity = ctx.entity;
        TabHeader::new()
            .close_button(if self.close_button_visibility {
                Visibility::Visible
            } else {
                Visibility::Collapsed
            })
            .text(String16::from(text))
            .on_header_click(move |states, _| {
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
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.header_container = ctx.child(HEADER_CONTAINER).entity();
        self.body_container = ctx.child(BODY_CONTAINER).entity();
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
    TabWidget<TabWidgetState>: ChangedHandler
    {
        /// Sets or shares the spacing between tabs.
        spacing: f64
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
        self.name("TabWidget")
        .close_button(true)
        .child(
            Grid::new()
                .rows(Rows::new().add(32).add("*"))
                .child(
                    Stack::new()
                        .id(HEADER_CONTAINER)
                        .orientation("horizontal")
                        .spacing(id)
                        .build(ctx),
                )
                .child(
                    Grid::new()
                        .attach(Grid::row(1))
                        .id(BODY_CONTAINER)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
