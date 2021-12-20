use orbtk::{prelude::*, widgets::themes::*};

// German localization file.
static SHOWCASE_DE_DE: &str = include_str!("assets/showcase/showcase_de_DE.ron");

// Constants
static ID_BUTTON_GRID: &str = "ButtonGrid";
static ID_BUTTON_STACK: &str = "ButtonStack";
static ID_BUTTON_CHECK: &str = "ButtonCheck";
static ID_BUTTON_DISABLE: &str = "ButtonDisable";
static ID_BUTTON_PRIMARY: &str = "ButtonPrimary";
static ID_BUTTON_SINGLE: &str = "ButtonSingle";
static ID_BUTTON_TEXT: &str = "ButtonText";
static ID_BUTTON_TOGGLE: &str = "ButtonToggle";
static ID_CHECK_BOX: &str = "CheckBox";
static ID_CHECK_BOX_DISABLED: &str = "CheckBoxDisabled";
static ID_COMBO_BOX: &str = "ComboBox";
static ID_COMBO_BOX_TEXT: &str = "ComboBoxText";
static ID_IMAGE_WIDGET: &str = "ImageWidget";
static ID_INTERACTIVE_VIEW: &str = "InteractiveView";
static ID_INTERACTIVE_VIEW_BUTTON: &str = "InteractiveViewButton";
static ID_INTERACTIVE_VIEW_BUTTON2: &str = "InteractiveViewButton2";
static ID_INTERACTIVE_VIEW_BUTTON3: &str = "InteractiveViewButton3";
static ID_INTERACTIVE_VIEW_BUTTON4: &str = "InteractiveViewButton4";
static ID_INTERACTIVE_VIEW_COMBO_BOX: &str = "InteractiveViewComboBox";
static ID_INTERACTIVE_VIEW_TEXT_BLOCK: &str = "InteractiveViewTextBlock";
static ID_INTERACTIVE_VIEW_TEXT_BLOCK2: &str = "InteractiveViewTextBlock2";
static ID_INTERACTIVE_VIEW_TEXT_BLOCK3: &str = "InteractiveViewTextBlock3";
static ID_INTERACTIVE_VIEW_TEXT_BLOCK4: &str = "InteractiveViewTextBlock4";
static ID_INTERACTIVE_VIEW_TEXT_BLOCK5: &str = "InteractiveViewTextBlock5";
static ID_ITEMS_VIEW: &str = "ItemsView";
static ID_ITEMS_VIEW_STACK: &str = "ItemsViewStack";
static ID_ITEMS_VIEW_TEXT_BLOCK: &str = "ItemsViewHeader";
static ID_ITEMS_VIEW_TEXT_BLOCK_HEADER: &str = "ItemsViewTextBlockHeader";
static ID_ITEMS_VIEW_TEXT_BLOCK_HEADER_2: &str = "ItemsViewTextBlockHeader2";
static ID_ITEMS_VIEW_ITEMS_WIDGET: &str = "ItemsWidgetBody";
static ID_ITEMS_VIEW_ITEMS_WIDGET_TEXT_BLOCK: &str = "ItemsWidgetText";
static ID_LAYOUT_VIEW: &str = "LayoutView";
static ID_LAYOUT_VIEW_STACK: &str = "LayoutViewStack";
static ID_LAYOUT_VIEW_STACK_TEXT_BLOCK: &str = "LayoutViewStackTextBlock";
static ID_LAYOUT_VIEW_STACK_TEXT_BLOCK2: &str = "LayoutViewStackTextBlock2";
static ID_LAYOUT_VIEW_STACK_CONTAINER: &str = "LayoutViewStackContainer";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID: &str = "LayoutViewStackContainerGrid";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER: &str =
    "LayoutViewStackContainerGridContainer";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER_TEXT_BLOCK: &str =
    "LayoutViewStackContainerGridContainerTextBlock";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER2: &str =
    "LayoutViewStackContainerGridContainer2";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER2_TEXT_BLOCK: &str =
    "LayoutViewStackContainerGridContainer2TextBlock";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER3: &str =
    "LayoutViewStackContainerGridContainer2";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER3_TEXT_BLOCK: &str =
    "LayoutViewStackContainerGridContainer2TextBlock";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER4: &str =
    "LayoutViewStackContainerGridContainer2";
static ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER4_TEXT_BLOCK: &str =
    "LayoutViewStackContainerGridContainer2TextBlock";
static ID_LAYOUT_VIEW_STACK_CONTAINER2: &str = "LayoutViewStackContainer2";
static ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK: &str = "LayoutViewStackContainer2Stack";
static ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER: &str =
    "LayoutViewStackContainerStackContainer";
static ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER2: &str =
    "LayoutViewStackContainerStackContainer2";
static ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER3: &str =
    "LayoutViewStackContainerStackContainer3";
static ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER4: &str =
    "LayoutViewStackContainerStackContainer4";
static ID_LAYOUT_VIEW_STACK_CONTAINER2_TEXT_BLOCK: &str = "LayoutViewStackContainer2TextBlock";
static ID_LAYOUT_VIEW_STACK_CONTAINER3: &str = "LayoutViewStackContainer3";
static ID_LAYOUT_VIEW_STACK_CONTAINER3_CONTAINER: &str = "LayoutViewStackContainer3Container";
static ID_LIST_VIEW: &str = "ListView";
static ID_LOCALIZATION_VIEW_STACK: &str = "LocalizationViewStack";
static ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK: &str = "LocalizationViewStackTextBlock";
static ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK2: &str = "LocalizationViewStackTextBlock2";
static ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK3: &str = "LocalizationViewStackTextBlock3";
static ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK4: &str = "LocalizationViewStackTextBlock4";
static ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK5: &str = "LocalizationViewStackTextBlock5";
static ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK6: &str = "LocalizationViewStackTextBlock6";
static ID_LOCALIZATION_VIEW_STACK_COMBO_BOX: &str = "LocalizationViewStackComboBox";
static ID_LOCALIZATION_VIEW_STACK_COMBO_BOX_TEXT_BLOCK: &str =
    "LocalizationViewStackComboBoxTextBlock";
static ID_MAIN_VIEW: &str = "MainView";
static ID_NAVIGATION_VIEW: &str = "NavigationView";
static ID_NAVIGATION_VIEW_GRID: &str = "NavigationViewPagerGrid";
static ID_NAVIGATION_VIEW_GRID_BUTTON: &str = "NavigationViewPagerGridButton";
static ID_NAVIGATION_VIEW_GRID_BUTTON2: &str = "NavigationViewPagerGridButton2";
static ID_NAVIGATION_VIEW_GRID_TEXT_BLOCK: &str = "NavigationViewPagerGridTextBlock";
static ID_NAVIGATION_VIEW_GRID_TEXT_BLOCK2: &str = "NavigationViewPagerGridTextBlock2";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL: &str = "NavigationViewPagerGridMasterDetail";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER: &str =
    "NavigationViewGridMasterDetailContainer";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK: &str =
    "NavigationViewGridMasterDetailContainerStack";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK_TEXT_BLOCK: &str =
    "NavigationViewGridMasterDetailContainerStackTextBlock";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK_TEXT_BLOCK2: &str =
    "NavigationViewGridMasterDetailContainerStackTextBlock2";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_TEXT_BLOCK: &str =
    "NavigationViewGridMasterDetailContainerTextBlock";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_BUTTON: &str =
    "NavigationViewGridMasterDetailContainerButton";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2: &str =
    "NavigationViewGridMasterDetailContainer2";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_TEXT_BLOCK: &str =
    "NavigationViewGridMasterDetailContainer2TextBlock";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_TEXT_BLOCK2: &str =
    "NavigationViewGridMasterDetailContainer2TextBlock2";
static ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_BUTTON: &str =
    "NavigationViewGridMasterDetailContainer2Button";
static ID_NAVIGATION_VIEW_PAGER: &str = "NavigationViewPager";
static ID_NAVIGATION_VIEW_PAGER_CONTAINER: &str = "NavigationViewPagerContainer";
static ID_NAVIGATION_VIEW_PAGER_CONTAINER2: &str = "NavigationViewPagerContainer2";
static ID_NAVIGATION_VIEW_PAGER_CONTAINER3: &str = "NavigationViewPagerContainer3";
static ID_PASSWORD_BOX: &str = "PasswordBox";
static ID_PROGRESS_BAR: &str = "ProgressBar";
static ID_SLIDER: &str = "Slider";
static ID_SWITCH: &str = "Switch";
static ID_SWITCH_DISABLED: &str = "SwitchDisabled";
static ID_STACK: &str = "Stack";
static ID_STACK_TEXT_BLOCK_HEADER: &str = "StackTextBLockHeader";
static ID_STACK_TEXT_BLOCK_TEXT: &str = "StackTextBlockText";
static ID_STACK_TEXT_BOX_WATER_MARK: &str = "StackTextBoxWaterMark";
static ID_TAB_WIDGET: &str = "TabWidget";
static ID_WINDOW: &str = "Window";

fn main() {
    // if no dictionary is set for the default language e.g. english the content of the text property will drawn.
    let localization = RonLocalization::create()
        .language("en_US")
        .dictionary("de_DE", SHOWCASE_DE_DE)
        .build();

    Application::new()
        // .theme(theme_default_light())
        .localization(localization)
        .window(|ctx| {
            Window::new()
                .id(ID_WINDOW)
                .name(ID_WINDOW)
                .title("OrbTk - showcase example")
                .position((100, 100))
                .size(1000, 730)
                .resizable(true)
                .child(
                    MainView::new()
                        .id(ID_MAIN_VIEW)
                        .name(ID_MAIN_VIEW)
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}

// [START] views

// Represents the main wrapper view with main navigation.
widget!(MainView {});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.id(ID_MAIN_VIEW).name(ID_MAIN_VIEW).child(
            TabWidget::new()
                .id(ID_TAB_WIDGET)
                .name(ID_TAB_WIDGET)
                .tab("Buttons / Text", ButtonView::new().build(ctx))
                .tab("Items", ItemsView::new().build(ctx))
                .tab("Layouts", LayoutView::new().build(ctx))
                .tab("Image", ImageView::new().build(ctx))
                .tab("Localization", LocalizationView::new().build(ctx))
                .tab("Navigation", NavigationView::new().build(ctx))
                .tab("Interactive", InteractiveView::new().build(ctx))
                .build(ctx),
        )
    }
}

// Represents an overview with button and text widgets.
widget!(ButtonView {});

impl Template for ButtonView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let slider = Slider::new()
            .id(ID_SLIDER)
            .name(ID_SLIDER)
            .min(0.0)
            .max(1.0)
            .build(ctx);
        self.child(
            Grid::new()
                .id(ID_BUTTON_GRID)
                .name(ID_BUTTON_GRID)
                .margin(16)
                .columns("140, 32, 140")
                .child(
                    Stack::new()
                        .id(ID_BUTTON_STACK)
                        .name(ID_BUTTON_STACK)
                        .spacing(8)
                        .child(
                            Button::new()
                                .id(ID_BUTTON_CHECK)
                                .name(ID_BUTTON_CHECK)
                                .text("Button")
                                .icon(material_icons_font::MD_CHECK)
                                .on_enter(|_, _| {
                                    println!("Enter Button boundries");
                                })
                                .on_leave(|_, _| {
                                    println!("Leave Button boundries");
                                })
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .id(ID_BUTTON_DISABLE)
                                .name(ID_BUTTON_DISABLE)
                                .enabled(false)
                                .text("disabled")
                                .icon(material_icons_font::MD_CHECK)
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .id(ID_BUTTON_PRIMARY)
                                .name(ID_BUTTON_PRIMARY)
                                .text("Primary")
                                .style("button_primary")
                                .icon(material_icons_font::MD_360)
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .id(ID_BUTTON_TEXT)
                                .name(ID_BUTTON_TEXT)
                                .text("Text only")
                                .style("button_single_content")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .id(ID_BUTTON_SINGLE)
                                .name(ID_BUTTON_SINGLE)
                                .icon(material_icons_font::MD_CHECK)
                                .style("button_single_content")
                                .build(ctx),
                        )
                        .child(
                            ToggleButton::new()
                                .id(ID_BUTTON_TOGGLE)
                                .name(ID_BUTTON_TOGGLE)
                                .text("ToggleButton")
                                .icon(material_icons_font::MD_ALARM_ON)
                                .build(ctx),
                        )
                        .child(
                            CheckBox::new()
                                .id(ID_CHECK_BOX)
                                .name(ID_CHECK_BOX)
                                .text("CheckBox")
                                .build(ctx),
                        )
                        .child(
                            CheckBox::new()
                                .id(ID_CHECK_BOX_DISABLED)
                                .name(ID_CHECK_BOX_DISABLED)
                                .enabled(false)
                                .text("disabled")
                                .build(ctx),
                        )
                        .child(Switch::new().id(ID_SWITCH).name(ID_SWITCH).build(ctx))
                        .child(
                            Switch::new()
                                .id(ID_SWITCH_DISABLED)
                                .name(ID_SWITCH_DISABLED)
                                .enabled(false)
                                .build(ctx),
                        )
                        .child(slider)
                        .child(
                            ProgressBar::new()
                                .id(ID_PROGRESS_BAR)
                                .name(ID_PROGRESS_BAR)
                                .val(slider)
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Stack::new()
                        .id(ID_STACK)
                        .name(ID_STACK)
                        .attach(Grid::column(2))
                        .spacing(8)
                        .child(
                            TextBlock::new()
                                .id(ID_STACK_TEXT_BLOCK_HEADER)
                                .name(ID_STACK_TEXT_BLOCK_HEADER)
                                .style("header")
                                .text("Header")
                                .build(ctx),
                        )
                        .child(
                            TextBlock::new()
                                .id(ID_STACK_TEXT_BLOCK_TEXT)
                                .name(ID_STACK_TEXT_BLOCK_TEXT)
                                .style("body")
                                .text("Text")
                                .build(ctx),
                        )
                        .child(
                            TextBox::new()
                                .id(ID_STACK_TEXT_BOX_WATER_MARK)
                                .name(ID_STACK_TEXT_BOX_WATER_MARK)
                                .water_mark("Insert text...")
                                .build(ctx),
                        )
                        .child(
                            PasswordBox::new()
                                .id(ID_PASSWORD_BOX)
                                .name(ID_PASSWORD_BOX)
                                .water_mark("Insert password...")
                                .build(ctx),
                        )
                        .child(NumericBox::new().max(123).step(0.123).val(0.123).build(ctx))
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

type List = Vec<String>;

// Represents an overview of list widgets like ListView, ItemsWidget and ComboBox.
widget!(ItemsView { items: List });

impl Template for ItemsView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let items = vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 4".to_string(),
            "Item 5".to_string(),
        ];
        let count = items.len();

        self.id(ID_ITEMS_VIEW)
            .name(ID_ITEMS_VIEW)
            .items(items)
            .child(
                Stack::new()
                    .id(ID_ITEMS_VIEW_STACK)
                    .name(ID_ITEMS_VIEW_STACK)
                    .width(140)
                    .margin(16)
                    .spacing(8)
                    .child(
                        TextBlock::new()
                            .id(ID_ITEMS_VIEW_TEXT_BLOCK)
                            .name(ID_ITEMS_VIEW_TEXT_BLOCK)
                            .text("ItemsWidget")
                            .style("header")
                            .build(ctx),
                    )
                    .child(
                        ItemsWidget::new()
                            .id(ID_ITEMS_VIEW_ITEMS_WIDGET)
                            .name(ID_ITEMS_VIEW_ITEMS_WIDGET)
                            .count(count)
                            .items_builder(move |bc, index| {
                                let text =
                                    bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                                TextBlock::new()
                                    .style("body")
                                    .id(ID_ITEMS_VIEW_ITEMS_WIDGET_TEXT_BLOCK)
                                    .name(ID_ITEMS_VIEW_ITEMS_WIDGET_TEXT_BLOCK)
                                    .text(text)
                                    .v_align("center")
                                    .build(bc)
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .id(ID_ITEMS_VIEW_TEXT_BLOCK_HEADER)
                            .name(ID_ITEMS_VIEW_TEXT_BLOCK_HEADER)
                            .style("header")
                            .text("ListView")
                            .build(ctx),
                    )
                    .child(
                        ListView::new()
                            .id(ID_LIST_VIEW)
                            .name(ID_LIST_VIEW)
                            .count(count)
                            .items_builder(move |bc, index| {
                                let text =
                                    bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                                TextBlock::new().v_align("center").text(text).build(bc)
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .id(ID_ITEMS_VIEW_TEXT_BLOCK_HEADER_2)
                            .name(ID_ITEMS_VIEW_TEXT_BLOCK_HEADER_2)
                            .style("header")
                            .text("ComboBox")
                            .build(ctx),
                    )
                    .child(
                        ComboBox::new()
                            .id(ID_COMBO_BOX)
                            .name(ID_COMBO_BOX)
                            .count(count)
                            .enabled(false)
                            .items_builder(move |bc, index| {
                                let text = ItemsView::items_ref(&bc.get_widget(id))[index].clone();
                                TextBlock::new()
                                    .id(ID_COMBO_BOX_TEXT)
                                    .text(text)
                                    .v_align("center")
                                    .build(bc)
                            })
                            .selected_index(0)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

// Represents an overview of layout widgets.
widget!(LayoutView {});

impl Template for LayoutView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.id(ID_LAYOUT_VIEW)
            .name(ID_LAYOUT_VIEW)
            .child(
                Stack::new()
                    .id(ID_LAYOUT_VIEW_STACK)
                    .name(ID_LAYOUT_VIEW_STACK)
                    .width(400)
                    .margin(16)
                    .spacing(8)
                    .child(TextBlock::new()
                           .id(ID_LAYOUT_VIEW_STACK_TEXT_BLOCK)
                           .name(ID_LAYOUT_VIEW_STACK_TEXT_BLOCK)
                           .style("header")
                           .text("Grid")
                           .build(ctx))
                    .child(
                        Container::new()
                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER)
                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER)
                            .width(300)
                            .height(150)
                            .background("black")
                            .child(
                                Grid::new()
                                    .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID)
                                    .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID)
                                    .columns("*, auto, 50")
                                    .rows("*, *")
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER)
                                            .background("lynch")
                                            .margin((10, 0, 0, 4))
                                            .attach(Grid::column(0))
                                            .child(
                                                TextBlock::new()
                                                    .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER_TEXT_BLOCK)
                                                    .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER_TEXT_BLOCK)
                                                    .text("(0,0)")
                                                    .style("light_text")
                                                    .h_align("center")
                                                    .v_align("center")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER2)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER2)
                                            .background("bluebayoux")
                                            .margin(10)
                                            .constraint(Constraint::create().width(150))
                                            .attach(Grid::column(1))
                                            .child(
                                                TextBlock::new()
                                                    .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER2_TEXT_BLOCK)
                                                    .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER2_TEXT_BLOCK)
                                                    .text("(1,0)")
                                                    .style("body")
                                                    .h_align("center")
                                                    .v_align("center")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER3)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER3)
                                            .background("linkwater")
                                            .attach(Grid::column(2))
                                            .child(
                                                TextBlock::new()
                                                    .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER3_TEXT_BLOCK)
                                                    .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER3_TEXT_BLOCK)
                                                    .text("(2,0)")
                                                    .foreground("black")
                                                    .h_align("center")
                                                    .v_align("center")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER4)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER4)
                                            .background("goldendream")
                                            .attach(Grid::column(0))
                                            .attach(Grid::row(1))
                                            .attach(Grid::column_span(3))
                                            .child(
                                                TextBlock::new()
                                                    .id(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER4_TEXT_BLOCK)
                                                    .name(ID_LAYOUT_VIEW_STACK_CONTAINER_GRID_CONTAINTER4_TEXT_BLOCK)
                                                    .text("(0,1) - ColumnSpan 3")
                                                    .foreground("black")
                                                    .h_align("center")
                                                    .v_align("center")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(TextBlock::new()
                           .id(ID_LAYOUT_VIEW_STACK_TEXT_BLOCK2)
                           .name(ID_LAYOUT_VIEW_STACK_TEXT_BLOCK2)
                           .style("header")
                           .text("Stack")
                           .build(ctx))
                    .child(
                        Container::new()
                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER2)
                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER2)
                            .background("black")
                            .width(300)
                            .child(
                                Stack::new()
                                    .id(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK)
                                    .name(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK)
                                    .spacing(4)
                                    .child(Container::new()
                                           .id(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER)
                                           .name(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER)
                                           .background("lynch").height(50).build(ctx))
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER2)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER2)
                                            .background("bluebayoux")
                                            .height(50)
                                            .build(ctx),
                                    )
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER3)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER3)
                                            .background("linkwater")
                                            .height(50)
                                            .build(ctx),
                                    )
                                    .child(
                                        Container::new()
                                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER4)
                                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER2_STACK_CONTAINER4)
                                            .background("goldendream")
                                            .height(50)
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(TextBlock::new()
                           .id(ID_LAYOUT_VIEW_STACK_CONTAINER2_TEXT_BLOCK)
                           .name(ID_LAYOUT_VIEW_STACK_CONTAINER2_TEXT_BLOCK)
                           .style("header")
                           .text("Padding")
                           .build(ctx))
                    .child(
                        Container::new()
                            .id(ID_LAYOUT_VIEW_STACK_CONTAINER3)
                            .name(ID_LAYOUT_VIEW_STACK_CONTAINER3)
                            .background("black")
                            .width(300)
                            .height(150)
                            .padding((16, 8, 16, 8))
                            .child(Container::new()
                                   .id(ID_LAYOUT_VIEW_STACK_CONTAINER3_CONTAINER)
                                   .background("lynch")
                                   .build(ctx))
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

// Represents an overview of the image widget.
widget!(ImageView {});

impl Template for ImageView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            ImageWidget::new()
                .id(ID_IMAGE_WIDGET)
                .name(ID_IMAGE_WIDGET)
                .margin(16)
                .image("orbtk/examples/assets/showcase/orbtk_logo.png")
                .build(ctx),
        )
    }
}

// Contains an example how to use localization in OrbTk.
widget!(LocalizationView<LocalizationState> { languages: List, selected_index: i32 });

impl Template for LocalizationView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let languages = vec!["English".to_string(), "German".to_string()];
        let count = languages.len();

        self.languages(languages).selected_index(0).child(
            Stack::new()
                .id(ID_LOCALIZATION_VIEW_STACK)
                .name(ID_LOCALIZATION_VIEW_STACK)
                .width(120)
                .margin(16)
                .spacing(8)
                .child(
                    TextBlock::new()
                        .id(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK)
                        .name(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK)
                        .style("small_text")
                        .text("Hello")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .id(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK2)
                        .name(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK2)
                        .style("small_text")
                        .text("world")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .id(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK3)
                        .name(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK3)
                        .style("small_text")
                        .text("I")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .id(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK4)
                        .name(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK4)
                        .style("small_text")
                        .text("love")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("small_text")
                        .id(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK5)
                        .name(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK5)
                        .text("localization")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .id(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK6)
                        .name(ID_LOCALIZATION_VIEW_STACK_TEXT_BLOCK6)
                        .style("small_text")
                        .text("!")
                        .build(ctx),
                )
                .child(
                    ComboBox::new()
                        .id(ID_LOCALIZATION_VIEW_STACK_COMBO_BOX)
                        .name(ID_LOCALIZATION_VIEW_STACK_COMBO_BOX)
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text =
                                LocalizationView::languages_ref(&bc.get_widget(id))[index].clone();
                            TextBlock::new()
                                .id(ID_LOCALIZATION_VIEW_STACK_COMBO_BOX_TEXT_BLOCK)
                                .name(ID_LOCALIZATION_VIEW_STACK_COMBO_BOX_TEXT_BLOCK)
                                .v_align("center")
                                .text(text)
                                .build(bc)
                        })
                        .on_changed("selected_index", move |states, _| {
                            states.get_mut::<LocalizationState>(id).change_language();
                        })
                        .selected_index(id)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

// Represents an overview of navigation widgets.
widget!(
    NavigationView<NavigationState> {
        md_navigation_visibility: Visibility
    }
);

impl Template for NavigationView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let pager = Pager::new()
            .id(ID_NAVIGATION_VIEW_PAGER)
            .attach(Grid::row(1))
            .child(
                Container::new()
                    .id(ID_NAVIGATION_VIEW_PAGER_CONTAINER)
                    .name(ID_NAVIGATION_VIEW_PAGER_CONTAINER)
                    .padding(8)
                    .background("lynch")
                    .child(TextBlock::new().text("Page 1").build(ctx))
                    .build(ctx),
            )
            .child(
                Container::new()
                    .id(ID_NAVIGATION_VIEW_PAGER_CONTAINER2)
                    .name(ID_NAVIGATION_VIEW_PAGER_CONTAINER2)
                    .padding(8)
                    .background("goldendream")
                    .child(
                        TextBlock::new()
                            .foreground("black")
                            .text("Page 2")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .child(
                Container::new()
                    .id(ID_NAVIGATION_VIEW_PAGER_CONTAINER3)
                    .name(ID_NAVIGATION_VIEW_PAGER_CONTAINER3)
                    .padding(8)
                    .background("linkwater")
                    .child(
                        TextBlock::new()
                            .foreground("black")
                            .text("Page 3")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .build(ctx);

        self.id(ID_NAVIGATION_VIEW)
            .name(ID_NAVIGATION_VIEW)
            .child(
                Grid::new()
                    .id(ID_NAVIGATION_VIEW_GRID)
                    .name(ID_NAVIGATION_VIEW_GRID)
                    .margin(16)
                    .rows("32, *, 8, auto, 8, 32, *")
                    .child(TextBlock::new()
                           .id(ID_NAVIGATION_VIEW_GRID_TEXT_BLOCK)
                           .name(ID_NAVIGATION_VIEW_GRID_TEXT_BLOCK)
                           .style("header")
                           .text("Pager")
                           .build(ctx))
                    .child(pager)
                    .child(
                        Button::new()
                            .id(ID_NAVIGATION_VIEW_GRID_BUTTON)
                            .name(ID_NAVIGATION_VIEW_GRID_BUTTON)
                            .style("button_single_content")
                            .icon(material_icons_font::MD_KEYBOARD_ARROW_LEFT)
                            .h_align("start")
                            .attach(Grid::row(3))
                            .on_click(move |states, _| {
                                states.send_message(PagerAction::Previous, pager);
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .id(ID_NAVIGATION_VIEW_GRID_BUTTON2)
                            .name(ID_NAVIGATION_VIEW_GRID_BUTTON2)
                            .style("button_single_content")
                            .attach(Grid::row(3))
                            .h_align("end")
                            .icon(material_icons_font::MD_KEYBOARD_ARROW_RIGHT)
                            .on_click(move |states, _| {
                                states.send_message(PagerAction::Next, pager);
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .id(ID_NAVIGATION_VIEW_GRID_TEXT_BLOCK2)
                            .text("MasterDetail")
                            .attach(Grid::row(5))
                            .style("header")
                            .build(ctx),
                    )
                    .child(
                        MasterDetail::new()
                            .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL)
                            .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL)
                            .responsive(true)
                            .break_point(1000)
                            .navigation_visibility(("md_navigation_visibility", id))
                            .attach(Grid::row(6))
                            .master_detail(
                                Container::new()
                                    .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER)
                                    .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER)
                                    .padding(8)
                                    .background("lynch")
                                    .child(
                                        Stack::new()
                                            .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK)
                                            .orientation("vertical")
                                            .h_align("center")
                                            .v_align("center")
                                            .child(TextBlock::new().text("Content inside the master pane")
                                                   .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK_TEXT_BLOCK)
                                                   .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK_TEXT_BLOCK)
                                                   .font_size(16)
                                                   .build(ctx))
                                            .child(TextBlock::new().text("Resize the window: Pane brake is set to 800 pixel")
                                                   .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK_TEXT_BLOCK2)
                                                   .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_STACK_TEXT_BLOCK2)
                                                   .font_size(14)
                                                   .build(ctx))
                                            .build(ctx))
                                    .child(TextBlock::new()
                                           .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_TEXT_BLOCK)
                                           .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_TEXT_BLOCK)
                                           .text("Master Pane")
                                           .v_align("end")
                                           .build(ctx))
                                    .child(
                                        Button::new()
                                            .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_BUTTON)
                                            .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER_BUTTON)
                                            .style("button_primary_single_content")
                                            .visibility(("md_navigation_visibility", id))
                                            .h_align("start")
                                            .text("show detail pane")
                                            .on_click(move |ctx, _| {
                                                ctx.send_message(MasterDetailAction::ShowDetail, id);
                                                true
                                            })
                                            .build(ctx),
                                    )
                                    .build(ctx),
                                Container::new()
                                    .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2)
                                    .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2)
                                    .padding(8)
                                    .background("goldendream")
                                    .child(TextBlock::new()
                                           .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_TEXT_BLOCK)
                                           .font_size(14)
                                           .foreground("black")
                                           .h_align("center")
                                           .text("Content inside the detail pane")
                                           .v_align("center")
                                           .build(ctx))
                                    .child(
                                        TextBlock::new()
                                            .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_TEXT_BLOCK2)
                                            .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_TEXT_BLOCK2)
                                            .text("Detail Pane")
                                            .v_align("end")
                                            .foreground("black")
                                            .margin(8)
                                            .build(ctx),
                                    )
                                    .child(
                                        Button::new()
                                            .id(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_BUTTON)
                                            .name(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL_CONTAINER2_BUTTON)
                                            .text("back")
                                            .style("button_single_content")
                                            .visibility(("md_navigation_visibility", id))
                                            .h_align("start")
                                            .on_click(move |ctx, _| {
                                                ctx.send_message(MasterDetailAction::ShowMaster, id);
                                                true
                                            })
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

// Contains examples how interaction works in OrbTk.
widget!(
    InteractiveView<InteractiveState> {
        settings_text: String,
        themes: List,
        selected_index: i32,
        count_text: String
    }
);

impl Template for InteractiveView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let themes = vec![
            "default_dark".to_string(),
            "default_light".to_string(),
            "redox".to_string(),
            "fluent_dark".to_string(),
            "fluent_light".to_string(),
        ];
        let themes_count = themes.len();

        self.id(ID_INTERACTIVE_VIEW)
            .name(ID_INTERACTIVE_VIEW)
            .count_text("0")
            .themes(themes)
            .child(
                Grid::new()
                    .name(ID_INTERACTIVE_VIEW_TEXT_BLOCK)
                    .margin(8)
                    .rows("auto, 4, 32, 8, auto, 3, 32, 8, auto, 4, 32, 4, auto, 4, 32")
                    .columns("auto, 4, auto, 4, auto, *")
                    // theme selection
                    .child(
                        TextBlock::new()
                            .id(ID_INTERACTIVE_VIEW_TEXT_BLOCK)
                            .name(ID_INTERACTIVE_VIEW_TEXT_BLOCK)
                            .style("header")
                            .attach(Grid::row(0))
                            .attach(Grid::column(0))
                            .style("small_text")
                            .text("Select theme")
                            .build(ctx),
                    )
                    .child(
                        ComboBox::new()
                            .id(ID_INTERACTIVE_VIEW_COMBO_BOX)
                            .name(ID_INTERACTIVE_VIEW_COMBO_BOX)
                            .attach(Grid::row(2))
                            .attach(Grid::column(0))
                            .count(themes_count)
                            .items_builder(move |bc, index| {
                                let text =
                                    InteractiveView::themes_ref(&bc.get_widget(id))[index].clone();
                                TextBlock::new().v_align("center").text(text).build(bc)
                            })
                            .on_changed("selected_index", move |ctx, _| {
                                ctx.send_message(InteractiveAction::ChangeTheme, id);
                            })
                            .selected_index(id)
                            .build(ctx),
                    )
                    // Settings
                    .child(
                        TextBlock::new()
                            .id(ID_INTERACTIVE_VIEW_TEXT_BLOCK2)
                            .name(ID_INTERACTIVE_VIEW_TEXT_BLOCK2)
                            .h_align("start")
                            .attach(Grid::row(4))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(5))
                            .text("Settings")
                            .style("header")
                            .build(ctx),
                    )
                    .child(
                        TextBox::new()
                            .id(ID_INTERACTIVE_VIEW_TEXT_BLOCK3)
                            .name(ID_INTERACTIVE_VIEW_TEXT_BLOCK3)
                            .text(("settings_text", id))
                            .attach(Grid::row(6))
                            .attach(Grid::column(0))
                            .water_mark("Insert text...")
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .id(ID_INTERACTIVE_VIEW_BUTTON)
                            .name(ID_INTERACTIVE_VIEW_BUTTON)
                            .style("button_single_content")
                            .attach(Grid::row(6))
                            .attach(Grid::column(2))
                            .text("load")
                            .on_click(move |ctx, _| {
                                ctx.send_message(InteractiveAction::LoadSettings, id);
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .id(ID_INTERACTIVE_VIEW_BUTTON2)
                            .name(ID_INTERACTIVE_VIEW_BUTTON2)
                            .style("button_single_content")
                            .attach(Grid::row(6))
                            .attach(Grid::column(4))
                            .text("save")
                            .on_click(move |ctx, _| {
                                ctx.send_message(InteractiveAction::SaveSettings, id);
                                true
                            })
                            .build(ctx),
                    )
                    // Counter
                    .child(
                        TextBlock::new()
                            .id(ID_INTERACTIVE_VIEW_TEXT_BLOCK4)
                            .name(ID_INTERACTIVE_VIEW_TEXT_BLOCK4)
                            .h_align("start")
                            .attach(Grid::row(8))
                            .attach(Grid::column(0))
                            .attach(Grid::column_span(5))
                            .text("Counter")
                            .style("header")
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .id(ID_INTERACTIVE_VIEW_BUTTON3)
                            .name(ID_INTERACTIVE_VIEW_BUTTON3)
                            .style("button_single_content")
                            .attach(Grid::row(10))
                            .attach(Grid::column(0))
                            .icon(material_icons_font::MD_PLUS)
                            .on_click(move |ctx, _| {
                                ctx.send_message(InteractiveAction::Increment, id);
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::new()
                            .id(ID_INTERACTIVE_VIEW_TEXT_BLOCK5)
                            .name(ID_INTERACTIVE_VIEW_TEXT_BLOCK5)
                            .style("body")
                            .h_align("center")
                            .v_align("center")
                            .attach(Grid::row(12))
                            .attach(Grid::column(0))
                            .text(("count_text", id))
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .id(ID_INTERACTIVE_VIEW_BUTTON4)
                            .name(ID_INTERACTIVE_VIEW_BUTTON4)
                            .style("button_single_content")
                            .attach(Grid::row(14))
                            .attach(Grid::column(0))
                            .icon(material_icons_font::MD_MINUS)
                            .on_click(move |ctx, _| {
                                ctx.send_message(InteractiveAction::Decrement, id);
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

// [END] views

// [START] states

#[derive(Debug, Default, AsAny)]
struct NavigationState {
    master_detail: Entity,
}

impl State for NavigationState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.master_detail = ctx.child(ID_NAVIGATION_VIEW_GRID_MASTER_DETAIL).entity();
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<MasterDetailAction>() {
            ctx.send_message(message, self.master_detail);
        }
    }
}

#[derive(Debug, Default, AsAny)]
struct LocalizationState {
    change_language: bool,
}

impl LocalizationState {
    fn change_language(&mut self) {
        self.change_language = true;
    }
}

impl State for LocalizationState {
    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !self.change_language {
            return;
        }

        let index = *LocalizationView::selected_index_ref(&ctx.widget()) as usize;
        let selected_language = LocalizationView::languages_ref(&ctx.widget())[index].clone();

        match selected_language.as_str() {
            "English" => ctx.set_language("en_US"),
            "German" => ctx.set_language("de_DE"),
            _ => (),
        }

        self.change_language = false;
    }
}

#[derive(Debug, Default, AsAny)]
struct InteractiveState {
    count: i32,
}

impl State for InteractiveState {
    fn messages(
        &mut self,
        mut messages: MessageReader,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<InteractiveAction>() {
            match message {
                InteractiveAction::LoadSettings => registry
                    .get::<Settings>("settings")
                    .load_async::<SettingsData>("settings_data".to_string(), ctx.entity()),
                InteractiveAction::SaveSettings => {
                    let text: String = InteractiveView::settings_text_clone(&ctx.widget());
                    registry.get::<Settings>("settings").save_async(
                        "settings_data".to_string(),
                        SettingsData(text),
                        ctx.entity(),
                    );
                }
                InteractiveAction::ChangeTheme => {
                    let theme_index = *InteractiveView::selected_index_ref(&ctx.widget());

                    match theme_index {
                        0 => ctx.switch_theme(Rc::new(theme_default_dark())),
                        1 => ctx.switch_theme(Rc::new(theme_default_light())),
                        2 => ctx.switch_theme(Rc::new(theme_redox())),
                        3 => ctx.switch_theme(Rc::new(theme_fluent_dark())),
                        4 => ctx.switch_theme(Rc::new(theme_fluent_light())),
                        _ => {}
                    }
                }
                InteractiveAction::Increment => {
                    self.count += 1;
                    InteractiveView::count_text_set(&mut ctx.widget(), self.count.to_string());
                }
                InteractiveAction::Decrement => {
                    self.count -= 1;
                    InteractiveView::count_text_set(&mut ctx.widget(), self.count.to_string());
                }
            }
        }

        // save result
        for message in messages.read::<SettingsResult<()>>() {
            println!("Result {:?}", message);
        }

        // load result
        for message in messages.read::<SettingsResult<SettingsData>>().flatten() {
            InteractiveView::settings_text_set(&mut ctx.widget(), message.0);
        }
    }
}

// [END] states

// [START] Dummy data

#[derive(Clone, Debug)]
enum InteractiveAction {
    SaveSettings,
    LoadSettings,
    ChangeTheme,
    Increment,
    Decrement,
}

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsData(pub String);

// [END] Dummy data
