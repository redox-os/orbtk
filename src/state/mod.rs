use widget::WidgetContainer;

pub trait State {
    fn update(&self, widget: &mut WidgetContainer);
}
