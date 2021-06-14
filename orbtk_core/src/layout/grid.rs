use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::*;

use crate::{
    prelude::*, proc_macros::IntoLayout, render::RenderContext2D, theming::*, tree::Tree,
    utils::prelude::*,
};

use super::{component, component_try_mut, Layout};

/// Orders its children in a grid layout with columns and rows. If no
/// columns and rows are defined the grid layout could also be used as
/// an alignment layout.
#[derive(Default, IntoLayout)]
pub struct GridLayout {
    desired_size: RefCell<DirtySize>,
    children_sizes: RefCell<BTreeMap<Entity, (f64, f64)>>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl GridLayout {
    /// Preset the defaults.
    pub fn new() -> Self {
        GridLayout::default()
    }

    // calculates the available width for a column
    fn get_column_x_and_width(
        &self,
        columns_cache: &[(f64, f64)],
        entity: Entity,
        store: &mut ComponentStore,
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
        store: &mut ComponentStore,
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

    fn calculate_block_size(
        &self,
        child: Entity,
        block: Block,
        grid_block: usize,
        block_sizes: &mut BTreeMap<usize, f64>,
        margin: Thickness,
        vertical: bool,
    ) {
        if block.size != BlockSize::Auto {
            return;
        }

        if vertical {
            let child_size = self.children_sizes.borrow().get(&child).unwrap().1;

            if let Some(size) = block_sizes.get(&grid_block) {
                if *size < child_size + margin.top() + margin.bottom() {
                    block_sizes.insert(grid_block, child_size + margin.top() + margin.bottom());
                }
            } else {
                block_sizes.insert(grid_block, child_size + margin.top() + margin.bottom());
            }
        } else {
            let child_size = self.children_sizes.borrow().get(&child).unwrap().0;

            if let Some(size) = block_sizes.get(&grid_block) {
                if *size < child_size + margin.left() + margin.right() {
                    block_sizes.insert(grid_block, child_size + margin.left() + margin.right());
                }
            } else {
                block_sizes.insert(grid_block, child_size + margin.left() + margin.right());
            }
        };
    }

    fn calculate_blocks(
        &self,
        size: f64,
        blocks_cache: &mut Vec<(f64, f64)>,
        blocks: &mut Blocks,
        block_sizes: &BTreeMap<usize, f64>,
    ) {
        if !blocks.is_empty() {
            // sets auto blocks size to the size of the largest child
            for (grid_block, size) in block_sizes {
                if let Some(block) = blocks.get_mut(*grid_block) {
                    block.set_current_size(*size);
                }
            }

            // sets the size of blocks with fixed size
            for block in blocks.iter_mut() {
                if let BlockSize::Size(size) = block.size {
                    block.set_current_size(size);
                }
            }

            // calculates the size of the stretch blocks
            let used_size: f64 = blocks
                .iter()
                .filter(|block| block.size != BlockSize::Stretch)
                .map(|block| block.current_size())
                .sum();

            let stretch_size = ((size - used_size)
                / blocks
                    .iter()
                    .filter(|block| block.size == BlockSize::Stretch)
                    .count() as f64)
                .trunc();

            blocks
                .iter_mut()
                .filter(|block| block.size == BlockSize::Stretch)
                .for_each(|block| block.set_current_size(stretch_size));

            let mut block_sum = 0.0;

            blocks_cache.reserve(blocks.len());
            for block in blocks.iter() {
                blocks_cache.push((block_sum, block.current_size()));
                block_sum += block.current_size();
            }

            // fix rounding gab
            if size - block_sum > 0.0 {
                if let Some(last_block) = blocks
                    .iter_mut()
                    .rev()
                    .find(|block| block.size == BlockSize::Stretch)
                {
                    last_block.set_current_size(last_block.current_size() + size - block_sum);
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
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> DirtySize {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return *self.desired_size.borrow();
        }

        let horizontal_alignment: Alignment = component(ecm, entity, "h_align");
        let vertical_alignment: Alignment = component(ecm, entity, "v_align");

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        self.children_sizes.borrow_mut().clear();

        let mut sum_row: BTreeMap<usize, f64> = BTreeMap::new();
        let mut sum_col: BTreeMap<usize, f64> = BTreeMap::new();
        let mut desired_size: (f64, f64) = (0.0, 0.0);

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];
            if let Some(child_layout) = layouts.get(&child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, child, ecm, layouts, theme);

                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);

                // If the child is a grid, add the greatest width per column into sum_col.
                if let Ok(grid_col) = ecm.component_store().get::<usize>("column", child) {
                    if let Some(current_width) = sum_col.get(grid_col) {
                        if current_width < &child_desired_size.width() {
                            sum_col.insert(*grid_col, child_desired_size.width());
                        }
                    } else {
                        sum_col.insert(*grid_col, child_desired_size.width());
                    }
                } else {
                    desired_size.0 = desired_size.0.max(child_desired_size.width());
                }
                // If the child is a grid, add the greatest height per row into sum_row.
                if let Ok(grid_row) = ecm.component_store().get::<usize>("row", child) {
                    if let Some(current_height) = sum_row.get(grid_row) {
                        if current_height < &child_desired_size.height() {
                            sum_row.insert(*grid_row, child_desired_size.height());
                        }
                    } else {
                        sum_row.insert(*grid_row, child_desired_size.height());
                    }
                } else {
                    desired_size.1 = desired_size.1.max(child_desired_size.height());
                }

                self.children_sizes.borrow_mut().insert(
                    child,
                    (child_desired_size.width(), child_desired_size.height()),
                );
            }
        }

        desired_size.0 = desired_size.0.max(sum_col.iter().map(|x| x.1).sum());
        desired_size.1 = desired_size.1.max(sum_row.iter().map(|x| x.1).sum());

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
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return (0.0, 0.0);
        }

        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment: Alignment = component(ecm, entity, "h_align");
        let vertical_alignment: Alignment = component(ecm, entity, "v_align");
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

            let margin: Thickness = component(ecm, entity, "margin");

            if let Ok(grid_column) = ecm.component_store().get::<usize>("column", child) {
                if let Ok(columns) = ecm.component_store().get::<Blocks>("columns", entity) {
                    if let Some(column) = columns.get(*grid_column) {
                        self.calculate_block_size(
                            child,
                            *column,
                            *grid_column,
                            &mut column_widths,
                            margin,
                            false,
                        );
                    }
                }
            }

            if let Ok(grid_row) = ecm.component_store().get::<usize>("row", child) {
                let grid_row = *grid_row;

                // calculate row_height for each row and insert into row_heights
                if let Ok(rows) = ecm.component_store().get::<Blocks>("rows", entity) {
                    if let Some(row) = rows.get(grid_row) {
                        self.calculate_block_size(
                            child,
                            *row,
                            grid_row,
                            &mut row_heights,
                            margin,
                            true,
                        );
                    }
                }
            }
        }
        if let Ok(columns) = ecm
            .component_store_mut()
            .get_mut::<Blocks>("columns", entity)
        {
            self.calculate_blocks(size.0, &mut columns_cache, columns, &column_widths);
        }

        // take row_heights and calculate rows_cache
        if let Ok(rows) = ecm.component_store_mut().get_mut::<Blocks>("rows", entity) {
            self.calculate_blocks(size.1, &mut rows_cache, rows, &row_heights);
        }

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            let child_horizontal_alignment: Alignment =
                *ecm.component_store().get("h_align", child).unwrap();
            let child_vertical_alignment: Alignment =
                *ecm.component_store().get("v_align", child).unwrap();
            let mut cell_position = (0.0, 0.0);
            let mut available_size = size;

            let has_columns =
                if let Ok(columns) = ecm.component_store().get::<Blocks>("columns", entity) {
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

            let has_rows = if let Ok(rows) = ecm.component_store().get::<Blocks>("rows", entity) {
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

            mark_as_dirty("bounds", child, ecm);
        }

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        // todo refactor the usage of mark_as_dirty on layouts
        mark_as_dirty("bounds", entity, ecm);

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}
