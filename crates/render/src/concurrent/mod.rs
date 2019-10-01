use std::{
    sync::{mpsc, Arc, Mutex, MutexGuard},
    thread,
};

use crate::{
    platform::{self, Font, Image},
    utils::*,
    TextMetrics,
};

pub enum RenderTask {
    Resize {
        width: f64,
        height: f64,
    },
    RegisterFont {
        family: String,
        font_file: &'static [u8],
    },
    FillRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    StrokeRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    FillText {
        text: String,
        x: f64,
        y: f64,
    },
    MeasureText {
        text: String,
    },
    Fill(),
    Stroke(),
    BeginPath(),
    ClosePath(),
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    Arc {
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    },
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    QuadraticCurveTo {
        cpx: f64,
        cpy: f64,
        x: f64,
        y: f64,
    },
    BesierCurveTo {
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    },
    // todo: draw image
    Clip(),
    SetLineWidth {
        line_width: f64,
    },
    SetFontFamily {
        family: String,
    },
    SetFontSize {
        size: f64,
    },
    SetFillStyle {
        fill_style: Brush,
    },
    SetStrokeStyle {
        stroke_style: Brush,
    },
    Save(),
    Restore(),
    Clear {
        brush: Brush,
    }, // todo: transform
    Finish(),
}

pub enum RenderResult {
    TextMetrics(TextMetrics),
    Finish { data: Vec<u32> },
}

struct RenderWorker {
    render_thread: thread::JoinHandle<()>,
}

impl RenderWorker {
    fn new(
        width: f64,
        height: f64,
        receiver: Arc<Mutex<mpsc::Receiver<RenderTask>>>,
        sender: Arc<Mutex<mpsc::Sender<RenderResult>>>,
    ) -> Self {
        let render_thread = thread::spawn(move || {
            let mut tasks = vec![];

            let mut render_context_2_d = platform::RenderContext2D::new(width, height);

            loop {
                let task = receiver.lock().unwrap().recv().unwrap();

                tasks.push(task);

                if tasks.len() > 0 {
                    match tasks.remove(0) {
                        RenderTask::Resize { width, height } => {
                            render_context_2_d.resize(width, height);
                        }
                        RenderTask::RegisterFont { family, font_file } => {
                            render_context_2_d.register_font(family.as_str(), font_file);
                        }
                        RenderTask::FillRect {
                            x,
                            y,
                            width,
                            height,
                        } => {
                            render_context_2_d.fill_rect(x, y, width, height);
                        }
                        RenderTask::StrokeRect {
                            x,
                            y,
                            width,
                            height,
                        } => {
                            render_context_2_d.stroke_rect(x, y, width, height);
                        }
                        RenderTask::FillText { text, x, y } => {
                            render_context_2_d.fill_text(text.as_str(), x, y, None);
                        }
                        RenderTask::MeasureText { text } => {
                            sender.lock().unwrap().send(RenderResult::TextMetrics(
                                render_context_2_d.measure_text(text.as_str()),
                            ));
                        }
                        RenderTask::Fill() => {
                            render_context_2_d.fill();
                        }
                        RenderTask::Stroke() => {
                            render_context_2_d.stroke();
                        }
                        RenderTask::BeginPath() => {
                            render_context_2_d.begin_path();
                        }
                        RenderTask::ClosePath() => {
                            render_context_2_d.close_path();
                        }
                        RenderTask::Rect {
                            x,
                            y,
                            width,
                            height,
                        } => {
                            render_context_2_d.rect(x, y, width, height);
                        }
                        RenderTask::Arc {
                            x,
                            y,
                            radius,
                            start_angle,
                            end_angle,
                        } => {
                            render_context_2_d.arc(x, y, radius, start_angle, end_angle, true);
                        }
                        RenderTask::MoveTo { x, y } => {
                            render_context_2_d.move_to(x, y);
                        }
                        RenderTask::LineTo { x, y } => {
                            render_context_2_d.line_to(x, y);
                        }
                        RenderTask::QuadraticCurveTo { cpx, cpy, x, y } => {
                            render_context_2_d.quadratic_curve_to(cpx, cpy, x, y);
                        }
                        RenderTask::BesierCurveTo {
                            cp1x,
                            cp1y,
                            cp2x,
                            cp2y,
                            x,
                            y,
                        } => {
                            render_context_2_d.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
                        }
                        RenderTask::SetLineWidth { line_width } => {
                            render_context_2_d.set_line_width(line_width);
                        }
                        RenderTask::Clip() => {
                            render_context_2_d.clip();
                        }
                        RenderTask::SetFontFamily { family } => {
                            render_context_2_d.set_font_family(family);
                        }
                        RenderTask::SetFontSize { size } => {
                            render_context_2_d.set_font_size(size);
                        }
                        RenderTask::SetFillStyle { fill_style } => {
                            render_context_2_d.set_fill_style(fill_style);
                        }
                        RenderTask::SetStrokeStyle { stroke_style } => {
                            render_context_2_d.set_stroke_style(stroke_style);
                        }
                        RenderTask::Save() => {
                            render_context_2_d.save();
                        }
                        RenderTask::Restore() => {
                            render_context_2_d.restore();
                        }
                        RenderTask::Clear { brush } => {
                            render_context_2_d.clear(&brush);
                        }
                        Finish => {
                            sender.lock().unwrap().send(RenderResult::Finish {
                                data: render_context_2_d.data().iter().map(|a| *a).collect(),
                            });

                            return;
                        }
                    };
                }
            }
        });

        RenderWorker {
            render_thread
        }
    }
}

/// The RenderContext2D provides a concurrent 2D render context.
pub struct RenderContext2D {
    width: f64,
    height: f64,
    output: Vec<u32>,
    worker: Option<RenderWorker>,
    sender: mpsc::Sender<RenderTask>,
    receiver: Arc<Mutex<mpsc::Receiver<RenderTask>>>,
    result_sender: Arc<Mutex<mpsc::Sender<RenderResult>>>,
    result_receiver: mpsc::Receiver<RenderResult>,
    finished: bool,
}

impl RenderContext2D {
    /// Creates a new render context 2d.
    pub fn new(width: f64, height: f64) -> Self {
        let (sender, receiver) = mpsc::channel();

        let (result_sender, result_receiver) = mpsc::channel();

        let task = RenderTask::Clip();

        RenderContext2D {
            width,
            height,
            output: vec![0; width as usize * height as usize],
            worker: None,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            result_sender: Arc::new(Mutex::new(result_sender)),
            result_receiver,
            finished: false,
        }
    }

    pub fn start(&mut self) {
        if self.worker.is_some() {
            return;
        }

        self.worker = Some(RenderWorker::new(
            self.width,
            self.height,
            self.receiver.clone(),
            self.result_sender.clone(),
        ));
    }

    pub fn finish(&mut self) {
        self.start();
        self.sender.send(RenderTask::Finish());
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.start();
        self.sender.send(RenderTask::Resize { width, height });
    }

    /// Registers a new font file.
    pub fn register_font(&mut self, family: &str, font_file: &'static [u8]) {
        // todo: fix font loading
        self.start();
        self.sender.send(RenderTask::RegisterFont {
            family: family.to_string(),
            font_file,
        });
    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.start();
        self.sender.send(RenderTask::FillRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.start();
        self.sender.send(RenderTask::StrokeRect {
            x,
            y,
            width,
            height,
        });
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64, o: Option<f64>) {
        self.start();
        self.sender.send(RenderTask::FillText {
            text: text.to_string(),
            x,
            y,
        });
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        self.start();
        let mut text_metrics = TextMetrics::default();
        self.sender.send(RenderTask::MeasureText {
            text: text.to_string(),
        });
        if let RenderResult::TextMetrics(t_m) = self.result_receiver.recv().unwrap() {
            text_metrics = t_m;
        }

        text_metrics
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.start();
        self.sender.send(RenderTask::Fill());
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.start();
        self.sender.send(RenderTask::Stroke());
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.start();
        self.sender.send(RenderTask::BeginPath());
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.start();
        self.sender.send(RenderTask::ClosePath());
    }
    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.start();
        self.sender.send(RenderTask::Rect {
            x,
            y,
            width,
            height,
        });
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, o: bool) {
        self.start();
        self.sender.send(RenderTask::Arc {
            x,
            y,
            radius,
            start_angle,
            end_angle,
        });
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.start();
        self.sender.send(RenderTask::MoveTo { x, y });
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.start();
        self.sender.send(RenderTask::LineTo { x, y });
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.start();
        self.sender
            .send(RenderTask::QuadraticCurveTo { cpx, cpy, x, y });
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.start();
        self.sender.send(RenderTask::BesierCurveTo {
            cp1x,
            cp1y,
            cp2x,
            cp2y,
            x,
            y,
        });
    }

    // Draw image

    /// Draws the image.
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {}

    /// Draws the image with the given size.
    pub fn draw_image_with_size(
        &mut self,
        image: &mut Image,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip_and_size(
        &mut self,
        image: &mut Image,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        self.start();
        self.sender.send(RenderTask::Clip());
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.start();
        self.sender.send(RenderTask::SetLineWidth { line_width });
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.start();
        let family = family.into();
        self.sender.send(RenderTask::SetFontFamily { family });
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        self.start();
        self.sender.send(RenderTask::SetFontSize { size });
    }

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, fill_style: Brush) {
        self.start();
        self.sender.send(RenderTask::SetFillStyle { fill_style });
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: Brush) {
        self.start();
        self.sender
            .send(RenderTask::SetStrokeStyle { stroke_style });
    }

    // Transformations

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {}

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {}

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.start();
        self.sender.send(RenderTask::Save());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.start();
        self.sender.send(RenderTask::Restore());
    }

    pub fn clear(&mut self, brush: &Brush) {
        self.start();
        let brush = brush.clone();
        self.sender.send(RenderTask::Clear { brush });
    }

    pub fn data(&mut self) -> Option<&[u32]> {
        if let Ok(result) = self.result_receiver.try_recv() {
            if let RenderResult::Finish { data } = result {
                self.worker = None;
                self.output = data;
                return Some(&self.output);
            }
        }

        None
    }

    pub fn data_mut(&mut self) -> &mut [u32] {
        &mut self.output
    }

    pub fn data_u8_mut(&mut self) -> &mut [u8] {
        let p = self.output[..].as_mut_ptr();
        let len = self.output[..].len();
        // we want to return an [u8] slice instead of a [u32] slice. This is a safe thing to
        // do because requirements of a [u32] slice are stricter.
        unsafe { std::slice::from_raw_parts_mut(p as *mut u8, len * std::mem::size_of::<u32>()) }
    }
}
