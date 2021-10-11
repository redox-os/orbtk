use orbtk::{
    prelude::*,
    // only instantiate the `default` theme
    widgets::themes::theme_orbtk::{
        register_default_fonts, THEME_DEFAULT, THEME_DEFAULT_COLORS_DARK, THEME_DEFAULT_FONTS,
    },
};

// German localization file.
static POPUP_DE_DE: &str = include_str!("assets/popup/popup_de_DE.ron");

static DARK_EXT: &str = include_str!("assets/popup/default_dark.ron");

static ID_BUTTON: &str = "Toggle Popup";
static ID_GRID: &str = "PopupGrid";
static ID_GRID_PLACEMENT: &str = "Selection GRID";
static ID_MAIN_VIEW: &str = "MainView";
static ID_POPUP: &str = "Popup";
static ID_POPUP_STACK: &str = "PopupStack";
static ID_PLACEMENT_COMBO_BOX: &str = "Placement Selecton Box";
static ID_PLACEMENT_OFFSET: &str = "Placement Offset";
static ID_PLACEMENT_OFFSET_LABEL: &str = "Offset";
static ID_PLACEMENT_COMBO_BOX_LABEL: &str = "Placement Label";
static ID_TARGET: &str = "Target";

/// Valid `actions` that are handled as state changes in the `MainView` widget.
#[derive(Copy, Clone)]
enum MainViewAction {
    /// Toggle the visibility of the popup
    TogglePopup,

    /// Update the offset of the popup relative to the target
    UpdateOffset,

    /// Update the placement of the popup relative to the target
    UpdatePlacement,
}

/// Valid `structures` that are handled inside the state of the `MainView` widget.
#[derive(AsAny, Default)]
struct MainViewState {
    _action: Option<MainViewAction>,
}

/// Method definitions, that react on any given state change inside the `MainView` widget.
impl MainViewState {
    fn _action(&mut self, action: impl Into<Option<MainViewAction>>) {
        self._action = action.into();
    }

    fn toggle_popup(&mut self, ctx: &mut Context<'_>) {
        let open = ctx.child(ID_POPUP).clone::<bool>("open");

        if open {
            Popup::open_set(&mut ctx.child(ID_POPUP), false);
            Popup::visibility_set(&mut ctx.child(ID_POPUP), Visibility::Collapsed);
            Button::text_set(&mut ctx.child(ID_BUTTON), "Click me to show the popup");
        } else {
            Popup::visibility_set(&mut ctx.child(ID_POPUP), Visibility::Visible);
            Popup::open_set(&mut ctx.child(ID_POPUP), true);
            Button::text_set(&mut ctx.child(ID_BUTTON), "Click me to hide the popup");
        }
    }

    fn update_placement(&mut self, ctx: &mut Context<'_>) {
        let selected_index: i32 = ctx.child(ID_PLACEMENT_COMBO_BOX).clone("selected_index");
        let placement: Placement = ctx.child(ID_POPUP).clone_or_default("placement");
        match selected_index {
            0 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.bottom()),
            1 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.bottom_left()),
            2 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.bottom_right()),
            3 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.left()),
            4 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.right()),
            5 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.top()),
            6 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.top_left()),
            7 => Popup::placement_set(&mut ctx.child(ID_POPUP), placement.top_right()),
            _ => panic!(),
        }
        if cfg!(debug) {
            println!(
                "popup_v2: Placement {:?} (index: {:?}).",
                ctx.child(ID_POPUP).get::<Placement>("placement"),
                selected_index
            );
        }
    }

    fn update_offset(&mut self, ctx: &mut Context<'_>) {
        let offset: f64 = ctx.child(ID_PLACEMENT_OFFSET).clone("val");
        Popup::offset_set(&mut ctx.child(ID_POPUP), offset);

        if cfg!(debug) {
            println!(
                "popup_v2: Offset {:?}.",
                ctx.child(ID_POPUP).get::<f64>("offset")
            );
        }
    }
}

/// Associated methods handled inside the `MainViewState`
impl State for MainViewState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        // initialize the popup properties
        ctx.child(ID_POPUP).set("open", false);
        ctx.child(ID_POPUP).set("visibility", Visibility::Collapsed);
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<MainViewAction>() {
            match message {
                MainViewAction::TogglePopup => {
                    MainViewState::toggle_popup(self, ctx);
                }
                MainViewAction::UpdatePlacement => {
                    MainViewState::update_placement(self, ctx);
                    }
                MainViewAction::UpdateOffset => {
                    MainViewState::update_offset(self, ctx);
                }
            }
        }
    }
}

fn theme() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(DARK_EXT)
            .extend(ThemeConfig::from(THEME_DEFAULT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

// Used to define ComboBox list members
type List = Vec<String>;

// constructs the MainView
widget!(
    MainView<MainViewState> {
    /// Active seleced index of combo box.
    selected_index: i32,

    /// Acitve selected position of popup.
    placements: List
    }
);

/// The template implementation of the main view
/// GUI elements are styled using the "style" attribute referencing to a ron based css
impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let placements = vec![
            // TODO: make it sort independent
            "Bottom".to_string(),
            "BottomLeft".to_string(),
            "BottomRight".to_string(),
            "Left".to_string(),
            "Right".to_string(),
            "Top".to_string(),
            "TopLeft".to_string(),
            "TopRight".to_string(),
        ];
        let count_placements = placements.len();

        let target_container = Container::new()
            .id(ID_TARGET)
            .attach(Grid::row(2))
            .attach(Grid::column(1))
            .style("container_form")
            .child(
                Stack::new()
                    .orientation("vertical")
                    .spacing(16.0)
                    .child(
                        TextBlock::new()
                            .style("target_text_block")
                            .text("Target")
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .style("target_text_block")
                            .text("Target content")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .build(ctx);

        let popup = Popup::new()
            .id(ID_POPUP)
            .name(ID_POPUP)
            .style("popup_form")
            // alternative: popup is target of given coordinates (Point)
            //.target(Point::new(0.0, 0.0))
            .height(180.0)
            .open(false)
            .offset(1.0)
            .width(180.0)
            .target(target_container)
        .child(
            Container::new()
                    .child(
                        Stack::new()
                            .id(ID_POPUP_STACK)
                            .orientation("vertical")
                            .spacing(16.0)
                .child(
                    TextBlock::new()
                                    .style("popup_text_block")
                        .h_align("center")
                                    .v_align("center")
                                    .text("Popup Header")
                                    .build(ctx),
                )
                            .child(
                                TextBlock::new()
                                    .style("popup_text_block")
                                    .h_align("center")
                                    .v_align("center")
                                    .text("Popup content")
                                    .build(ctx),
        )
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .build(ctx);

        let _ = ctx.append_child_to_overlay(popup);

        // Widget:  MainView
        self.id(ID_MAIN_VIEW)
            .name(ID_MAIN_VIEW)
            .margin(20.0)
            .placements(placements) // vector with placement strings
            .child(
                Grid::new()
                    .id(ID_GRID)
                    .rows("80, 200, 200, *")
                    .columns("200, 200, 250")
                .child(
                    Button::new()
                            .id(ID_BUTTON)
                            .attach(Grid::row(0))
                            .attach(Grid::column(1))
                            .text("Click me to show the popup")
                            .on_click(move |state, _entity| -> bool {
                                state.send_message(MainViewAction::TogglePopup, id);
                            true
                        })
                            .build(ctx),
                )
                    .child(
                        Grid::new()
                            .id(ID_GRID_PLACEMENT)
                            .attach(Grid::row(0))
                            .attach(Grid::column(2))
                            .rows("auto, 4, auto")
                            .columns("120, 8, auto")
                            //.style("container_form")
                            .name(ID_GRID_PLACEMENT)
                            .child(
                                TextBlock::new()
                                    .id(ID_PLACEMENT_COMBO_BOX_LABEL)
                                    .attach(Grid::row(0))
                                    .attach(Grid::column(0))
                                    .h_align("end")
                                    .text("Placement:")
                                    .v_align("center")
                                    .build(ctx),
                            )
                            .child(
                                ComboBox::new()
                                    .id(ID_PLACEMENT_COMBO_BOX)
                                    .attach(Grid::row(0))
                                    .attach(Grid::column(2))
                                    .count(count_placements)
                                    //.style("combo_box")
                                    .items_builder(move |ibc, index| {
                                        let text = MainView::placements_ref(&ibc.get_widget(id))
                                            [index]
                                            .clone();
                                        TextBlock::new().v_align("center").text(text).build(ibc)
                                    })
                                    .on_changed("selected_index", move |states, _entity| {
                                        states.send_message(MainViewAction::UpdatePlacement, id);
                                    })
                                    .selected_index(id)
                                    .build(ctx),
                            )
                            .child(
                                TextBlock::new()
                                    .id(ID_PLACEMENT_OFFSET_LABEL)
                                    .attach(Grid::row(2))
                                    .attach(Grid::column(0))
                                    .h_align("end")
                                    .text("Offset:")
                                    .v_align("center")
                                    .build(ctx),
                            )
                            .child(
                                NumericBox::new()
                                    .id(ID_PLACEMENT_OFFSET)
                                    .attach(Grid::row(2))
                                    .attach(Grid::column(2))
                                    .style("popup_numeric_box")
                                    .max(50)
                                    .step(1)
                                    .val(5)
                                    //.water_mark("in pixel")
                                    .on_changed("val", move |states, _entity| {
                                        states.send_message(MainViewAction::UpdateOffset, id);
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(target_container)
                    .build(ctx),
        )
    }
}

fn main() {
    // if no dictionary is set for the default language
    // the content of the text property will drawn.
    let localization = RonLocalization::create()
        .language("de_DE")
        //.language("en_US")
        .dictionary("de_DE", POPUP_DE_DE)
        .build();

    Application::new()
        .localization(localization)
        .theme(theme())
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Popup example")
                .position((100.0, 100.0))
                .size(680, 690.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
