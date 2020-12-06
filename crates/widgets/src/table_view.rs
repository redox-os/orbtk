use crate::{api::prelude::*, prelude::*, proc_macros::*};

const HEADER_GRID: &str = "__COLUMNS_HEADER__";
const DATA_GRID: &str = "__DATA_GRID__";
const ORDER_ASC_ICON: &str = "\u{e05e}";
const ORDER_DESC_ICON: &str = "\u{e068}";

type RowBuilder = Option<Box<dyn Fn(&mut BuildContext, usize, &mut Vec<Entity>)>>;
type RowSorter = Option<Box<dyn Fn(&str, TableSortDirection, Entity, &mut Context)>>;
enum TableAction {
    AddDefaultColumn(String, String),
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
    /// From lower values to higher.Lowest value will be placed on the top of the table,
    /// and the highest on the bottom of the table.
    Ascending,
    /// From higher values to lower.The highest value will be placed on the top of the table,
    /// and the lowest on the bottom of the table.
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

#[derive(Default, AsAny)]
struct TableState {
    actions: Vec<TableAction>,
    column_count: usize,
    data_grid: Entity,
    data_column_widths: HashMap<usize, f64>,
    header_column_widths: HashMap<usize, f64>,
    header_grid: Entity,
    request_update: bool,
    row_builder: RowBuilder,
    row_count: usize,
    sorted_column_header: Option<Entity>,
    sorter: RowSorter,
}

impl TableState {
    fn adjust_columns(&self, ctx: &mut Context) {
        // Header and Data grid starts with no columns: adjust column count to real one
        let adjusted_columns = Columns::create().repeat("auto", self.column_count).build();
        let cloned_columns = adjusted_columns.clone();
        ctx.get_widget(self.header_grid)
            .set::<Columns>("columns", adjusted_columns);
        ctx.get_widget(self.data_grid)
            .set::<Columns>("columns", cloned_columns);
        ctx.widget().set::<usize>("column_count", self.column_count);
    }

    fn adjust_column_widths(&mut self, ctx: &mut Context) {
        let table_view = ctx.entity();

        // measure header column widths
        let mut index = 0;
        ctx.change_into(self.header_grid);
        while let Some(header) = ctx.try_child_from_index(index) {
            // TODO: querying constraint always returns 0.0
            // Using the bounds of the header instead of
            let bounds = header.get::<Rectangle>("bounds");
            let header_width = bounds.width();
            let column_index = header.get::<usize>("column");
            self.header_column_widths
                .insert(*column_index, header_width);
            index += 1;
        }

        ctx.change_into(table_view);

        // get data grid wildest width by columns: values already computed by the GridLayout since its ColumnWidth is auto.
        let data_grid = ctx.get_widget(self.data_grid);
        let data_columns_widths = data_grid.get::<Columns>("columns");
        for i in 0..data_columns_widths.len() {
            if let Some(column) = data_columns_widths.get(i) {
                let data_w = column.current_width();
                self.data_column_widths.insert(i, data_w);
            }
        }

        // comparing header and data grid column widths
        for i in 0..self.header_column_widths.len() {
            let header_width = *self.header_column_widths.get(&i).unwrap();
            let data_width = *self.data_column_widths.get(&i).unwrap();

            if header_width < data_width {
                ctx.change_into(self.header_grid);
                if let Some(mut header) = ctx.try_child_from_index(i) {
                    if let Some(new_width) = self.data_column_widths.get(&i) {
                        header
                            .get_mut::<Constraint>("constraint")
                            .set_width(*new_width);
                    }
                }
            } else if header_width > data_width {
                // change data grid children column widths to match the column header width
                ctx.change_into(self.data_grid);
                let mut idx = 0;
                while let Some(mut child) = ctx.try_child_from_index(idx) {
                    if *child.get::<usize>("column") == i {
                        if let Some(new_width) = self.header_column_widths.get(&i) {
                            child
                                .get_mut::<Constraint>("constraint")
                                .set_width(*new_width);
                        }
                    }
                    idx += 1;
                }
            }
        }

        self.header_column_widths.clear();
        self.data_column_widths.clear();
    }

    fn adjust_rows(&self, row_count: usize, ctx: &mut Context) {
        // Set data grid row count to real one
        let new_rows = Rows::create().repeat("auto", row_count).build();
        ctx.get_widget(self.data_grid).set::<Rows>("rows", new_rows);
    }

    fn generate_column_headers(&mut self, ctx: &mut Context) {
        for action in &self.actions {
            if let TableAction::AddDefaultColumn(title, column_id) = action {
                let table_view = ctx.entity();
                let c_id = column_id.clone();
                let build_context = &mut ctx.build_context();
                let header = Button::new()
                    .id(column_id.clone())
                    .icon_brush("#000000")
                    .style("table_column_header")
                    .text(title.clone())
                    .on_click(move |states, _| -> bool {
                        states
                            .get_mut::<TableState>(table_view)
                            .actions
                            .push(TableAction::Sort(c_id.clone()));
                        false
                    })
                    .build(build_context);

                build_context.register_property::<usize>("column", header, self.column_count);
                build_context.register_property::<usize>("row", header, 0);
                build_context.append_child(self.header_grid, header);
                self.column_count += 1;
            }
        }
        assert!(
            self.column_count > 0,
            "You must define at least one column on a TableView!"
        );
        self.actions.clear();
        self.adjust_columns(ctx);
    }

    fn generate_cells(&mut self, ctx: &mut Context) {
        let actual_row_count = ctx.widget().clone::<usize>("row_count");
        let should_update = ctx.widget().clone::<bool>("request_update");

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
                        build_context.register_property::<usize>("row", *child, i);
                        build_context.register_property::<usize>("column", *child, j);
                        build_context.append_child(self.data_grid, *child);
                    }

                    cells_of_row.clear();
                }

                self.row_count = actual_row_count;
            }
        }
    }

    fn set_sort_direction_icon(
        &mut self,
        column_id: &str,
        order: TableSortDirection,
        ctx: &mut Context,
    ) {
        let table_view = ctx.entity();
        let arrow = match order {
            TableSortDirection::Ascending => String::from(ORDER_ASC_ICON),
            TableSortDirection::Descending => String::from(ORDER_DESC_ICON),
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

impl State for TableState {
    fn init(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        self.data_grid = ctx
            .entity_of_child(DATA_GRID)
            .expect("TableState.init(): Table data grid could not be found.");
        self.header_grid = ctx
            .entity_of_child(HEADER_GRID)
            .expect("TableState.init(): Table column header grid could not be found.");

        // must be come first because it is explicitly sets the column_count property
        self.generate_column_headers(ctx);

        self.data_column_widths = HashMap::new();
        self.header_column_widths = HashMap::with_capacity(self.column_count);
        self.generate_cells(ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let actions: Vec<TableAction> = self.actions.drain(..).collect();
        for action in actions {
            if let TableAction::Sort(column_id) = action {
                self.sort_rows(column_id, ctx);
            }
        }
        self.generate_cells(ctx);
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.adjust_column_widths(ctx);
    }
}

widget!(
    TableView<TableState> {
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

        /// Sets or shares the Entity of the widget is holding the data to display
        data_source: u32,

        /// Set it tro `true` to trigger redrawing of the items.
        request_update: bool,

        /// Sets or shares the row count property.
        /// Changing the value of this property will trigger a redraw.
        row_count: usize,

        /// Sets or shares the order of the sorting property.
        sort_direction: TableSortDirection,

        /// Sets or shares the sorting predicate property.
        sort_predicate: TableSortPredicate
    }
);

impl Template for TableView {
    fn template(self, _id: Entity, build_context: &mut BuildContext) -> Self {
        self.name("TableView")
            .style("table_view")
            .background("transparent")
            .border_brush("transparent")
            .border_radius(0)
            .border_width(0)
            .column_count(0)
            .request_update(false)
            .row_count(0)
            .child(
                Stack::new()
                    .orientation("vertical")
                    .child(
                        Grid::new()
                            .id(HEADER_GRID)
                            .columns(Columns::create().build())
                            .rows(Rows::create().push(36.0).build())
                            .build(build_context),
                    )
                    .child(
                        Grid::new()
                            .id(DATA_GRID)
                            .columns(Columns::create().build())
                            .rows(Rows::create().build())
                            .build(build_context),
                    )
                    .build(build_context),
            )
    }
}

impl TableView {
    /// Adds a new column to the header of the TableView.
    /// The widget will be a Button with an on_click callback triggering a sorting of rows.
    /// The style is **table_column_header**.
    ///
    /// # Arguments:
    /// * `title`: the title of the column. It will be displayed in the header of the table.
    /// * `column_id`: the unique id of the column.
    pub fn column<T: AsRef<str>>(mut self, title: T, column_id: T) -> Self {
        let title = title.as_ref().to_owned();
        let column_id = column_id.as_ref().to_owned();
        assert!(!column_id.is_empty(), "column_id must be not empty!");
        self.state
            .actions
            .push(TableAction::AddDefaultColumn(title, column_id));
        self
    }

    /// Defines the template build function for a row of the `TableView`.
    /// TableView will call this function when redrawing is triggered by
    /// changing the `row_count` or `request_update` property.
    pub fn row_builder<F: Fn(&mut BuildContext, usize, &mut Vec<Entity>) + 'static>(
        mut self,
        builder: F,
    ) -> Self {
        self.state_mut().row_builder = Some(Box::new(builder));
        self
    }

    /// Defines the callback function for sorting rows of the `TableView`.
    /// Clicking on one of the TableView column headers will trigger sorting, and this callback
    /// will be used during sorting.
    ///
    /// # Arguments:
    /// * `&str:` the sorting predicate, e.g. the id of the column the TableView is sorted by.
    /// * `TableSortDirection`: the current order of the sorting.
    /// * `Entity`: The entitiy of the widget which contains the data to display.The value of the `data_source` property.
    ///  It could be used together with the `Context` to query properties.
    pub fn on_sort<F: Fn(&str, TableSortDirection, Entity, &mut Context) + 'static>(
        mut self,
        sorter: F,
    ) -> Self {
        self.state_mut().sorter = Some(Box::new(sorter));
        self
    }
}
