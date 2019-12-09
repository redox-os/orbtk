use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

use super::{Layout, component, component_or_default, component_try_mut};

/// Orders its children in a grid layout with columns and rows. If now columns and rows are defined
/// the gird layout could also be used as alignment layout.
#[derive(Default)]
pub struct GridLayout {
    desired_size: RefCell<DirtySize>,
    children_sizes: RefCell<BTreeMap<Entity, (f64, f64)>>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl GridLayout {
    pub fn new() -> Self {
        GridLayout::default()
    }

    // calculates the available width for a column
    fn get_column_x_and_width(
        &self,
        columns_cache: &[(f64, f64)],
        entity: Entity,
        store: &mut StringComponentStore,
        grid_column: usize,
    ) -> (f64, f64) {
        let mut width = 0.0;
        let column = columns_cache.get(grid_column);

        let x = if let Some((x, _)) = column { *x } else { 0.0 };

        if let Ok(column_span) = store.get::<usize>("column_span", entity) {
            for column in columns_cache.iter().skip(grid_column).take(*column_span) {
                width += column.1;
            }
        } else if let Some((_, column_width)) = column {
            width = *column_width;
        }

        (x, width)
    }

    // calculates the available height for a row
    fn get_row_y_and_height(
        &self,
        rows_cache: &[(f64, f64)],
        entity: Entity,
        store: &mut StringComponentStore,
        grid_row: usize,
    ) -> (f64, f64) {
        let mut height = 0.0;
        let row = rows_cache.get(grid_row);

        let y = if let Some((y, _)) = row { *y } else { 0.0 };

        if let Ok(row_span) = store.get::<usize>("row_span", entity) {
            for row in rows_cache.iter().skip(grid_row).take(*row_span) {
                height += row.1;
            }
        } else if let Some((_, row_height)) = row {
            height = *row_height;
        }

        (y, height)
    }

    fn calculate_column_width(
        &self,
        child: Entity,
        column: Column,
        grid_column: usize,
        column_widths: &mut BTreeMap<usize, f64>,
        margin: Thickness,
    ) {
        if column.width != ColumnWidth::Auto {
            return;
        }
        let child_width = self.children_sizes.borrow().get(&child).unwrap().0;

        if let Some(width) = column_widths.get(&grid_column) {
            if *width < child_width + margin.left() + margin.right() {
                column_widths.insert(grid_column, child_width + margin.left() + margin.right());
            }
        } else {
            column_widths.insert(grid_column, child_width + margin.left() + margin.right());
        }
    }

    fn calculate_row_height(
        &self,
        child: Entity,
        row: Row,
        grid_row: usize,
        row_heights: &mut BTreeMap<usize, f64>,
        margin: Thickness,
    ) {
        if row.height != RowHeight::Auto {
            return;
        }

        let child_height = self.children_sizes.borrow().get(&child).unwrap().1;

        if let Some(height) = row_heights.get(&grid_row) {
            if *height < child_height + margin.top() + margin.bottom() {
                row_heights.insert(grid_row, child_height + margin.top() + margin.bottom());
            }
        } else {
            row_heights.insert(grid_row, child_height + margin.top() + margin.bottom());
        }
    }

    fn calculate_columns(
        &self,
        size: (f64, f64),
        columns_cache: &mut Vec<(f64, f64)>,
        columns: &mut Columns,
        column_widths: &BTreeMap<usize, f64>,
    ) {
        if !columns.is_empty() {
            // sets auto columns width to the width of the largest child
            for (grid_column, width) in column_widths {
                if let Some(column) = columns.get_mut(*grid_column) {
                    column.set_current_width(*width);
                }
            }

            // sets the width of columns with fixed width
            for column in columns.iter_mut() {
                if let ColumnWidth::Width(width) = column.width {
                    column.set_current_width(width);
                }
            }

            // calculates the width of the stretch columns
            let used_width: f64 = columns
                .iter()
                .filter(|column| column.width != ColumnWidth::Stretch)
                .map(|column| column.current_width())
                .sum();

            let stretch_width = ((size.0 - used_width)
                / columns
                    .iter()
                    .filter(|column| column.width == ColumnWidth::Stretch)
                    .count() as f64)
                .trunc();

            columns
                .iter_mut()
                .filter(|column| column.width == ColumnWidth::Stretch)
                .for_each(|column| column.set_current_width(stretch_width));

            let mut column_sum = 0.0;

            columns_cache.reserve(columns.len());
            for col in columns.iter() {
                columns_cache.push((column_sum, col.current_width()));
                column_sum += col.current_width();
            }

            // fix rounding gab
            if size.0 - column_sum > 0.0 {
                if let Some(last_column) = columns
                    .iter_mut()
                    .rev()
                    .find(|column| column.width == ColumnWidth::Stretch)
                {
                    last_column
                        .set_current_width(last_column.current_width() + size.0 - column_sum);
                }
            }
        }
    }

    fn calculate_rows(
        &self,
        size: (f64, f64),
        rows_cache: &mut Vec<(f64, f64)>,
        rows: &mut Rows,
        row_heights: &BTreeMap<usize, f64>,
    ) {
        if !rows.is_empty() {
            // sets auto rows height to the height of the largest child
            for (grid_row, height) in row_heights {
                if let Some(row) = rows.get_mut(*grid_row) {
                    row.set_current_height(*height);
                }
            }

            // sets the height of rows with fixed height
            for row in rows.iter_mut() {
                if let RowHeight::Height(height) = row.height {
                    row.set_current_height(height);
                }
            }

            // calculates the height of the stretch rows
            let used_height: f64 = rows
                .iter()
                .filter(|row| row.height != RowHeight::Stretch)
                .map(|row| row.current_height())
                .sum();

            let stretch_height = ((size.1 - used_height)
                / rows
                    .iter()
                    .filter(|row| row.height == RowHeight::Stretch)
                    .count() as f64)
                .trunc();

            rows.iter_mut()
                .filter(|row| row.height == RowHeight::Stretch)
                .for_each(|row| row.set_current_height(stretch_height));

            let mut row_sum = 0.0;

            rows_cache.reserve(rows.len());
            for col in rows.iter() {
                rows_cache.push((row_sum, col.current_height()));
                row_sum += col.current_height();
            }

            // fix rounding gab
            if size.1 - row_sum > 0.0 {
                if let Some(last_row) = rows
                    .iter_mut()
                    .rev()
                    .find(|row| row.height == RowHeight::Stretch)
                {
                    last_row.set_current_height(last_row.current_height() + size.1 - row_sum);
                }
            }
        }
    }
}

impl Layout for GridLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &ThemeValue,
    ) -> DirtySize {
         if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return *self.desired_size.borrow();
        }

        let horizontal_alignment: Alignment = component(ecm, entity, "horizontal_alignment");
        let vertical_alignment: Alignment = component(ecm, entity, "vertical_alignment");

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        self.children_sizes.borrow_mut().clear();
        let mut desired_size: (f64, f64) = (0.0, 0.0);

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];
            if let Some(child_layout) = layouts.get(&child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, child, ecm, layouts, theme);

                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);
                desired_size.0 = desired_size.0.max(child_desired_size.width());
                desired_size.1 = desired_size.1.max(child_desired_size.height());

                self.children_sizes.borrow_mut().insert(
                    child,
                    (child_desired_size.width(), child_desired_size.height()),
                );
            }
        }

        self.desired_size
            .borrow_mut()
            .set_size(desired_size.0, desired_size.1);

        let size = ecm
            .component_store()
            .get::<Constraint>("constraint", entity)
            .unwrap()
            .perform(self.desired_size.borrow().size());
        self.desired_size.borrow_mut().set_size(size.0, size.1);

        *self.desired_size.borrow()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
         if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return (0.0, 0.0);
        }

        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment: Alignment = component(ecm, entity, "horizontal_alignment");
        let vertical_alignment: Alignment =  component(ecm, entity, "vertical_alignment");
        let margin: Thickness = *ecm.component_store().get("margin", entity).unwrap();
        let constraint: Constraint = component(ecm, entity, "constraint");

        let size = constraint.perform((
            horizontal_alignment.align_measure(
                parent_size.0,
                self.desired_size.borrow().width(),
                margin.left(),
                margin.right(),
            ),
            vertical_alignment.align_measure(
                parent_size.1,
                self.desired_size.borrow().height(),
                margin.top(),
                margin.bottom(),
            ),
        ));

        let mut columns_cache = Vec::new();
        let mut column_widths = BTreeMap::new();
        let mut rows_cache = Vec::new();
        let mut row_heights = BTreeMap::new();

        // calculates the auto column widths

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            let margin: Thickness = *ecm.component_store().get("margin", child).unwrap();

            if let Ok(grid_column) = ecm.component_store().get::<usize>("column", child) {
                if let Ok(columns) = ecm.component_store().get::<Columns>("columns", entity) {
                    if let Some(column) = columns.get(*grid_column) {
                        self.calculate_column_width(
                            child,
                            *column,
                            *grid_column,
                            &mut column_widths,
                            margin,
                        );
                    }
                }
            }

            if let Ok(grid_row) = ecm.component_store().get::<usize>("row", child) {
                let grid_row = *grid_row;

                if let Ok(rows) = ecm.component_store().get::<Rows>("rows", entity) {
                    if let Some(row) = rows.get(grid_row) {
                        self.calculate_row_height(child, *row, grid_row, &mut row_heights, margin);
                    }
                }
            }
        }

        if let Ok(columns) = ecm
            .component_store_mut()
            .get_mut::<Columns>("columns", entity)
        {
            self.calculate_columns(size, &mut columns_cache, columns, &column_widths);
        }

        if let Ok(rows) = ecm.component_store_mut().get_mut::<Rows>("rows", entity) {
            self.calculate_rows(size, &mut rows_cache, rows, &row_heights);
        }

        if let Ok(bounds) = ecm
            .component_store_mut()
            .get_mut::<Rectangle>("bounds", entity)
        {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            let child_horizontal_alignment: Alignment = *ecm
                .component_store()
                .get("horizontal_alignment", child)
                .unwrap();
            let child_vertical_alignment: Alignment = *ecm
                .component_store()
                .get("vertical_alignment", child)
                .unwrap();
            let mut cell_position = (0.0, 0.0);
            let mut available_size = size;

            let has_columns =
                if let Ok(columns) = ecm.component_store().get::<Columns>("columns", entity) {
                    !columns.is_empty()
                } else {
                    false
                };

            // column
            if has_columns {
                let grid_column =
                    if let Ok(grid_column) = ecm.component_store().get::<usize>("column", child) {
                        *grid_column
                    } else {
                        0
                    };

                let (offset_x, available_width) = self.get_column_x_and_width(
                    &columns_cache,
                    child,
                    ecm.component_store_mut(),
                    grid_column,
                );

                cell_position.0 = offset_x;
                available_size.0 = available_width;
            } else {
                available_size.0 = size.0;
            }

            let has_rows = if let Ok(rows) = ecm.component_store().get::<Rows>("rows", entity) {
                !rows.is_empty()
            } else {
                false
            };

            // rows
            if has_rows {
                let grid_row =
                    if let Ok(grid_row) = ecm.component_store().get::<usize>("row", child) {
                        *grid_row
                    } else {
                        0
                    };

                let (offset_y, available_height) = self.get_row_y_and_height(
                    &rows_cache,
                    child,
                    ecm.component_store_mut(),
                    grid_row,
                );

                cell_position.1 = offset_y;
                available_size.1 = available_height;
            } else {
                available_size.1 = size.1;
            }

            let mut child_desired_size = (0.0, 0.0);
            if let Some(child_layout) = layouts.get(&child) {
                child_desired_size = child_layout.arrange(
                    render_context_2_d,
                    available_size,
                    child,
                    ecm,
                    layouts,
                    theme,
                );
            }

            let child_margin = {
                if child_desired_size.0 > 0.0 && child_desired_size.1 > 0.0 {
                    *ecm.component_store()
                        .get::<Thickness>("margin", child)
                        .unwrap()
                } else {
                    Thickness::default()
                }
            };

            if let Ok(child_bounds) = ecm
                .component_store_mut()
                .get_mut::<Rectangle>("bounds", child)
            {
                child_bounds.set_x(
                    cell_position.0
                        + child_horizontal_alignment.align_position(
                            available_size.0,
                            child_bounds.width(),
                            child_margin.left(),
                            child_margin.right(),
                        ),
                );
                child_bounds.set_y(
                    cell_position.1
                        + child_vertical_alignment.align_position(
                            available_size.1,
                            child_bounds.height(),
                            child_margin.top(),
                            child_margin.bottom(),
                        ),
                );
            }
        }

        if let Ok(bounds) = ecm
            .component_store_mut()
            .get_mut::<Rectangle>("bounds", entity)
        {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}

impl Into<Box<dyn Layout>> for GridLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
