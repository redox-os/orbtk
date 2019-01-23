use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::{
        Bounds, Column, ColumnWidth, Columns, Constraint, GridColumn, HorizontalAlignment, Margin,
        VerticalAlignment,
    },
    structs::{Position, Size, Spacer},
    LayoutResult,
};

#[derive(Default)]
pub struct GridLayout {
    current_child: Cell<usize>,
    columns_cache: RefCell<BTreeMap<usize, (f64, f64)>>,
}

impl Into<Box<dyn Layout>> for GridLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for GridLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        parent_constraint: &Constraint,
        children: &[Entity],
        child_size: Option<(f64, f64)>,
    ) -> LayoutResult {
        let mut constraint = {
            if let Ok(constraint) = ecm.borrow_component::<Constraint>(entity) {
                *constraint
            } else {
                Constraint::default()
            }
        };

        let margin = {
            if let Ok(margin) = ecm.borrow_component::<Margin>(entity) {
                *margin
            } else {
                Margin::default()
            }
        };

        let vertical_alignment = get_vertical_alignment(entity, ecm);
        let horizontal_alignment = get_horizontal_alignment(entity, ecm);

        // size is independent from the children
        let size = constraint.perform((
            horizontal_alignment.align_width(parent_constraint.width(), constraint.width(), margin),
            vertical_alignment.align_height(
                parent_constraint.height(),
                constraint.height(),
                margin,
            ),
        ));

        constraint.set_width(size.0);
        constraint.set_height(size.1);

        // todo: this is complete wip, column and rows not implemented at the moment
        if let Some(child_size) = child_size {
            let child = children[self.current_child.get()];

            // child margin
            let c_margin = {
                if let Ok(margin) = ecm.borrow_component::<Margin>(child) {
                    *margin
                } else {
                    Margin::default()
                }
            };

            let mut non_rows_and_columns = true;

            if let Ok(columns) = ecm.borrow_component::<Columns>(entity) {
                non_rows_and_columns = columns.len() == 0;
            }

            let c_vertical_alignment = get_vertical_alignment(child, ecm);
            let c_horizontal_alignment = get_horizontal_alignment(child, ecm);

            if non_rows_and_columns {
                // center child if now row and columns are set
                if let Ok(c_bounds) = ecm.borrow_mut_component::<Bounds>(child) {
                    c_bounds.set_x(c_horizontal_alignment.align_x(size.0, child_size.0, c_margin));
                    c_bounds.set_y(c_vertical_alignment.align_y(size.1, child_size.1, c_margin));
                }
            } else {
                let grid_column = if let Ok(grid_column) = ecm.borrow_component::<GridColumn>(child)
                {
                    grid_column.0
                } else {
                    0
                };

                let (offset_x, available_width) =
                    if let Some((x, width)) = self.columns_cache.borrow().get(&grid_column) {
                        (*x, *width)
                    } else {
                        (0.0, size.0)
                    };

                if let Ok(c_bounds) = ecm.borrow_mut_component::<Bounds>(child) {
                    c_bounds.set_x(
                        offset_x + c_horizontal_alignment.align_x(size.0, child_size.0, c_margin),
                    );
                    c_bounds.set_width(c_horizontal_alignment.align_width(
                        available_width,
                        child_size.0,
                        c_margin,
                    ));

                    // todo rows
                    c_bounds.set_y(c_vertical_alignment.align_y(size.1, child_size.1, c_margin));
                }
            }

            self.current_child.set(self.current_child.get() + 1);

            if self.current_child.get() < children.len() {
                return LayoutResult::RequestChild(children[self.current_child.get()], constraint);
            }

            return LayoutResult::Size(size);
        } else {
            if children.is_empty() {
                return LayoutResult::Size(size);
            }

            self.current_child.set(0);
            self.columns_cache.borrow_mut().clear();
        }

        let mut column_widths = BTreeMap::new();

        // calculates the column and row sizes only by the first child
        if self.current_child.get() == 0 {
            // todo clear column widths after size is changed

            // calculates the auto column widths
            for child in children {
                if let Ok(grid_column) = ecm.borrow_component::<GridColumn>(*child) {
                    if let Ok(constraint) = ecm.borrow_component::<Constraint>(*child) {
                        if let Ok(columns) = ecm.borrow_component::<Columns>(entity) {
                            if let Some(column) = columns.get(grid_column.0) {
                                if column.width == ColumnWidth::Auto {
                                    if column.current_width() < constraint.width() {
                                        column_widths.insert(grid_column.0, constraint.width());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Ok(columns) = ecm.borrow_mut_component::<Columns>(entity) {
                if columns.len() > 0 {
                    // sets auto columns width to the width of the largest child
                    for (grid_column, width) in column_widths {
                        if let Some(column) = columns.get_mut(grid_column) {
                            column.set_current_width(width);
                        }
                    }

                    // sets the width column widths
                    columns
                        .iter_mut()
                        .filter(|column| {
                            column.width != ColumnWidth::Auto
                                && column.width != ColumnWidth::Stretch
                        })
                        .for_each(|column| match column.width {
                            ColumnWidth::Width(width) => {
                                column.set_current_width(width);
                            }
                            _ => {}
                        });

                    // calculates the width of the stretch columns
                    let remaining_width: f64 = columns
                        .iter()
                        .filter(|column| column.width != ColumnWidth::Stretch)
                        .map(|column| column.current_width())
                        .sum();

                    let stretch_width = remaining_width
                        / columns
                            .iter()
                            .filter(|column| column.width == ColumnWidth::Stretch)
                            .count() as f64;

                    columns
                        .iter_mut()
                        .filter(|column| column.width == ColumnWidth::Stretch)
                        .for_each(|column| match column.width {
                            ColumnWidth::Stretch => {
                                column.set_current_width(stretch_width);
                            }
                            _ => {}
                        });

                    let mut column_sum = 0.0;

                    for i in 0..columns.len() {
                        self.columns_cache
                            .borrow_mut()
                            .insert(i, (column_sum, columns.get(i).unwrap().current_width()));
                        column_sum += columns.get(i).unwrap().current_width();
                    }

                    println!("cw: {}", stretch_width);
                }
            }
        }

        LayoutResult::RequestChild(children[self.current_child.get()], constraint)
    }
}

// --- helpers ---

fn get_vertical_alignment(entity: Entity, ecm: &EntityComponentManager) -> VerticalAlignment {
    if let Ok(vertical_alignment) = ecm.borrow_component::<VerticalAlignment>(entity) {
        return *vertical_alignment;
    }

    VerticalAlignment::default()
}

fn get_horizontal_alignment(entity: Entity, ecm: &EntityComponentManager) -> HorizontalAlignment {
    if let Ok(horizontal_alignment) = ecm.borrow_component::<HorizontalAlignment>(entity) {
        return *horizontal_alignment;
    }

    HorizontalAlignment::default()
}

fn get_column_index(entity: Entity, ecm: &EntityComponentManager) -> usize {
    if let Ok(column) = ecm.borrow_component::<GridColumn>(entity) {
        return column.0;
    }

    0
}

// --- helpers ---
