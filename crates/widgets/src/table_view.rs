use crate::{api::prelude::*, prelude::*, proc_macros::*};

const HEADER_GRID: &str = "__COLUMNS_HEADER__";
const DATA_GRID: &str = "__DATA_GRID__";

type RowBuilder = Option<Box<dyn Fn(&mut BuildContext, usize, &mut Vec<Entity>)>>;

enum TableAction {
    AddDefaultColumn(Entity),
}

#[derive(Default, AsAny)]
struct TableState {
    actions: Vec<TableAction>,
    column_count: usize,
    data_grid: Entity,
    header_grid: Entity,
    row_builder: RowBuilder,
    row_count: usize,
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

    fn adjust_rows(&self, row_count: usize, ctx: &mut Context) {
        // Set data grid row count to real one
        let new_rows = Rows::create().repeat("auto", row_count).build();
        ctx.get_widget(self.data_grid).set::<Rows>("rows", new_rows);
    }

    fn generate_column_headers(&mut self, ctx: &mut Context) {
        let build_context = &mut ctx.build_context();

        for action in &self.actions {
            match action {
                TableAction::AddDefaultColumn(header) => {
                    build_context.register_property::<usize>("column", *header, self.column_count);
                    build_context.register_property::<usize>("row", *header, 0);
                    build_context.append_child(self.header_grid, *header);
                    self.column_count += 1;
                }
            }
        }

        self.adjust_columns(ctx);
    }

    fn generate_cells(&mut self, ctx: &mut Context) {
        let actual_row_count = ctx.widget().clone::<usize>("row_count");

        if actual_row_count != self.row_count || *ctx.widget().get::<bool>("request_update") {
            if let Some(row_builder) = &self.row_builder {
                ctx.widget().set::<bool>("request_update", false);
                self.adjust_rows(actual_row_count, ctx);

                // Probably the fastest method currently: only one allocation per update()
                // TO-DO: implement maybe a TableRowMapper ?
                let mut cells_of_row = Vec::with_capacity(actual_row_count);
                ctx.clear_children_of(self.data_grid);
                let build_context = &mut ctx.build_context();

                for i in 0..actual_row_count {
                    row_builder(build_context, i, &mut cells_of_row);

                    for (j, child) in cells_of_row.iter().enumerate() {
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
        self.generate_cells(ctx);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.generate_cells(ctx)
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

        /// Set it tro `true` to trigger redrawing of the items.
        request_update: bool,

        /// Sets or shares the row count property.
        row_count: usize
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
    /// The widget will be a Button with style **table_column_header**.
    ///
    /// # Arguments:
    /// * `title`: the title of the column. It will be displayed in the header of the table.
    /// * `build_context`: the BuildContext used to build the header widget
    pub fn column<T: Into<String> + Copy>(
        mut self,
        title: T,
        build_context: &mut BuildContext,
    ) -> Self {
        self.state.actions.push(TableAction::AddDefaultColumn(
            Button::new()
                .style("table_column_header")
                .text(title.into())
                .build(build_context),
        ));
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
}
