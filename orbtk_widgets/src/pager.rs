use crate::{api::prelude::*, proc_macros::*};

/// Use this enum to trigger navigation actions on a pager.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PagerAction {
    /// Navigates to the next child. If the current child is the last in the list nothing will happen.
    Next,

    /// Navigates to the previous child. If the current child is the first in the list nothing will happen.
    Previous,

    /// Navigates to the given index. Is the index out of bounds nothing will happen.
    Navigate(usize),

    /// Navigates to the current given index.
    NavigateToCurrent,

    /// Pushes the given entity on the end of the pagers children.
    Push(Entity),

    /// Removes the child on the given index. If the index is out of bounds nothing will happen.
    Remove(usize),
}

/// Structure of elements used to handle the pager status.
#[derive(AsAny, Clone, Debug, Default)]
pub struct PagerState {
    current_index: usize,
}

impl PagerState {
    /// Returns the current index.
    pub fn get_current_index(&self) -> usize {
        self.current_index
    }

    /// Sets the current index.
    pub fn set_current_index(&mut self, new_index: usize) -> usize {
        self.current_index = new_index;
        self.current_index
    }
}

impl State for PagerState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        PagerState::set_current_index(self, Pager::correct_current_index(ctx));

        if let Some(count) = ctx.widget().children_count() {
            for i in 0..count {
                // make child on the current index always visible
                if i == PagerState::get_current_index(self) {
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

        Pager::update_next_previous_enabled(ctx, self.current_index);
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for action in messages.read::<PagerAction>() {
            match action {
                PagerAction::Next => Pager::next(ctx, ctx.entity()),
                PagerAction::Previous => Pager::previous(ctx, ctx.entity()),
                PagerAction::Navigate(index) => Pager::navigate(ctx, ctx.entity(), index),
                PagerAction::Remove(index) => Pager::remove(ctx, ctx.entity(), index),
                PagerAction::Push(entity) => Pager::push(ctx, ctx.entity(), entity),
                PagerAction::NavigateToCurrent => {
                    PagerState::set_current_index(self, Pager::correct_current_index(ctx));
                    Pager::navigate(ctx, ctx.entity(), PagerState::get_current_index(self));
                }
            }
        }
    }
}

widget!(
    /// `Pager` is a navigation widget that provides a stack based navigation.
    ///
    /// There are two way to interact with the `Pager`. By using the
    /// `PagerState` and by using the associated functions of `Pager`.
    /// It is suggested to use the state methods on callbacks and the
    /// associated functions inside of states of other widgets.
    ///
    /// # Example
    ///
    /// ```rust
    /// let pager = Pager::new()
    ///     .child(TextBlock::new().text("Page 1").build(ctx))
    ///     .child(TextBlock::new().text("Page 2").build(ctx))
    ///     .build(ctx);
    ///
    /// let next_button = Button::new()
    ///     .enabled(("next_enabled", pager))
    ///     .text("next")
    ///     .on_click(move |states, _| {
    ///         states.send_message(PagerAction::Next, pager);
    ///         true
    ///     })
    ///     .build(ctx);
    ///
    /// let previous_button = Button::new()
    ///     .enabled(("previous_enabled", pager))
    ///     .text("previous")
    ///     .on_click(move |states, _| {
    ///         states.send_message(PagerAction::Previous, pager);
    ///         true
    ///     })
    ///     .build(ctx);
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

// associated functions
impl Pager {
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

    /// Navigates to the given index. Is the index out of bounds nothing will happen.
    pub fn navigate(ctx: &mut Context, entity: Entity, index: usize) {
        // update enabled next / previous
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        // if the given index equals to the current index, the visible item it returned
        if let Some(child) = ctx.try_child_from_index(index) {
            if *child.get::<Visibility>("visibility") == Visibility::Visible {
                return;
            }
        }

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

            // mark corresponding child for given index as visible
            if let Some(child) = &mut ctx.try_child_from_index(index) {
                child.set("visibility", Visibility::Visible);
            }

            Pager::current_index_set(&mut ctx.get_widget(entity), index);

            Pager::update_next_previous_enabled(ctx, index);
        }
    }

    /// Navigates to the next child. If the current child is the last
    /// in the list nothing will happen.
    pub fn next(ctx: &mut Context, entity: Entity) {
        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));
        Pager::navigate(ctx, entity, current_index + 1);
    }

    /// Navigates to the previous child. If the current child is the
    /// first in the list nothing will happen.
    pub fn previous(ctx: &mut Context, entity: Entity) {
        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));
        if current_index == 0 {
            return;
        }
        Pager::navigate(ctx, entity, current_index - 1);
    }

    /// Pushes the given entity on the end of the pagers children.
    pub fn push(ctx: &mut Context, entity: Entity, child: Entity) {
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        ctx.append_child_entity_to(child, entity);
        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));
        Pager::update_next_previous_enabled(ctx, current_index);
    }

    /// Removes the child on the given index. If the index is out of
    /// bounds nothing will happen.
    pub fn remove(ctx: &mut Context, entity: Entity, index: usize) {
        Pager::panics_on_wrong_type(&ctx.get_widget(entity));

        let current_index = *Pager::current_index_ref(&ctx.get_widget(entity));

        let child_entity = ctx.try_child_from_index(index).map(|child| child.entity());

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

    // checks and updates if next and previous can be executed.
    fn update_next_previous_enabled(ctx: &mut Context, current_index: usize) {
        if current_index == 0 {
            Pager::previous_enabled_set(&mut ctx.widget(), false);
        } else {
            Pager::previous_enabled_set(&mut ctx.widget(), true);
        }

        if let Some(count) = ctx.widget().children_count() {
            if current_index == count - 1 {
                Pager::next_enabled_set(&mut ctx.widget(), false);
            } else {
                Pager::next_enabled_set(&mut ctx.widget(), true);
            }
        }
    }
}

impl Template for Pager {
    fn template(self, _id: Entity, _context: &mut BuildContext) -> Self {
        self.name("Pager").on_changed("current_index", |ctx, id| {
            ctx.send_message(PagerAction::NavigateToCurrent, id);
        })
    }
}
