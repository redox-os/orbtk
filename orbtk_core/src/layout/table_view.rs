use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::*;

use crate::{
    prelude::*, proc_macros::IntoLayout, render::RenderContext2D, theming::*, tree::Tree,
    utils::prelude::*,
};

use super::{component, component_try_mut, try_component, Layout};

// Constants
//const ID_TABLE_VIEW: &str = "TableView";
//const ID_TABLE_VIEW_STACK: &str = "__STACK__";
//const ID_TABLE_VIEW_GRID_HEADER: &str = "__COLUMNS_HEADER__";
//const ID_TABLE_VIEW_GRID_DATA: &str = "__DATA_GRID__";
//const ICON_ORDER_ASCENDING: &str = "\u{e05e}";
//const ICON_ORDER_DESCENDING: &str = "\u{e068}";

/// Add padding to the widget.
#[derive(Default, IntoLayout)]
pub struct TableViewLayout {
    desired_size: RefCell<DirtySize>,
    //old_alignment: Cell<(Alignment, Alignment)>,
}

impl TableViewLayout {
    /// Preset the defaults.
    pub fn new() -> Self {
        TableViewLayout::default()
    }
}

impl Layout for TableViewLayout {
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

        #[cfg(feature = "debug")]
        println!("table_view entity: {:?}", entity);

        if let Some(column_count) = try_component::<usize>(ecm, entity, "column_count") {
            println!("table_view -> column_count: {:?}", column_count);
        } else {
            println!("table_view -> column_count: 'None'");
        }

        #[cfg(feature = "debug")]
        println!("table_view -> got {:?} children",ecm.entity_store().children[&entity].len() );

        let padding: Thickness = component(ecm, entity, "padding");

        // TableView -> Stack -> Grids
        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            #[cfg(feature = "debug")]
            println!("table_view child entity: {:?}", child);

            if let Some(child_layout) = layouts.get(&child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, child, ecm, layouts, theme);
                let mut desired_size = self.desired_size.borrow().size();

                // react, if childs dired_size has changed -> set dirty flag
                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);

                let child_margin = *ecm
                    .component_store()
                    .get::<Thickness>("margin", child)
                    .unwrap();

                 desired_size.0 = desired_size.0.max(
                     child_desired_size.width()
                         + padding.left()
                         + padding.right()
                         + child_margin.left()
                         + child_margin.right(),
                 );

                 desired_size.1 = desired_size.1.max(
                     child_desired_size.height()
                         + padding.top()
                         + padding.bottom()
                         + child_margin.top()
                         + child_margin.left(),
                 );

                 self.desired_size
                     .borrow_mut()
                     .set_size(desired_size.0, desired_size.1);
             }
        }

        // // synchronize width of header and data columns
        // // if they defere, aline to max value from header or data
        // if let Some(header_grid) = try_component::<u32>(ecm, entity, ID_TABLE_VIEW_GRID_HEADER) {
        //     let header_grid_bounds = component::<Rectangle>(ecm, header_grid.into(), "bounds");
        //     component_try_mut::<Constraint>(ecm, entity, "constraint")
        //         .unwrap()
        //         .set_width(header_grid_bounds.width());
        //     let column_count = component::<usize>(ecm, header_grid.into(), "column_count");
        //     println!("table_view -> column_count: {:?}", column_count);
        //     println!("table_view -> entity header_grid: {:?}", header_grid);
        // }
        // if let Some(data_grid) = try_component::<u32>(ecm, entity, ID_TABLE_VIEW_GRID_DATA) {
        //     let data_grid_bounds = component::<Rectangle>(ecm, data_grid.into(), "bounds");
        //     component_try_mut::<Constraint>(ecm, entity, "constraint")
        //         .unwrap()
        //         .set_width(data_grid_bounds.width());
        //     println!("table_view -> entity data_grid: {:?}", data_grid);
        // }

        // // let constraint: Constraint = component(ecm, entity, "constraint");
        // // if constraint.width() > 0.0 {
        // //     self.desired_size.borrow_mut().set_width(constraint.width());
        // // }

        // // if constraint.height() > 0.0 {
        // //     self.desired_size
        // //         .borrow_mut()
        // //         .set_height(constraint.height());
        // // }

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
        let margin = *ecm
            .component_store()
            .get::<Thickness>("margin", entity)
            .unwrap();
        let padding: Thickness = component(ecm, entity, "padding");
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

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        mark_as_dirty("bounds", entity, ecm);

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            let child_margin: Thickness = component(ecm, entity, "margin");

            if let Some(child_layout) = layouts.get(&child) {
                child_layout.arrange(
                    render_context_2_d,
                    size,
                    child,
                    ecm,
                    layouts,
                    theme,
                );
            }

            let child_horizontal_alignment: Alignment =
                *ecm.component_store().get("h_align", child).unwrap();
            let child_vertical_alignment: Alignment =
                *ecm.component_store().get("v_align", child).unwrap();
            if let Ok(child_bounds) = ecm
                .component_store_mut()
                .get_mut::<Rectangle>("bounds", child)
            {
                child_bounds.set_x(
                    padding.left()
                        + child_horizontal_alignment.align_position(
                            size.0,
                            child_bounds.width(),
                            child_margin.left(),
                            child_margin.right(),
                        ),
                );
                child_bounds.set_y(
                    padding.top()
                        + child_vertical_alignment.align_position(
                            size.1,
                            child_bounds.height(),
                            child_margin.top(),
                            child_margin.bottom(),
                        ),
                );
            }

            mark_as_dirty("bounds", child, ecm);
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
   }
}
