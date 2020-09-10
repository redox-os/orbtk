use std::collections::VecDeque;

use crate::{api::prelude::*, prelude::*, proc_macros::*, theme::prelude::*};

#[derive(Debug, Clone, PartialEq)]
enum PagerAction {
    Next,
    Previous,
    Navigate(usize),
    Remove(usize),
    Insert(usize, Entity),
}

#[derive(Default, Clone, Debug, AsAny)]
pub struct PagerState {
    current_index: usize,
    actions: VecDeque<PagerAction>,
}

impl PagerState {
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
        // todo check if current_index out of bounds and correct

        if let Some(count) = ctx.widget().children_count() {
            for i in 0..count {
                // make child on the current index always visible (todo: use current_index)
                if i == 0 {
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
       

            // todo initial next previous enabled
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
            match action {
                PagerAction::Next => Pager::next(ctx, ctx.entity),
                PagerAction::Previous => Pager::previous(ctx, ctx.entity),
                PagerAction::Navigate(index) => Pager::navigate(ctx, ctx.entity, index),
                PagerAction::Remove(index) => Pager::remove(ctx, ctx.entity, index),
                PagerAction::Insert(index, entity) => {
                    Pager::insert(ctx, ctx.entity, index, entity)
                }
            }
        }
    }
}

widget!(
    Pager<PagerState> {
        current_index: usize
    }
);

impl Pager {
    /// Navigates to the next child. If the current child is the last in the list nothing will happen.
    pub fn next(ctx: &mut Context, entity: Entity) {
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));
        let next_index = current_index + 1;
       
        if let Some(count) = ctx.get_widget(entity).children_count() {
            if next_index >= count {
                return;
            }

            // todo next, previous enabled flags or methods
            if let Some(child) = &mut ctx.try_child_from_index(current_index) {
                child.set("visibility", Visibility::Collapsed);
            }

            if let Some(child) = &mut ctx.try_child_from_index(next_index) {
                child.set("visibility", Visibility::Visible);
            }

            Pager::current_index_set(&mut ctx.get_widget(entity), next_index);

            // next index add end set next enabled to false
        }
    }

    /// Navigates to the previous child. If the current child is the first in the list nothing will happen.
    pub fn previous(ctx: &mut Context, entity: Entity) {
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));

        if current_index == 0 {
            return;
        }

        Pager::current_index_set(&mut ctx.get_widget(entity), current_index - 1);
    }

    /// Navigates to the given index. Is the index out of bounds nothing will happen.
    pub fn navigate(ctx: &mut Context, entity: Entity, index: usize) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));
    }

    /// Removes the child on the given index. If the index is out of bounds nothing will happen.
    pub fn remove(ctx: &mut Context, entity: Entity, index: usize) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));
    }

    /// Inserts a child on the given position.
    pub fn insert(ctx: &mut Context, entity: Entity, index: usize, child: Entity) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));
    }
}

impl Template for Pager {
    fn template(self, _id: Entity, _context: &mut BuildContext) -> Self {
        self.name("Pager")

        // todo current index changed => navigate on_current_index changed method on state
    }
}
