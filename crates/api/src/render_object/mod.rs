//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::{any::Any, cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{prelude::*, render::*, shell::WindowShell, utils::*};

pub use self::default::*;
pub use self::font_icon::*;
pub use self::image::*;
pub use self::rectangle::*;
pub use self::text::*;

mod default;
mod font_icon;
mod image;
mod rectangle;
mod text;

pub trait RenderObject: Any {
    fn render(
        &self,
        shell: &mut WindowShell<WindowAdapter>,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        render_objects: &Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: &Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
        theme: &ThemeValue,
        offsets: &mut BTreeMap<Entity, (f64, f64)>,
        debug: bool,
    ) {
        let mut global_position = Point::default();
        let root = ecm.entity_store().root;

        if let Some(parent) = ecm.entity_store().parent[&entity] {
            if let Some(offset) = offsets.get(&parent) {
                global_position = Point::new(offset.0, offset.1);
            }
        }

        if let Ok(visibility) = ecm.component_store().borrow_component::<Visibility>(entity) {
            if visibility.0 != VisibilityValue::Visible {
                return;
            }
        } else {
            return;
        }
        
        {
            let mut context = Context::new(
                entity,
                ecm,
                shell,
                &theme,
                render_objects.clone(),
                layouts.clone(),
                handlers.clone(),
                states.clone(),
            );

            self.render_self(&mut context, &global_position);
        }

        let mut global_pos = (0.0, 0.0);

        if let Ok(bounds) = ecm.component_store().borrow_component::<Bounds>(entity) {
            global_pos = (
                global_position.x + bounds.x(),
                global_position.y + bounds.y(),
            );
            offsets.insert(entity, global_pos);
        }

        if let Ok(g_pos) = ecm
            .component_store_mut()
            .borrow_mut_component::<Point>(entity)
        {
            g_pos.x = global_pos.0;
            g_pos.y = global_pos.1;
        }

        self.render_children(
            shell,
            entity,
            ecm,
            render_objects,
            layouts,
            handlers,
            states,
            theme,
            offsets,
            debug,
        );

        // render debug border for each widget
        if debug {
            if let Ok(bounds) = ecm.component_store().borrow_component::<Bounds>(entity) {
                let selector = Selector::from("debug-border");
                let brush = theme.brush("border-color", &selector.0).unwrap();
                shell.render_context_2_d().set_stroke_style(brush);
                shell.render_context_2_d().stroke_rect(
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                );
            }
        }
    }

    fn render_self(&self, _: &mut Context<'_>, _: &Point) {
        return;
    }

    fn render_children(
        &self,
        shell: &mut WindowShell<WindowAdapter>,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        render_objects: &Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: &Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: &Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
        theme: &ThemeValue,
        offsets: &mut BTreeMap<Entity, (f64, f64)>,
        debug: bool,
    ) {
        if ecm.entity_store().children[&entity].len() == 0 {
            return;
        }

        let mut index = 0;

        loop {
            let child = ecm.entity_store().children[&entity][index];

            if let Some(render_object) = render_objects.borrow().get(&child) {
                render_object.render(
                    shell,
                    child,
                    ecm,
                    render_objects,
                    layouts,
                    handlers,
                    states,
                    theme,
                    offsets,
                    debug,
                );
            }

            if index + 1 < ecm.entity_store().children[&entity].len() {
                index += 1;
            } else {
                break;
            }
        }
    }
}
