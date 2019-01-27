use crate::{
    enums::ParentType,
    layout::GridLayout,
    properties::{Background, Columns, Rows},
    render_object::RectangleRenderObject,
    theme::Selector,
    widget::{Template, Widget},
};

/// Defines a flexible grid area that consists of columns and rows.
///
/// # Properties
///
/// * `Background` - stores the css background.
/// * `Columns` - used to define the columns of the grid.
/// * `Rows` - used to define the rows of the grid.
/// * `Selector` - css selector with element `grid`.
///
/// # CSS properties
///
/// * `background` - defines the background of the widget.
///
/// # Others
///
/// * `ParentType`- Multi.
/// * `GridLayout` - used to layout the widget.
pub struct Grid;

impl Widget for Grid {
    type Template = Template;

    fn create() -> Self::Template {
        Template::default()
            .property(Background::default())
            .property(Columns::default())
            .property(Rows::default())
            .parent_type(ParentType::Multi)
            .layout(GridLayout::default())
            .render_object(RectangleRenderObject)
            .selector("grid")
            .debug_name("Grid")
    }
}

//impl From<GridTemplate> for Template {
//    fn from(temp: GridTemplate) -> Template {
//        temp.template
//    }
//}
//
//impl From<Template> for GridTemplate {
//    fn from(temp: Template) -> GridTemplate {
//        GridTemplate {
//            template: temp
//        }
//    }
//}
//
//pub struct GridTemplate {
//    template: Template
//}
//
//impl Templateable for GridTemplate {
//    fn template<F: FnOnce(Template) -> Templateable>(self, transform: F) -> Self
//        where Self: Into<Template> + From<Template>
//    {
//        Self::from(transform(self.into))
//    }
//}
//
//pub trait Templateable {
//    fn template<F: FnOnce(Template) -> Templateable>(self, transform: F) -> Self
//        where Self: Into<Template> + From<Template>;
//
//    fn background(mut self, background: Background) -> Self {
//        self.template(move |template| template.property(background))
//    }
//}
