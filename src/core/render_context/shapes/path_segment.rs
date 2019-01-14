use crate::core::Brush;

use orbgl::Image;

/// A `PathSegment` describes one render step e.g. arc, rect, line_to and will be translated into an render call.
#[derive(Clone)]
pub enum PathSegment {
    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    Arc {
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_engle: f64,
    },

    /// Adds a circular arc to the current sub-path, using the given control points and radius. The arc is automatically connected to the path's latest point with a straight line, if necessary for the specified parameters.
    ArcTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    },

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    BeginPath(),

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    BezierCurveTo {
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    },

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    ClosePath(),

    /// Erases the pixels in a rectangular area by setting them to transparent black.
    ClearRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },

    /// Draws an image.
    DrawImage { image: Image, x: f64, y: f64 },

    /// Draws an image with the given size.
    DrawImageWithSize {
        image: Image,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },

    /// Drawa an image part.
    DrawImageWithClipAndSize {
        image: Image,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },

    /// Fills the current or given path with the current file style.
    Fill(),

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    FillRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },

    /// Draws a text string at the specified coordinates, filling the string's characters with the current foreground color. An optional parameter allows specifying a maximum width for the rendered text, which the user agent will achieve by condensing the text or by using a lower font size.
    FillText {
        text: String,
        x: f64,
        y: f64,
        max_width: Option<f64>,
    },

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    LineTo { x: f64, y: f64 },

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.
    MoveTo { x: f64, y: f64 },

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    Restore(),

    /// Adds a rotation to the transformation matrix.
    Rotate { angle: f64 },

    /// Specifies the brush to use inside shapes.
    SetFillStyleBrush { brush: Brush },

    /// Specifies the current text style being used when drawing text.
    SetFont { font: String },

    /// Sets the thickness of lines.
    SetLineWidth { width: f64 },

    /// Specifies the amount of blur applied to shadows. The default is 0 {no blur}.
    SetShadowBlur { blur: f64 },

    /// Specifies the color of shadows.
    SetShadowColor { color: String },

    /// Specifies the distance that shadows will be offset horizontally.
    SetShadowOffsetX { x: f64 },

    /// Specifies the distance that shadows will be offset vertically.
    SetShadowOffsetY { y: f64 },

    /// Specifies the color or style to use for the lines around shapes. The default is #000 {black}.
    SetStrokeStyleColor { color: String },

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    Save(),

    /// Adds a scaling transformation to the canvas units horizontally and/or vertically.
    Scale { x: f64, y: f64 },

    /// Strokes {outlines} the current or given path with the current stroke style.
    Stroke(),

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    Transform {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    },

    /// Adds a translation transformation to the current matrix.
    Translate { x: f64, y: f64 },
}