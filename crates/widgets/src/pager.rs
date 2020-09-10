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
    stack: Vec<Entity>,
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
        for child in self.stack.pop() {
            ctx.append_child_entity(child);
        }
    }

    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.actions.pop_front() {
            match action {
                PagerAction::Next => Pager::next(&mut ctx.widget()),
                PagerAction::Previous => Pager::previous(&mut ctx.widget()),
                PagerAction::Navigate(index) => Pager::navigate(&mut ctx.widget(), index),
                PagerAction::Remove(index) => Pager::remove(&mut ctx.widget(), index),
                PagerAction::Insert(index, entity) => {
                    Pager::insert(&mut ctx.widget(), index, entity)
                }
            }
        }
    }
}

widget!(
    Pager<PagerState> {
        cached: bool,
        current_index: usize
    }
);

impl Pager {
    /// Appends an widget as entity to the navigation stack.
    pub fn push<W: Widget>(mut self, child: Entity) -> Self {
        self.state_mut().stack.push(child);
        self
    }

    /// Navigates to the next child. If the current child is the last in the list nothing will happen.
    pub fn next(ctx: &mut WidgetContainer) {
        Pager::panics_on_wrong_type(ctx);

        let current_index = *Pager::current_index_ref(ctx);
    }

    /// Navigates to the previous child. If the current child is the first in the list nothing will happen.
    pub fn previous(ctx: &mut WidgetContainer) {
        Pager::panics_on_wrong_type(ctx);
    }

    /// Navigates to the given index. Is the index out of bounds nothing will happen.
    pub fn navigate(ctx: &mut WidgetContainer, index: usize) {
        Pager::panics_on_wrong_type(ctx);
    }

    /// Removes the child on the given index. If the index is out of bounds nothing will happen.
    pub fn remove(ctx: &mut WidgetContainer, index: usize) {
        Pager::panics_on_wrong_type(ctx);
    }

    /// Inserts a child on the given position.
    pub fn insert(ctx: &mut WidgetContainer, index: usize, entity: Entity) {
        Pager::panics_on_wrong_type(ctx);
    }
}

impl Template for Pager {
    fn template(self, _id: Entity, _context: &mut BuildContext) -> Self {
        self.name("Pager").cached(true)
    }
}
