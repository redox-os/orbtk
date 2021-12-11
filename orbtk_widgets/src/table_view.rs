use super::behaviors::MouseBehavior;
use crate::{api::prelude::*, prelude::*, proc_macros::*};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

// Constants
const ID_TABLE_VIEW: &str = "TableView";
const ID_TABLE_VIEW_STACK: &str = "__STACK__";
const ID_TABLE_VIEW_GRID_HEADER: &str = "__COLUMNS_HEADER__";
const ID_TABLE_VIEW_GRID_DATA: &str = "__DATA_GRID__";
const ICON_ORDER_ASCENDING: &str = "\u{e05e}";
const ICON_ORDER_DESCENDING: &str = "\u{e068}";

type RowBuilder = Option<Box<dyn Fn(&mut BuildContext, usize, &mut Vec<Entity>)>>;
type RowSorter = Option<Box<dyn Fn(&str, TableSortDirection, Entity, &mut Context)>>;
pub enum TableAction {
    AddDefaultColumn(String, String),
    AddCustomColumn(Entity),
    Sort(String),
}

/// Specifies which column is the `TableView` sorted by.
#[derive(Clone, Debug, PartialEq)]
pub enum TableSortPredicate {
    ColumnHeaderId(String),
}

impl Default for TableSortPredicate {
    fn default() -> Self {
        TableSortPredicate::ColumnHeaderId(String::default())
    }
}

/// Specifies the order of the sorting of the rows in the `TableView`.
/// Default is TableSortDirection::Ascending.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TableSortDirection {
    /// Values are arranged from the `smallest` to the `largest`
    /// value. The lowest value will be placed on the top of the
    /// table. Sorting will continue and place the next increasing
    /// value in the row unit until it reaches the largest value that is
    /// placed on the bottom of the table.
    Ascending,
    /// Values are arranged from the `largest` to the `smallest`
    /// value. The highest value will be placed on the top of the
    /// table. Sorting will continue and place the next decreasing
    /// value in the row unit until it reaches the lowest value that is
    /// placed on the bottom of the table.
    Descending,
}

impl TableSortDirection {
    fn reverse(&mut self) {
        match self {
            TableSortDirection::Ascending => *self = TableSortDirection::Descending,
            TableSortDirection::Descending => *self = TableSortDirection::Ascending,
        }
    }
}

impl Default for TableSortDirection {
    fn default() -> Self {
        TableSortDirection::Ascending
    }
}

// [START] state: TableView

/// Specifies the structure to handle `TableView` elements and their associated states.
#[derive(Default, AsAny)]
struct TableViewState {
    /// Vector with handled actions
    actions: Vec<TableAction>,
    /// Sets or shares the count propery for given columns.
    column_count: usize,
    /// Sets the entity id for grid containing data values.
    data_grid: Entity,
    /// Sets or shares a HashMap that stores the column width for given data columns.
    data_column_width: HashMap<usize, f64>,
    /// Sets or shares a HashMap that stores the column width for given header columns.
    header_column_width: HashMap<usize, f64>,
    /// Sets the entity id for grid containing header values.
    header_grid: Entity,
    /// Triggers internal update method.
    request_update: bool,
    /// Calls the row builder.
    row_builder: RowBuilder,
    /// Sets or shares the count propery for given rows.
    row_count: usize,
    /// Sets or shares a reference to the selected enteties.
    selected_entities: RefCell<HashSet<Entity>>,
    /// Sets or shares the sorting order for the given header.
    sorted_column_header: Option<Entity>,
    /// Sets or shares the sorting order for the given rows.
    sorter: RowSorter,
}

/// The `TableViewState` handles column and row management. You may
/// create, sort and adjust them via the supported associated
/// functions.
impl TableViewState {
    fn adjust_columns(&self, ctx: &mut Context) {
        // Header and Data grid starts with no columns: adjust column count to real one
        let adjusted_columns = Blocks::create().repeat("auto", self.column_count).build();
        let cloned_columns = adjusted_columns.clone();
        ctx.get_widget(self.header_grid)
            .set::<Blocks>("columns", adjusted_columns);
        ctx.get_widget(self.data_grid)
            .set::<Blocks>("columns", cloned_columns);
        ctx.widget().set::<usize>("column_count", self.column_count);
    }

    // TODO: move to layout function
    fn adjust_column_width(&mut self, ctx: &mut Context) {
        let table_view = ctx.entity();

        // measure header column width
        let mut index = 0;
        ctx.change_into(self.header_grid);
        while let Some(header) = ctx.try_child_from_index(index) {
            // TODO: querying constraint always returns 0.0
            // Using the bounds of the header instead of
            let bounds = header.get::<Rectangle>("bounds");
            let header_width = bounds.width();
            let column_index = header.get::<usize>("column");
            self.header_column_width.insert(*column_index, header_width);
            index += 1;
        }

        ctx.change_into(table_view);

        // get data grid wildest width by columns: values already computed by the GridLayout since its ColumnWidth is auto.
        let data_grid = ctx.get_widget(self.data_grid);
        let data_columns_width = data_grid.get::<Blocks>("columns");
        for i in 0..data_columns_width.len() {
            if let Some(column) = data_columns_width.get(i) {
                let data_w = column.current_size();
                self.data_column_width.insert(i, data_w);
            }
        }

        // comparing header and data grid column width
        for i in 0..self.header_column_width.len() {
            let header_width = *self.header_column_width.get(&i).unwrap();
            let data_width = *self.data_column_width.get(&i).unwrap();

            if header_width < data_width {
                ctx.change_into(self.header_grid);
                if let Some(mut header) = ctx.try_child_from_index(i) {
                    if let Some(new_width) = self.data_column_width.get(&i) {
                        header
                            .get_mut::<Constraint>("constraint")
                            .set_width(*new_width);
                    }
                }
            } else if header_width > data_width {
                // change data grid children column width to match the column header width
                ctx.change_into(self.data_grid);
                let mut idx = 0;
                while let Some(mut child) = ctx.try_child_from_index(idx) {
                    if *child.get::<usize>("column") == i {
                        if let Some(new_width) = self.header_column_width.get(&i) {
                            child
                                .get_mut::<Constraint>("constraint")
                                .set_width(*new_width);
                        }
                    }
                    idx += 1;
                }
            }
        }
        ctx.change_into(table_view);

        self.header_column_width.clear();
        self.data_column_width.clear();
    }

    fn adjust_rows(&self, row_count: usize, ctx: &mut Context) {
        // Set data grid row count to real one
        let new_rows = Blocks::create().repeat("auto", row_count).build();
        ctx.get_widget(self.data_grid)
            .set::<Blocks>("rows", new_rows);
    }

    fn create_column_headers(&mut self, ctx: &mut Context) {
        for action in &self.actions {
            match action {
                TableAction::AddDefaultColumn(title, column_id) => {
                    let table_view = ctx.entity();
                    let column_id = column_id.clone();
                    let build_context = &mut ctx.build_context();
                    let header = Button::new()
                        .id(column_id.clone())
                        .icon_brush("#000000")
                        .style("table_column_header")
                        .text(title.clone())
                        .on_click(move |states, _| -> bool {
                            states
                                .get_mut::<TableViewState>(table_view)
                                .actions
                                .push(TableAction::Sort(column_id.clone()));
                            false
                        })
                        .build(build_context);

                    build_context.register_property::<usize>("column", header, self.column_count);
                    build_context.register_property::<usize>("row", header, 0);
                    build_context.append_child(self.header_grid, header);
                }
                TableAction::AddCustomColumn(header) => {
                    let build_context = &mut ctx.build_context();
                    build_context.register_property::<usize>("column", *header, self.column_count);
                    build_context.register_property::<usize>("row", *header, 0);
                    build_context.append_child(self.header_grid, *header);
                }
                _ => {}
            }
            self.column_count += 1;
        }
        assert!(
            self.column_count > 0,
            "You must define at least one column on a TableView!"
        );
        self.actions.clear();
        self.adjust_columns(ctx);
    }

    fn create_cells(&mut self, ctx: &mut Context) {
        let actual_row_count = ctx.widget().clone::<usize>("row_count");
        let should_update = ctx.widget().clone::<bool>("request_update");
        let table_view = ctx.entity();

        if actual_row_count != self.row_count || self.request_update || should_update {
            if let Some(row_builder) = &self.row_builder {
                self.request_update = false;
                ctx.widget().set::<bool>("request_update", false);
                self.adjust_rows(actual_row_count, ctx);

                // Probably the fastest method currently: only one allocation per update()
                // TO-DO: implement maybe a TableRowMapper ?
                let mut cells_of_row = Vec::with_capacity(actual_row_count);
                ctx.clear_children_of(self.data_grid);
                let build_context = &mut ctx.build_context();

                for i in 0..actual_row_count {
                    row_builder(build_context, i, &mut cells_of_row);

                    for (j, child) in cells_of_row.iter().enumerate().rev() {
                        // TableCell wraps the entity from the row_builder
                        let cell = {
                            let cell = TableCell::new()
                                .column_index(j)
                                .parent(table_view.0)
                                .row_index(i)
                                .build(build_context);

                            let mouse_behavior =
                                MouseBehavior::new().target(cell.0).build(build_context);
                            build_context.register_shared_property::<Selector>(
                                "selector",
                                mouse_behavior,
                                cell,
                            );
                            build_context.register_shared_property::<bool>(
                                "pressed",
                                mouse_behavior,
                                cell,
                            );
                            build_context.append_child(cell, mouse_behavior);

                            build_context.register_shared_property::<Brush>(
                                "foreground",
                                *child,
                                cell,
                            );
                            build_context
                                .register_shared_property::<f32>("opacity", cell, table_view);
                            build_context
                                .register_shared_property::<f32>("opacity", *child, table_view);
                            build_context.register_shared_property::<f64>(
                                "font_size",
                                *child,
                                cell,
                            );
                            build_context.register_shared_property::<f64>("font", *child, cell);
                            build_context.append_child(cell, *child);
                            //build_context.append_child(mouse_behavior, *child);
                            cell
                        };

                        build_context.register_property::<usize>("row", cell, i);
                        build_context.register_property::<usize>("column", cell, j);
                        build_context.append_child(self.data_grid, cell);
                    }

                    cells_of_row.clear();
                }

                self.row_count = actual_row_count;
            }
        }
    }

    fn remove_selection(&mut self, clear_selected: bool, ctx: &mut Context) {
        if clear_selected {
            for index in ctx
                .widget()
                .get::<SelectedEntities>("selected_entities")
                .0
                .clone()
                .symmetric_difference(&self.selected_entities.borrow())
            {
                let mut cell = ctx.get_widget(*index);
                cell.set::<bool>("selected", false);
                cell.get_mut::<Selector>("selector")
                    .remove_state("selected");
                cell.update(false);
            }
        }

        self.selected_entities.borrow_mut().clear();
        ctx.widget()
            .get_mut::<SelectedEntities>("selected_entities")
            .0
            .clear();
    }

    fn set_sort_direction_icon(
        &mut self,
        column_id: &str,
        order: TableSortDirection,
        ctx: &mut Context,
    ) {
        let table_view = ctx.entity();
        let arrow = match order {
            TableSortDirection::Ascending => String::from(ICON_ORDER_ASCENDING),
            TableSortDirection::Descending => String::from(ICON_ORDER_DESCENDING),
        };

        ctx.change_into(self.header_grid);
        // remove the sorted indicator arrow from the header
        if let Some(current_sorted_header) = self.sorted_column_header {
            let mut header = ctx.get_widget(current_sorted_header);
            header.set::<String>("icon", "".to_string());
        }

        if let Some(column_header) = ctx.entity_of_child(column_id) {
            let mut button = ctx.get_widget(column_header);
            button.set::<String>("icon", arrow);
            self.sorted_column_header = Some(column_header);
        }
        ctx.change_into(table_view);
    }

    fn sort_rows(&mut self, column_id: String, ctx: &mut Context) {
        // removing selected entities before sorting,
        // since the entities in this list will be invalidated after sorting
        self.remove_selection(true, ctx);

        if let Some(sorter) = &self.sorter {
            let column_str = column_id.as_str();
            let new_sort_predicate = TableSortPredicate::ColumnHeaderId(column_id.clone());
            let old_sort_predicate = ctx.widget().clone::<TableSortPredicate>("sort_predicate");

            if new_sort_predicate == old_sort_predicate {
                ctx.widget()
                    .get_mut::<TableSortDirection>("sort_direction")
                    .reverse();
            }

            let sort_direction = ctx.widget().clone::<TableSortDirection>("sort_direction");
            let data_source = Entity::from(ctx.widget().clone::<u32>("data_source"));
            ctx.widget()
                .set::<TableSortPredicate>("sort_predicate", new_sort_predicate);
            sorter(column_str, sort_direction, data_source, ctx);
            self.set_sort_direction_icon(column_str, sort_direction, ctx);
            self.request_update = true;
        }
    }
}

/// Method definitions, that react on any given state change inside
/// the `TableViewItem` widget.
impl State for TableViewState {
    fn init(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        self.data_grid = ctx
            .entity_of_child(ID_TABLE_VIEW_GRID_DATA)
            .expect("TableViewState.init(): TableView data_grid can't be found.");
        self.header_grid = ctx
            .entity_of_child(ID_TABLE_VIEW_GRID_HEADER)
            .expect("TableViewState.init(): TableView header_grid can't be found.");

        // run first, to explicitly sets the column_count property
        self.create_column_headers(ctx);

        self.data_column_width = HashMap::new();
        self.header_column_width = HashMap::with_capacity(self.column_count);
        self.create_cells(ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let actions: Vec<TableAction> = self.actions.drain(..).collect();
        for action in actions {
            if let TableAction::Sort(column_id) = action {
                self.sort_rows(column_id, ctx);
            }
        }
        self.create_cells(ctx);
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        for index in ctx
            .widget()
            .get::<SelectedEntities>("selected_entities")
            .0
            .clone()
            .symmetric_difference(&self.selected_entities.borrow())
        {
            let mut cell = ctx.get_widget(*index);

            if !cell.get::<bool>("selected") {
                continue;
            }

            let selected = !cell.get::<bool>("selected");
            cell.set::<bool>("selected", selected);

            if selected {
                cell.get_mut::<Selector>("selector").push_state("selected");
            } else {
                cell.get_mut::<Selector>("selector")
                    .remove_state("selected");
            }
            cell.update(false);

            *self.selected_entities.borrow_mut() = ctx
                .widget()
                .get::<SelectedEntities>("selected_entities")
                .0
                .clone();
        }

        self.adjust_column_width(ctx);
    }
}

// [End] state: TableView

// [Start] view: TableView

widget!(
    /// The `TableView` is designed to visualize a collection of data
    /// that is broken into columns and rows.
    ///
    /// Columns are derived from fields of a data struct. Rows are the
    /// stuct instances. The TableView widget acts similar to the
    /// [`ListView`] widget.
    ///
    /// # Features
    ///
    /// * Atomatic creation of data columns.
    /// * Automatic adjustment of column width (taking the actual cell width).
    /// * Interactive selectable sort of data. The row contents is sorted in
    /// respect to the choosen column sort direction.
    ///
    /// # Examples
    ///
    /// To create a `TableView`, you need to define a data source that
    /// has a minimum column count of one. Column id's need to be
    /// unique. The `row_builder` closure will implement the needed
    /// properties. Please consult existing `table_view` example, or
    /// `showcase` example for further details.
    ///
    /// # Ownership
    ///
    /// The current architecture of orbtk, doesn't allow ownership of data for a
    /// given widget. The feature implementation relies on callbacks (`closures`).
    ///
    /// # Panics
    ///
    /// Runtime panics will occure, for the following cases:
    ///
    /// * Minimum of `one` column isn't met.
    /// * Property `id` of a column is empty.
    ///
    /// [`ListView`]: ./struct.ListView.html
    TableView<TableViewState> {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the column count property.
        column_count: usize,

        /// Sets or shares the widget entity that presets the
        /// data.
        data_source: u32,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// A boolean that triggers redrawing of items.
        request_update: bool,

        /// Sets or shares the row count property.
        /// Changing the value of this property will trigger a redraw.
        row_count: usize,

        /// Sets or shares the selection mode property.
        selection_mode: SelectionMode,

        /// Sets or shares the selected indices.
        selected_indices: SelectedIndices,

        /// Sets or shares the list of selected entities.
        selected_entities: SelectedEntities,

        /// Sets or shares the order of the sorting property.
        sort_direction: TableSortDirection,

        /// Sets or shares the sorting predicate property.
        sort_predicate: TableSortPredicate
    }
);

// associated functions for TableView
impl TableView {
    /// Adds a new column to the header of the TableView. The widget
    /// will create a `Button` that handles an on_click callback. The
    /// callback will trigger row sorting of the given column id.
    /// The style is **table_column_header**.
    ///
    /// # Arguments:
    /// * `title`: the title of the column. It will be displayed in the header of the table.
    /// * `column_id`: the unique id of the column.
    pub fn column<T: AsRef<str>>(mut self, title: T, column_id: T) -> Self {
        let title = title.as_ref().to_owned();
        let column_id = column_id.as_ref().to_owned();
        assert!(!column_id.is_empty(), "column_id can't be empty!");
        self.state
            .actions
            .push(TableAction::AddDefaultColumn(title, column_id));
        self
    }

    /// Creates a customizable column header.
    pub fn custom_column(mut self, header: Entity) -> Self {
        self.state
            .actions
            .push(TableAction::AddCustomColumn(header));
        self
    }

    /// A template that triggers the build of a row inside the
    /// `TableView`. If any of its properties (`row_count` or `request_update`)
    /// is changed, row state is set `dirty` and a redraw is
    /// triggered.
    /// TableView will draw entities pushed into the Vec, and maps its index to column index
    /// (e.g.: Entity pushed to Vec with index 0 will be mapped to column 0).
    ///
    /// # Arguments
    /// * `&mut BuildContext`: query widgets by its Entity.
    /// * `usize`: the current row index when TableView draws the rows.
    ///
    /// TableView will loop starting from 0 until it reaches the value
    /// `row_count`. You can use this index to query the container
    /// holding the data to be displayed.
    pub fn row_builder<F: Fn(&mut BuildContext, usize, &mut Vec<Entity>) + 'static>(
        mut self,
        builder: F,
    ) -> Self {
        self.state_mut().row_builder = Some(Box::new(builder));
        self
    }

    /// Defines the callback function for sorting rows of the `TableView`.
    /// Clicking on one of the TableView column headers will trigger
    /// sorting, and this callback will be used during sorting.
    ///
    /// # Arguments:
    /// * `&str:` the sorting predicate, e.g. the id of the column used to sort the TableView.
    /// * `TableSortDirection`: the current sorting order.
    /// * `Entity`: The entitiy of the widget containing the data that will be displayed.
    /// * `data_source` property value.
    ///
    /// Using the `context` in combination with the entity id will
    /// enable the query of other properties as well.
    pub fn on_sort<F: Fn(&str, TableSortDirection, Entity, &mut Context) + 'static>(
        mut self,
        sorter: F,
    ) -> Self {
        self.state_mut().sorter = Some(Box::new(sorter));
        self
    }
}

impl Template for TableView {
    fn template(self, _id: Entity, build_context: &mut BuildContext) -> Self {
        self.id(ID_TABLE_VIEW)
            .name(ID_TABLE_VIEW)
            .style("table_view")
            .background("transparent")
            .border_brush("transparent")
            .border_radius(0)
            .border_width(0)
            .column_count(0)
            .request_update(false)
            .row_count(0)
            .selection_mode(SelectionMode::Single)
            .selected_indices(HashSet::new())
            .selected_entities(HashSet::new())
            .child(
                Stack::new()
                    .id(ID_TABLE_VIEW_STACK)
                    .orientation("vertical")
                    .child(
                        Grid::new()
                            .id(ID_TABLE_VIEW_GRID_HEADER)
                            .columns("*")
                            .rows("36.0")
                            .build(build_context),
                    )
                    .child(
                        Grid::new()
                            .id(ID_TABLE_VIEW_GRID_DATA)
                            .columns("*")
                            .rows("*")
                            .build(build_context),
                    )
                    .build(build_context),
            )
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        TableViewLayout::new().into()
    }
}

// [End] view: TableView

// +-----------------------------------------------------------------+
// | ___________     ___.   .__         _________        .__  .__    |
// | \__    ___/____ \_ |__ |  |   ____ \_   ___ \  ____ |  | |  |   |
// |   |    |  \__  \ | __ \|  | _/ __ \/    \  \/_/ __ \|  | |  |   |
// |   |    |   / __ \| \_\ \  |_\  ___/\     \___\  ___/|  |_|  |__ |
// |   |____|  (____  /___  /____/\___  >\______  /\___  >____/____/ |
// |                \/    \/          \/        \/     \/            |
// +-----------------------------------------------------------------+

#[derive(Default, AsAny)]
struct TableCellState {
    request_selection_toggle: Cell<bool>,
}

/// Method definitions, that react on any given state change inside
/// the `TableCell` widget.
impl TableCellState {
    fn toggle_selection(&self) {
        self.request_selection_toggle.set(true);
    }
}

impl State for TableCellState {
    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !ctx.widget().get::<bool>("enabled") || self.request_selection_toggle.get() {
            self.request_selection_toggle.set(false);

            let selected = *ctx.widget().get::<bool>("selected");
            let cell = ctx.entity();
            let index = ctx.index_as_child(cell).unwrap();
            let table_view = Entity::from(ctx.widget().clone::<u32>("parent"));
            // do not confuse with the real parent: cells are attached to the DATA_GRID, not the TableView
            let mut parent = ctx.get_widget(table_view);
            let selection_mode = *parent.get::<SelectionMode>("selection_mode");

            // deselect currently selected cell
            if selected {
                parent
                    .get_mut::<SelectedEntities>("selected_entities")
                    .0
                    .remove(&cell);
                parent
                    .get_mut::<SelectedIndices>("selected_indices")
                    .0
                    .remove(&index);
                return;
            }

            if parent
                .get::<SelectedEntities>("selected_entities")
                .0
                .contains(&cell)
                || selection_mode == SelectionMode::None
            {
                return;
            }

            if selection_mode == SelectionMode::Single {
                parent
                    .get_mut::<SelectedEntities>("selected_entities")
                    .0
                    .clear();
                parent
                    .get_mut::<SelectedIndices>("selected_indices")
                    .0
                    .clear();
            }

            // update TableView list of selected entities and indices
            parent
                .get_mut::<SelectedEntities>("selected_entities")
                .0
                .insert(cell);
            parent
                .get_mut::<SelectedIndices>("selected_indices")
                .0
                .insert(index);

            let selected_indices: Vec<usize> = parent
                .get::<SelectedIndices>("selected_indices")
                .0
                .iter()
                .copied()
                .collect();

            ctx.event_adapter().push_event_direct(
                table_view,
                SelectionChangedEvent(table_view, selected_indices),
            );
        }
    }
}

widget!(
    /// Used to represent a cell inside the `TableView`.
    /// A cell determinse the position inside the table matrix. It wraps
    /// a widget when rendering rows via `TableView::row_builder` callback.
    TableCell<TableCellState>: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the column index of a cell
        /// used for placment inside the TableView.
        column_index: usize,

        /// Sets or shares the font property.
        font: String,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Indicates a hover event triggered by the mouse cursor.
        hover: bool,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the parent entity of a cell
        /// used for placement inside the TableView.
        parent: u32,

        /// Sets or shares the pressed property.
        /// Indicates that the widget was clicked by the mouse.
        pressed: bool,

        /// Sets or shares the row index of a cell inside
        /// the TableView.
        row_index: usize,

        /// Sets or shares the selected property.
        selected: bool
    }
);

impl Template for TableCell {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TableCell")
            .style("table_cell")
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .font("Roboto-Regular")
            .font_size(32.0)
            .foreground("transparent")
            .hover(false)
            .padding(0.0)
            .pressed(false)
            .selected(false)
            .on_click(move |states, _| {
                states.get::<TableCellState>(id).toggle_selection();
                false
            })
            .child(
                MouseBehavior::new()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .build(ctx),
            )
    }
}

// +------------------------------------------------------------------------------------------------------------------------------+
// | ___________     ___.   .__         _________        .__                         ___ ___                     .___             |
// | \__    ___/____ \_ |__ |  |   ____ \_   ___ \  ____ |  |  __ __  _____   ____  /   |   \   ____ _____     __| _/___________  |
// |   |    |  \__  \ | __ \|  | _/ __ \/    \  \/ /  _ \|  | |  |  \/     \ /    \/    ~    \_/ __ \\__  \   / __ |/ __ \_  __ \ |
// |   |    |   / __ \| \_\ \  |_\  ___/\     \___(  <_> )  |_|  |  /  Y Y  \   |  \    Y    /\  ___/ / __ \_/ /_/ \  ___/|  | \/ |
// |   |____|  (____  /___  /____/\___  >\______  /\____/|____/____/|__|_|  /___|  /\___|_  /  \___  >____  /\____ |\___  >__|    |
// |                \/    \/          \/        \/                        \/     \/       \/       \/     \/      \/    \/        |
// +------------------------------------------------------------------------------------------------------------------------------+

// [Start] state: TableViewColumnHeader

enum TableViewColumnHeaderAction {
    OnClick,
}

/// The `TableViewColumnHeaderState` handles the interaction and selection of `column headers`.
#[derive(Default, AsAny)]
struct TableViewColumnHeaderState {
    actions: Vec<TableViewColumnHeaderAction>,
}

/// The `TableViewColumnHeaderState` handles the selection of a `column headers`
/// as well as the associated interation methods.
impl TableViewColumnHeaderState {
    fn on_click(&mut self) {
        self.actions.push(TableViewColumnHeaderAction::OnClick);
    }
}

impl State for TableViewColumnHeaderState {
    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        for _ in &self.actions {
            // sends a message to the TableView to sort the table by the custom header's ID
            let table_view = Entity::from(ctx.widget().clone::<u32>("parent"));
            if let Some(header_widget) = ctx.try_child_from_index(0) {
                let column_id = header_widget.get::<String>("id");
                let message = TableAction::Sort(column_id.to_owned());
                ctx.send_message(message, table_view);
            }
        }
    }
}

// [End] state: TableViewColumnHeader

widget!(
    /// Represents a custom column header in a TableView.
    /// Wraps a widget by its entity. This is used to preserve header's
    /// on_click callback. The callback triggers a message to sort
    /// the row data.
    TableViewColumnHeader: MouseHandler {
        /// Sets or shares the entity of the TableView this column
        /// header is attached to.
        parent: u32
});

impl Template for TableViewColumnHeader {
    fn template(self, id: Entity, _ctx: &mut BuildContext) -> Self {
        self.name("TableViewColumnHeader").on_click(move |states, _| {
            states.get_mut::<TableViewColumnHeaderState>(id).on_click();
            false
        })
    }
}

// [End] view: TableViewColumnHeader
