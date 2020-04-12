use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{platform, utils::*, Pipeline, RenderTarget, TextMetrics};
use platform::Image;

#[derive(Clone)]
struct PipelineWrapper(pub Box<dyn Pipeline>);

impl PartialEq for PipelineWrapper {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

// Used to sent render tasks to render thread.
#[derive(Clone, PartialEq)]
enum RenderTask {
    // Single tasks
    Start(),
    Resize {
        width: f64,
        height: f64,
    },
    RegisterFont {
        family: String,
        font_file: &'static [u8],
    },

    // Multi tasks
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
    Fill(),
    Stroke(),
    BeginPath(),
    ClosePath(),
    Rectangle {
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
    DrawRenderTarget {
        render_target: RenderTarget,
        x: f64,
        y: f64,
    },
    DrawImage {
        image: Image,
        x: f64,
        y: f64,
    },
    DrawImageWithClip {
        image: Image,
        clip: Rectangle,
        x: f64,
        y: f64,
    },
    DrawPipeline {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        pipeline: PipelineWrapper,
    },
    Clip(),
    SetLineWidth {
        line_width: f64,
    },
    SetAlpha {
        alpha: f32,
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
    },
    SetTransform {
        h_scaling: f64,
        h_skewing: f64,
        v_skewing: f64,
        v_scaling: f64,
        h_moving: f64,
        v_moving: f64,
    },
    Finish(),
    Terminate(),
}

// Used to send results to the main thread.
enum RenderResult {
    Finish { data: Vec<u32> },
}

// Wrapper for the render thread.
struct RenderWorker {
    render_thread: Option<thread::JoinHandle<()>>,
}

fn is_single_tasks(task: &RenderTask) -> bool {
    match task {
        RenderTask::Start() => true,
        RenderTask::Resize { .. } => true,
        RenderTask::RegisterFont { .. } => true,
        RenderTask::DrawRenderTarget { .. } => true,
        RenderTask::DrawImage { .. } => true,
        RenderTask::DrawImageWithClip { .. } => true,
        RenderTask::DrawPipeline { .. } => true,
        RenderTask::SetTransform { .. } => true,
        RenderTask::Terminate { .. } => true,
        _ => false,
    }
}

impl RenderWorker {
    fn new(
        width: f64,
        height: f64,
        receiver: Arc<Mutex<mpsc::Receiver<Vec<RenderTask>>>>,
        sender: Arc<Mutex<mpsc::Sender<RenderResult>>>,
    ) -> Self {
        let render_thread = thread::spawn(move || {
            let mut tasks_collection = vec![];

            let mut render_context_2_d = platform::RenderContext2D::new(width, height);

            loop {
                let mut tasks = receiver.lock().unwrap().recv().unwrap();

                // single tasks
                if tasks.len() == 1 && is_single_tasks(tasks.get(0).unwrap()) {
                    match tasks.remove(0) {
                        RenderTask::Start() => {
                            tasks_collection.clear();
                            continue;
                        }
                        RenderTask::Resize { width, height } => {
                            render_context_2_d.resize(width, height);
                            continue;
                        }
                        RenderTask::RegisterFont { family, font_file } => {
                            render_context_2_d.register_font(family.as_str(), font_file);
                            continue;
                        }
                        RenderTask::DrawRenderTarget {
                            render_target,
                            x,
                            y,
                        } => {
                            render_context_2_d.draw_render_target(&render_target, x, y);
                        }
                        RenderTask::DrawImage { image, x, y } => {
                            render_context_2_d.draw_image(&image, x, y);
                        }
                        RenderTask::DrawImageWithClip { image, clip, x, y } => {
                            render_context_2_d.draw_image_with_clip(&image, clip, x, y);
                        }
                        RenderTask::DrawPipeline {
                            x,
                            y,
                            width,
                            height,
                            pipeline,
                        } => {
                            render_context_2_d.draw_pipeline(x, y, width, height, pipeline.0);
                        }
                        RenderTask::SetTransform {
                            h_scaling,
                            h_skewing,
                            v_skewing,
                            v_scaling,
                            h_moving,
                            v_moving,
                        } => {
                            render_context_2_d.set_transform(
                                h_scaling, h_skewing, v_skewing, v_scaling, h_moving, v_moving,
                            );
                        }
                        RenderTask::Terminate() => {
                            return;
                        }
                        _ => {}
                    };
                }

                tasks_collection.push(tasks);

                if !tasks_collection.is_empty() {
                    for task in tasks_collection.remove(0) {
                        match task {
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
                                render_context_2_d.fill_text(text.as_str(), x, y);
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
                            RenderTask::Rectangle {
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
                                render_context_2_d.arc(x, y, radius, start_angle, end_angle);
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
                            RenderTask::SetAlpha { alpha } => {
                                render_context_2_d.set_alpha(alpha);
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
                            RenderTask::Finish() => {
                                sender
                                    .lock()
                                    .unwrap()
                                    .send(RenderResult::Finish {
                                        data: render_context_2_d.data().iter().copied().collect(),
                                    })
                                    .expect("Could not send render result to main thread.");
                            }
                            _ => {}
                        };
                    }
                }
            }
        });

        RenderWorker {
            render_thread: Some(render_thread),
        }
    }
}

/// The RenderContext2D provides a concurrent render ctx.
pub struct RenderContext2D {
    output: Vec<u32>,
    worker: RenderWorker,
    sender: mpsc::Sender<Vec<RenderTask>>,
    result_receiver: mpsc::Receiver<RenderResult>,
    tasks: Vec<RenderTask>,
    measure_context: platform::RenderContext2D,
}

impl Drop for RenderContext2D {
    fn drop(&mut self) {
        self.sender
            .send(vec![RenderTask::Terminate()])
            .expect("Could not send terminate to render thread.");
        if let Some(thread) = self.worker.render_thread.take() {
            thread.join().unwrap();
        }
    }
}

impl RenderContext2D {
    /// Creates a new render ctx 2d.
    pub fn new(width: f64, height: f64) -> Self {
        let (sender, receiver) = mpsc::channel();

        let (result_sender, result_receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let result_sender = Arc::new(Mutex::new(result_sender));

        let worker = RenderWorker::new(width, height, receiver, result_sender);

        RenderContext2D {
            output: vec![0; width as usize * height as usize],
            worker,
            sender,
            result_receiver,
            tasks: vec![],
            measure_context: platform::RenderContext2D::new(width, height),
        }
    }

    // Sends a render task to the render thread.
    fn send_tasks(&mut self) {
        if !self.tasks.is_empty() {
            self.sender
                .send(self.tasks.to_vec())
                .expect("Could not send render task.");
            self.tasks.clear();
        }
    }

    /// Starts a new render pipeline.
    pub fn start(&mut self) {
        self.sender
            .send(vec![RenderTask::Start()])
            .expect("Could not send start ot render thread.");
    }

    /// Finishes the current render pipeline.
    pub fn finish(&mut self) {
        self.tasks.push(RenderTask::Finish());
        self.send_tasks();
    }

    /// Resizes the render ctx.
    pub fn resize(&mut self, width: f64, height: f64) {
        self.sender
            .send(vec![RenderTask::Resize { width, height }])
            .expect("Could not send resize to render thread.");
    }

    /// Registers a new font file.
    pub fn register_font(&mut self, family: &str, font_file: &'static [u8]) {
        self.measure_context.register_font(family, font_file);
        self.sender
            .send(vec![RenderTask::RegisterFont {
                family: family.to_string(),
                font_file,
            }])
            .expect("Could not send register font to render thread.");
    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the
    /// specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.tasks.push(RenderTask::FillRect {
            x,
            y,
            width,
            height,
        });
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other ctx settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.tasks.push(RenderTask::StrokeRect {
            x,
            y,
            width,
            height,
        });
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        self.tasks.push(RenderTask::FillText {
            text: text.to_string(),
            x,
            y,
        });
    }

    pub fn measure(
        &mut self,
        text: &str,
        font_size: f64,
        family: impl Into<String>,
    ) -> TextMetrics {
        self.measure_context.set_font_family(family);
        self.measure_context.set_font_size(font_size);
        self.measure_text(text)
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        self.measure_context.measure_text(text)
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.tasks.push(RenderTask::Fill());
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.tasks.push(RenderTask::Stroke());
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.send_tasks();
        self.tasks.push(RenderTask::BeginPath());
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path.
    /// If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.tasks.push(RenderTask::ClosePath());
        self.send_tasks();
    }
    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.tasks.push(RenderTask::Rectangle {
            x,
            y,
            width,
            height,
        });
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius.
    /// The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        self.tasks.push(RenderTask::Arc {
            x,
            y,
            radius,
            start_angle,
            end_angle,
        });
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.tasks.push(RenderTask::MoveTo { x, y });
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to
    /// the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.tasks.push(RenderTask::LineTo { x, y });
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.tasks
            .push(RenderTask::QuadraticCurveTo { cpx, cpy, x, y });
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points:
    /// the first two are control points and the third one is the end point.
    /// The starting point is the latest point in the current path, which can be changed using
    /// MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.tasks.push(RenderTask::BesierCurveTo {
            cp1x,
            cp1y,
            cp2x,
            cp2y,
            x,
            y,
        });
    }

    // Draw image

    pub fn draw_render_target(&mut self, render_target: &RenderTarget, x: f64, y: f64) {
        self.sender
            .send(vec![RenderTask::DrawRenderTarget {
                render_target: render_target.clone(),
                x,
                y,
            }])
            .expect("Could not send render target to render thread.");
    }

    /// Draws the image.
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        self.sender
            .send(vec![RenderTask::DrawImage {
                image: image.clone(),
                x,
                y,
            }])
            .expect("Could not send image to render thread.");
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip(&mut self, image: &mut Image, clip: Rectangle, x: f64, y: f64) {
        self.sender
            .send(vec![RenderTask::DrawImageWithClip {
                image: image.clone(),
                clip,
                x,
                y,
            }])
            .expect("Could not send clipped image to render thread.");
    }

    pub fn draw_pipeline(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        pipeline: Box<dyn Pipeline>,
    ) {
        self.sender
            .send(vec![RenderTask::DrawPipeline {
                x,
                y,
                width,
                height,
                pipeline: PipelineWrapper(pipeline),
            }])
            .expect("Could not send draw_pipeline to render thread.");
    }

    /// Creates a clipping path from the current sub-paths.
    /// Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        self.tasks.push(RenderTask::Clip());
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.tasks.push(RenderTask::SetLineWidth { line_width });
    }

    /// Sets the alpha value,
    pub fn set_alpha(&mut self, alpha: f32) {
        self.tasks.push(RenderTask::SetAlpha { alpha });
    }

    /// Specifies the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        let family = family.into();
        self.tasks.push(RenderTask::SetFontFamily { family });
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        self.tasks.push(RenderTask::SetFontSize { size });
    }

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, fill_style: Brush) {
        self.tasks.push(RenderTask::SetFillStyle { fill_style });
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: Brush) {
        self.tasks.push(RenderTask::SetStrokeStyle { stroke_style });
    }

    // Transformations

    /// Sets the transformation.
    pub fn set_transform(
        &mut self,
        h_scaling: f64,
        h_skewing: f64,
        v_skewing: f64,
        v_scaling: f64,
        h_moving: f64,
        v_moving: f64,
    ) {
        self.tasks.push(RenderTask::SetTransform {
            h_scaling,
            h_skewing,
            v_skewing,
            v_scaling,
            h_moving,
            v_moving,
        });
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.tasks.push(RenderTask::Save());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack.
    /// If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.tasks.push(RenderTask::Restore());
    }

    pub fn clear(&mut self, brush: &Brush) {
        let brush = brush.clone();
        self.tasks.push(RenderTask::Clear { brush });
    }

    pub fn data(&mut self) -> Option<&[u32]> {
        if let Ok(RenderResult::Finish { data }) = self.result_receiver.try_recv() {
            self.output = data;
            Some(&self.output)
        } else {
            None
        }
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
