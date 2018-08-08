use std::sync::Arc;

use orbclient::Renderer;

use super::{Id, Tree};
use structs::Rect;
use theme::Theme;
use widgets::WidgetType;

pub struct TreeRenderer;

impl TreeRenderer {
    pub fn new() -> Arc<TreeRenderer> {
        Arc::new(TreeRenderer {})
    }

    pub fn render(&self, tree: &Tree, renderer: &mut Renderer, theme: &Theme) {
        self.render_tree(tree.graph.root(), tree, renderer, theme);
    }

    fn render_tree(&self, widget_id: Id, tree: &Tree, renderer: &mut Renderer, theme: &Theme) {
        for widget_type in tree.widget(widget_id).types() {
            match widget_type {
                WidgetType::Drawable(drawable) => {
                    drawable.draw(&Rect::new(0, 0, 100, 20), renderer, false, theme);
                }
                _ => {}
            }
        }

        for child in tree.graph.children(widget_id) {
            self.render_tree(*child, tree, renderer, theme);
        }
    }
}
