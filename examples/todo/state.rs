use orbtk::prelude::*;

use std::collections::VecDeque;

#[derive(Debug)]
pub enum Event {
    Create(Entity),
    Delete(Entity),
}

#[derive(AsAny, Default)]
pub struct AppState {
    events: VecDeque<Event>,
    pub notes: Entity,
    count: u32,
}

impl AppState {
    fn append_note(&self, ctx: &mut Context, copy: String) {
        let id = ctx.entity;
        let bctx = &mut ctx.build_context();
        let text = TextBox::create().text(copy);

        let container = Stack::create()
            .orientation(Orientation::Horizontal)
            .vertical_alignment(Alignment::Start)
            .child(text.build(bctx))
            .build(bctx);

        let button = Button::create()
            .horizontal_alignment(Alignment::End)
            .text("Delete")
            .on_click(move |ctxt, _| {
                ctxt.get_mut::<AppState>(id)
                    .events
                    .push_back(Event::Delete(container));

                true
            })
            .build(bctx);

        bctx.append_child(container, button);
        bctx.append_child(self.notes, container);
    }

    fn fetch_text(ctx: &mut Context, entity: Entity) -> Option<String> {
        let mut widget = ctx.get_widget(entity);

        let entry = widget.get_mut::<String16>("text");
        if entry.is_empty() {
            return None;
        }

        let copy = entry.to_string();
        entry.clear();
        Some(copy)
    }

    pub fn send(&mut self, event: Event) {
        self.events.push_back(event);
    }
}

impl State for AppState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(event) = self.events.pop_front() {
            match event {
                // Create and append the note to the UI.
                Event::Create(entity) => {
                    if let Some(copy) = Self::fetch_text(ctx, entity) {
                        self.append_note(ctx, copy);
                        let count = self.count + 1;
                        self.count = count;
                        if count == 1 {
                            ctx.widget().set::<f64>("spacing", 12.0);
                        }
                    }
                }

                // Delete the note of the given ID from the UI.
                Event::Delete(id) => {
                    ctx.remove_child_from(id, self.notes);
                    let count = self.count - 1;
                    self.count = count;
                    if count == 0 {
                        ctx.widget().set::<f64>("spacing", 0.0);
                    }
                }
            }
        }
    }
}
