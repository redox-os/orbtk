use std::collections::HashMap;
use std::sync::Arc;

use orbclient::{Color, Renderer as OrbRenderer, Window as OrbWindow};
use orbfont::Font;
use orbgl::Canvas;
use orbimage::Image;

use crate::{
    core::{Brush, FillRule, GradientStop, ImageElement, RenderContext2D, Renderer, TextMetrics},
    properties::{Bounds, Point},
    theme::{material_font_icons::MATERIAL_ICONS_REGULAR_FONT, ROBOTO_REGULAR_FONT},
};

pub struct OrbRenderContext2D<'a> {
    pub orbgl_context: &'a mut Canvas,
    pub orbclient_context: &'a mut OrbWindow,
    pub image_cache: &'a mut HashMap<String, Image>,
    pub fonts: &'a mut HashMap<String, Font>,
    pub position: (f64, f64),
    pub fill_color: Color,
    pub stroke_color: Color,
    pub gradient: Vec<GradientStop>,
}

impl<'a> OrbRenderContext2D<'a> {
    fn get_color(&self, hex: &str) -> Color {
        let clean_hex = hex.trim_start_matches("#");
        match clean_hex.len() {
            6 | 8 => {
                let mut x = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                if clean_hex.len() == 6 {
                    x |= 0xFF_000_000;
                }

                Color { data: x }
            }
            _ => Color { data: 0 },
        }
    }

    // Helper that writes the data from orb_gl_context to orbclient_context
    fn switch_context(&mut self) {
        self.orbclient_context.image_fast(
            0,
            0,
            self.orbclient_context.width(),
            self.orbclient_context.height(),
            &self.orbgl_context.data,
        );
        self.orbgl_context.clear_rect(
            0.0,
            0.0,
            self.orbclient_context.width() as f32,
            self.orbclient_context.height() as f32,
        );
    }
}

impl<'a> RenderContext2D for OrbRenderContext2D<'a> {
    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle, and travels in the direction given by anticlockwise (defaulting to clockwise).
    fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_engle: f64,
        anti_clockwise: bool,
    ) {
        self.switch_context();
        // todo: needs implementation
    }

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    fn arc_to(&mut self, x: f64, y: f64) {
        self.switch_context();
        // todo: needs implementation
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    fn begin_path(&mut self) {
        self.orbgl_context.begin_path();
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo() before creating the Bézier curve.
    fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.orbgl_context.bezier_curve_to(
            cp1x as f32,
            cp1y as f32,
            cp2x as f32,
            cp2y as f32,
            x as f32,
            y as f32,
        )
    }

    /// Erases the pixels in a rectangular area by setting them to transparent black.
    fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.orbgl_context
            .clear_rect(x as f32, y as f32, width as f32, height as f32);
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    fn close_path(&mut self) {
        self.orbgl_context.close_path();
    }

    /// Draws an image on (x, y).
    fn draw_image(&mut self, image_element: &ImageElement, x: f64, y: f64) {
        self.switch_context();
        if !self.image_cache.contains_key(&image_element.path) {
            if let Ok(image) = Image::from_path(&image_element.path) {
                self.image_cache.insert(image_element.path.clone(), image);
            }
        }

        if let Some(image) = self.image_cache.get(&image_element.path) {
            self.orbclient_context.image_fast(
                x as i32,
                y as i32,
                image.width(),
                image.height(),
                image.data(),
            );
        }
    }

    /// Draws an image on (x, y) with (width, height).
    fn draw_image_d(
        &mut self,
        image_element: &ImageElement,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        self.switch_context();
        if !self.image_cache.contains_key(&image_element.path) {
            if let Ok(image) = Image::from_path(&image_element.path) {
                self.image_cache.insert(image_element.path.clone(), image);
            }
        }

        if let Some(image) = self.image_cache.get(&image_element.path) {
            self.orbclient_context.image_fast(
                x as i32,
                y as i32,
                width as u32,
                height as u32,
                image.data(),
            );
        }
    }

    /// Draws a part of the image with the given (source_x, source_y, source_width, source_height) on (x, y) with (width, height).
    fn draw_image_s(
        &mut self,
        image_element: &ImageElement,
        source_x: f64,
        source_y: f64,
        source_width: f64,
        source_height: f64,
        x: f64,
        y: f64,
        _: f64,
        _: f64,
    ) {
        self.switch_context();
        if !self.image_cache.contains_key(&image_element.path) {
            if let Ok(image) = Image::from_path(&image_element.path) {
                self.image_cache.insert(image_element.path.clone(), image);
            }
        }

        if let Some(image) = self.image_cache.get(&image_element.path) {
            let image_roi = image.roi(
                source_x as u32,
                source_y as u32,
                source_width as u32,
                source_height as u32,
            );
            image_roi.draw(self.orbclient_context, x as i32, y as i32);
        }
    }

    /// Fills the current or given path with the current file style.
    fn fill(&mut self, _: FillRule) {
        self.orbgl_context.fill();
    }

    /// Draws a filled rectangle whose starting point is at the coordinates (x, y) with the specified width and height and whose style is determined by the fillStyle attribute.
    fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.orbgl_context
            .fill_rect(x as f32, y as f32, width as f32, height as f32);
    }

    /// Draws a text string at the specified coordinates, filling the string's characters with the current foreground color. An optional parameter allows specifying a maximum width for the rendered text, which the user agent will achieve by condensing the text or by using a lower font size.
    fn fill_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        // todo: needs implementation
        // todo: use OrbClient
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified (x, y) coordinates.
    fn line_to(&mut self, x: f64, y: f64) {
        self.orbgl_context.line_to(x as f32, y as f32);
    }

    /// Begins a new sub-path at the point specified by the given (x, y) coordinates.
    fn move_to(&mut self, x: f64, y: f64) {
        self.position = (x, y);
        self.orbgl_context.move_to(x as f32, y as f32);
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    fn restore(&mut self) {
        self.orbgl_context.restore();
    }

    /// Adds a rotation to the transformation matrix.
    fn rotate(&mut self, angle: f64) {
        self.orbgl_context.rotate(angle as f32);
    }

    /// Specifies the brush to use inside shapes.
    fn set_fill_style_brush(&mut self, brush: &Brush) {
        match brush {
            Brush::SolidColor(color) => {
                let color = self.get_color(color);
                self.fill_color = color;
            }
            Brush::Gradient(gradient) => {
                self.gradient = gradient.clone();
            }
        }

        self.orbgl_context.set_fill_style(self.fill_color);
    }

    /// Specifies the current text style being used when drawing text.
    fn set_font(&mut self, font: &str) {
        // todo: needs implementation
        // todo: use OrbClient
    }

    /// Sets the thickness of lines.
    fn set_line_width(&mut self, width: f64) {
        self.orbgl_context.set_line_width(width as f32);
    }

    /// Specifies the amount of blur applied to shadows. The default is 0 (no blur).
    fn set_shadow_blur(&mut self, blur: f64) {
        // todo: needs implementation
        // todo: use OrbClient
    }

    /// Specifies the color of shadows.
    fn set_shadow_color(&mut self, color: &str) {
        // todo: needs implementation
        // todo: use OrbClient
    }

    /// Specifies the distance that shadows will be offset horizontally.
    fn set_shadow_offset_x(&mut self, x: f64) {
        // todo: needs implementation
        // todo: use OrbClient
    }

    /// Specifies the distance that shadows will be offset vertically.
    fn set_shadow_offset_y(&mut self, y: f64) {
        // todo: needs implementation
        // todo: use OrbClient
    }

    /// Specifies the color or style to use for the lines around shapes. The default is #000 (black).
    fn set_stroke_style_color(&mut self, color: &str) {
        let color = self.get_color(color);
        self.fill_color = color;
        self.orbgl_context.set_stroke_style(color);
    }

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    fn save(&mut self) {
        self.orbgl_context.save();
    }

    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    fn scale(&mut self, x: f64, y: f64) {
        self.orbgl_context.scale(x as f32, y as f32);
    }

    /// Strokes (outlines) the current or given path with the current stroke style.
    fn stroke(&mut self) {
        self.orbgl_context.stroke();
    }

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.orbgl_context
            .transform(a as f32, b as f32, c as f32, d as f32, e as f32, f as f32);
    }

    /// Adds a translation transformation to the current matrix.
    fn translate(&mut self, x: f64, y: f64) {
        self.orbgl_context.translate(x as f32, y as f32);
    }

    /// Returns a `TextMetrics` object that contains information about the measured text (such as its width for example).
    fn measure_text(&self, text: &str) -> TextMetrics {
        // todo: implement with OrbClient
        TextMetrics { width: 12.0 }
    }

    fn finish(&self) {}

    /// Registers a new font from a path.
    fn register_font(&mut self, path: &str) {
        if let Ok(font) = Font::from_path(path) {}
    }
}

pub struct OrbFontRenderer {
    pub fonts: HashMap<&'static str, Font>,
}

impl OrbFontRenderer {
    fn render(
        &self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        renderer: &mut OrbWindow,
        font_size: f32,
        color: Color,
        font: &str,
    ) {
        if let Some(font) = &self.fonts.get(font) {
            let line = font.render(text, font_size);
            line.draw_clipped(
                renderer,
                global_position.x + bounds.x,
                global_position.y + bounds.y,
                global_position.x,
                parent_bounds.width,
                color,
            );
        } else {
            let rect = Bounds::new(
                global_position.x + bounds.x,
                global_position.y + bounds.y,
                bounds.width,
                bounds.height,
            );
            let mut current_rect = Bounds::new(rect.x, rect.y, rect.width, rect.height);
            let x = rect.x;

            for c in text.chars() {
                if c == '\n' {
                    current_rect.x = x;
                    current_rect.y += 16;
                } else {
                    if current_rect.x + 8 >= global_position.x
                        && current_rect.y + 16 >= global_position.y
                        && current_rect.x + 8 < global_position.x + parent_bounds.width as i32
                        && current_rect.y < global_position.y + parent_bounds.height as i32
                    {
                        renderer.char(current_rect.x, current_rect.y, c, color);
                    }
                    current_rect.x += 8;
                }
            }
        }
    }
}

lazy_static! {
    pub static ref FONT_RENDERER: Arc<OrbFontRenderer> = {
        let mut fonts = HashMap::new();

        if let Ok(font) = Font::from_data(ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
            fonts.insert("Roboto Regular", font);
        }

        if let Ok(font) = Font::from_data(MATERIAL_ICONS_REGULAR_FONT.to_vec().into_boxed_slice()) {
            fonts.insert("Material Icons Regular", font);
        }

        Arc::new(OrbFontRenderer { fonts })
    };
}

impl Renderer for OrbWindow {
    fn render(&mut self, background: Color) {
        // render window background
        self.set(background);
    }

    fn render_rectangle(
        &mut self,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        border_radius: u32,
        background: Color,
        border_width: u32,
        border_color: Color,
        opacity: f32,
    ) {
        let background = {
            if opacity < 1.0 {
                Color {
                    data: (((opacity * 255.0) as u32) << 24)
                        | ((background.r() as u32) << 16)
                        | ((background.g() as u32) << 8)
                        | (background.b() as u32),
                }
            } else {
                background
            }
        };

        let border_color = {
            if opacity < 1.0 {
                Color {
                    data: (((opacity * 255.0) as u32) << 24)
                        | ((border_color.r() as u32) << 16)
                        | ((border_color.g() as u32) << 8)
                        | (border_color.b() as u32),
                }
            } else {
                border_color
            }
        };

        let x = (bounds.x + global_position.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y).max(parent_bounds.y);
        let width = (bounds.width as i32).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32).min(parent_bounds.height as i32) as u32;

        self.rounded_rect(x, y, width, height, border_radius, true, background);

        if border_width > 0 {
            self.rounded_rect(x, y, width, height, border_radius, false, border_color);
        }
    }

    fn render_text(
        &mut self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &str,
    ) {
        // todo handle alpha by orbfong
        let alpha = (color.data >> 24) & 0xFF;

        if alpha == 0 {
            return;
        }

        FONT_RENDERER.render(
            text,
            bounds,
            parent_bounds,
            global_position,
            self,
            font_size as f32,
            color,
            font,
        );
    }

    fn render_image(
        &mut self,
        image: &[Color],
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
    ) {
        let x = (bounds.x + global_position.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y).max(parent_bounds.y);
        let width = (bounds.width as i32).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32).min(parent_bounds.height as i32) as u32;

        self.image_fast(x, y, width, height, image);
    }
}
