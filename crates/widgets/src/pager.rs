use std::collections::VecDeque;

use crate::{api::prelude::*, proc_macros::*};

/// This state can be used to operate on the `Pager` widget inside of callbacks.
#[derive(Debug, Clone, PartialEq)]
enum PagerAction {
    Next,
    Previous,
    Navigate(usize),
    NavigateToCurrent,
    Remove(usize),
    Insert(usize, Entity),
}

#[derive(Default, Clone, Debug, AsAny)]
pub struct PagerState {
    current_index: usize,
    actions: VecDeque<PagerAction>,
}

impl PagerState {
    // used internal to navigate to the current index.
    fn navigate_to_current_index(&mut self) {
        self.actions.push_front(PagerAction::NavigateToCurrent);
    }

    /// Navigates to the next child. If the current child is the last in the list nothing will happen.
    pub fn next(&mut self) {
        self.actions.push_front(PagerAction::Next);
    }

    /// Navigates to the previous child. If the current child is the first in the list nothing will happen.
    pub fn previous(&mut self) {
        self.actions.push_front(PagerAction::Previous);
    }

    /// Navigates to the given index. Is the index out of bounds nothing will happen.
    pub fn navigate(&mut self, index: usize) {
        self.actions.push_front(PagerAction::Navigate(index));
    }

    /// Removes the child on the given index. If the index is out of bounds nothing will happen.
    pub fn remove(&mut self, index: usize) {
        self.actions.push_front(PagerAction::Remove(index));
    }

    /// Inserts a child on the given position.
    pub fn insert(&mut self, index: usize, entity: Entity) {
        self.actions.push_front(PagerAction::Insert(index, entity));
    }
}

impl State for PagerState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let current_index = Pager::correct_current_index(ctx);

        if let Some(count) = ctx.widget().children_count() {
            for i in 0..count {
                // make child on the current index always visible (todo: use current_index)
                if i == current_index {
                    if let Some(child) = &mut ctx.try_child_from_index(i) {
                        child.set("visibility", Visibility::Visible);
                    }

                    continue;
                }

                // collapse all other children
                if let Some(child) = &mut ctx.try_child_from_index(i) {
                    child.set("visibility", Visibility::Collapsed);
                }
            }
        }

        Pager::update_next_previous_enabled(ctx, current_index);
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
            match action {
                PagerAction::Next => Pager::next(ctx, ctx.entity()),
                PagerAction::Previous => Pager::previous(ctx, ctx.entity()),
                PagerAction::Navigate(index) => Pager::navigate(ctx, ctx.entity(), index),
                PagerAction::Remove(index) => Pager::remove(ctx, ctx.entity(), index),
                PagerAction::Push(entity) => Pager::push(ctx, ctx.entity(), entity),
                PagerAction::NavigateToCurrent => {
                    let current_index = Pager::correct_current_index(ctx);
                    Pager::navigate(ctx, ctx.entity(), current_index);
                }
            }
        }
    }
}

widget!(
    /// `Pager` is a navigation widget that provides a stack based navigation.
    ///
    /// # Example
    ///
    /// ```rust
    /// tbd
    /// ```
    Pager<PagerState> {
        /// Defines the index of the current shown child.
        current_index: usize,

        /// Used to check if  next can be executed. If the current visible child is the last child next can not be executed.
        next_enabled: bool,

        /// Used to check if a previous can be executed. If the current visible child is the first child previous can not be executed.
        previous_enabled: bool
    }
);

impl Pager {
    // checks and updates if next and previous can be executed.
    fn update_next_previous_enabled(ctx: &mut Context, current_index: usize) {
        if current_index == 0 {
            Pager::next_enabled_set(&mut ctx.widget(), false);
        } else {
            Pager::next_enabled_set(&mut ctx.widget(), true);
        }

        if let Some(count) = ctx.widget().children_count() {
            if current_index == count - 1 {
                Pager::previous_enabled_set(&mut ctx.widget(), false);
            } else {
                Pager::previous_enabled_set(&mut ctx.widget(), true);
            }
        }
    }

    // corrects the current_index to bounds and returns the corrected value.
    fn correct_current_index(ctx: &mut Context) -> usize {
        let mut current_index = *Pager::current_index_ref(&ctx.widget());
        if let Some(count) = ctx.widget().children_count() {
            if current_index >= count {
                current_index = count - 1;
            }
        }

        Pager::current_index_set(&mut ctx.widget(), current_index);

        current_index
    }

    /// Navigates to the next child. If the current child is the last in the list nothing will happen.
    pub fn next(ctx: &mut Context, entity: Entity) {
        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));
        Pager::navigate(ctx, entity, current_index + 1);
    }

    /// Navigates to the previous child. If the current child is the first in the list nothing will happen.
    pub fn previous(ctx: &mut Context, entity: Entity) {
        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));
        Pager::navigate(ctx, entity, current_index - 1);
    }

    /// Navigates to the given index. Is the index out of bounds nothing will happen.
    pub fn navigate(ctx: &mut Context, entity: Entity, index: usize) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        // if the index is the index of the current visible items it returns
        if let Some(child) = ctx.try_child_from_index(index) {
            if *child.get::<Visibility>("visibility") == Visibility::Visible {
                return;
            }
        }

        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));

        if let Some(count) = ctx.get_widget(entity).children_count() {
            if index >= count {
                return;
            }

            // hide the last visible child
            if let Some(count) = ctx.widget().children_count() {
                for i in 0..count {
                    if let Some(child) = &mut ctx.try_child_from_index(i) {
                        if *child.get::<Visibility>("visibility") != Visibility::Visible {
                            continue;
                        }
    
                        child.set("visibility", Visibility::Collapsed);
                    }
                }
            }
           
            if let Some(child) = &mut ctx.try_child_from_index(index) {
                child.set("visibility", Visibility::Visible);
            }

            Pager::current_index_set(&mut ctx.get_widget(entity), index);

            Pager::update_next_previous_enabled(ctx, index);
        }
    }

    /// Removes the child on the given index. If the index is out of bounds nothing will happen.
    pub fn remove(ctx: &mut Context, entity: Entity, index: usize) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));

        let child_entity = {
            if let Some(child) = ctx.try_child_from_index(index) {
                Some(child.entity())
            } else {
                None
            }
        };

        if let Some(child) = child_entity {
            ctx.remove_child_from(child, entity);
        }

        if let Some(count) = ctx.widget().children_count() {
            if (index == current_index && index == count - 1) || index < current_index {
                Pager::navigate(ctx, entity, current_index - 1);
                return;
            }
        }

        Pager::navigate(ctx, entity, current_index);
    }

    /// Inserts a child on the given position.
    pub fn insert(ctx: &mut Context, entity: Entity, index: usize, child: Entity) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));
    }

    // todo add push method
}

impl Template for Pager {
    fn template(self, _id: Entity, _context: &mut BuildContext) -> Self {
        self.name("Pager")
            .on_changed("current_index", |states, id| {
                states.get_mut::<PagerState>(id).navigate_to_current_index();
            })
    }
}

// todo crash on previous
// next_enabled does not work
// maybe check in state if internal change of current index