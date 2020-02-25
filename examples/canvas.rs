use orbtk::{prelude::*, render::platform::RenderContext2D, utils};
use std::cell::Cell;

use euc::{buffer::Buffer2d, rasterizer, Pipeline};
use vek::*;

// Cube is copied from euc spinning_cube example
// https://github.com/zesterer/euc/blob/master/examples/spinning_cube.rs
struct Cube<'a> {
    mvp: Mat4<f32>,
    positions: &'a [Vec4<f32>],
}

impl<'a> Pipeline for Cube<'a> {
    type Vertex = (usize, Rgba<f32>);
    type VsOut = Rgba<f32>;
    type Pixel = u32;

    #[inline(always)]
    fn vert(&self, (v_index, v_color): &Self::Vertex) -> ([f32; 4], Self::VsOut) {
        ((self.mvp * self.positions[*v_index]).into_array(), *v_color)
    }

    #[inline(always)]
    fn frag(&self, v_color: &Self::VsOut) -> Self::Pixel {
        let bytes = v_color.map(|e| (e * 255.0) as u8).into_array();
        (bytes[2] as u32) << 0
            | (bytes[1] as u32) << 8
            | (bytes[0] as u32) << 16
            | (bytes[3] as u32) << 24
    }
}

#[derive(Clone, Default, PartialEq, Pipeline)]
struct CubePipeline {
    spin: Cell<f32>,
}

impl render::RenderPipeline for CubePipeline {
    fn draw(&self, render_target: &mut render::RenderTarget) {
        let mut color = Buffer2d::new(
            [
                render_target.width() as usize,
                render_target.height() as usize,
            ],
            0,
        );
        let mut depth = Buffer2d::new(
            [
                render_target.width() as usize,
                render_target.height() as usize,
            ],
            1.0,
        );

        let mvp = Mat4::perspective_fov_rh_no(
            1.3,
            render_target.width() as f32,
            render_target.height() as f32,
            0.01,
            100.0,
        ) * Mat4::translation_3d(Vec3::new(0.0, 0.0, -2.0))
            * Mat4::<f32>::scaling_3d(0.4)
            * Mat4::rotation_x((self.spin.get() * 0.002) * 8.0)
            * Mat4::rotation_y((self.spin.get() as f32 * 0.004).cos() * 4.0)
            * Mat4::rotation_z((self.spin.get() as f32 * 0.008).sin() * 2.0);

        Cube {
            mvp,
            positions: &[
                Vec4::new(-1.0, -1.0, -1.0, 1.0), // 0
                Vec4::new(-1.0, -1.0, 1.0, 1.0),  // 1
                Vec4::new(-1.0, 1.0, -1.0, 1.0),  // 2
                Vec4::new(-1.0, 1.0, 1.0, 1.0),   // 3
                Vec4::new(1.0, -1.0, -1.0, 1.0),  // 4
                Vec4::new(1.0, -1.0, 1.0, 1.0),   // 5
                Vec4::new(1.0, 1.0, -1.0, 1.0),   // 6
                Vec4::new(1.0, 1.0, 1.0, 1.0),    // 7
            ],
        }
        .draw::<rasterizer::Triangles<_, rasterizer::BackfaceCullingEnabled>, _>(
            &[
                // -x
                (0, Rgba::green()),
                (3, Rgba::blue()),
                (2, Rgba::red()),
                (0, Rgba::green()),
                (1, Rgba::red()),
                (3, Rgba::blue()),
                // +x
                (7, Rgba::blue()),
                (4, Rgba::green()),
                (6, Rgba::red()),
                (5, Rgba::red()),
                (4, Rgba::green()),
                (7, Rgba::blue()),
                // -y
                (5, Rgba::blue()),
                (0, Rgba::red()),
                (4, Rgba::green()),
                (1, Rgba::green()),
                (0, Rgba::red()),
                (5, Rgba::blue()),
                // +y
                (2, Rgba::red()),
                (7, Rgba::blue()),
                (6, Rgba::green()),
                (2, Rgba::red()),
                (3, Rgba::green()),
                (7, Rgba::blue()),
                // -z
                (0, Rgba::red()),
                (6, Rgba::green()),
                (4, Rgba::blue()),
                (0, Rgba::red()),
                (2, Rgba::blue()),
                (6, Rgba::green()),
                // +z
                (7, Rgba::green()),
                (1, Rgba::red()),
                (5, Rgba::blue()),
                (3, Rgba::blue()),
                (1, Rgba::red()),
                (7, Rgba::green()),
            ],
            &mut color,
            &mut depth,
        );

        render_target.draw(color.as_ref());
    }
}

// OrbTk 2D drawing
#[derive(Clone, Default, PartialEq, Pipeline)]
struct Graphic2DPipeline;

impl render::RenderPipeline for Graphic2DPipeline {
    fn draw(&self, render_target: &mut render::RenderTarget) {
        let mut render_context =
            RenderContext2D::new(render_target.width(), render_target.height());

        let width = 120.0;
        let height = 120.0;

        let x = (render_target.width() - width) / 2.0;
        let y = (render_target.height() - height) / 2.0;
        // render_context.set_fill_style(utils::Brush::SolidColor(Color::from("#000000")));

        render_context.set_fill_style(utils::Brush::LinearGradient {
            start: Point::new(x, y),
            end: Point::new(x + width, y + height),
            stops: vec![
                LinearGradientStop {
                    position: 0.0,
                    color: Color::from("#0021EB"),
                },
                LinearGradientStop {
                    position: 0.5,
                    color: Color::from("#CE2F24"),
                },
                LinearGradientStop {
                    position: 1.0,
                    color: Color::from("#70EF49"),
                },
            ],
        });
        render_context.fill_rect(x, y, width, height);
        render_target.draw(render_context.data());
    }
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    cube_spin: f32,
}

impl MainViewState {
    fn spin(&mut self) {
        self.cube_spin += 32.0;
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(cube) = ctx
            .widget()
            .get_mut::<RenderPipeline>("render_pipeline")
            .0
            .as_any()
            .downcast_ref::<CubePipeline>()
        {
            cube.spin.set(self.cube_spin);
        }
    }
}

widget!(
    MainView<MainViewState> {
         render_pipeline: RenderPipeline
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .render_pipeline(RenderPipeline(Box::new(CubePipeline::default())))
            .child(
                Grid::create()
                    .rows(
                        Rows::create()
                            .row("auto")
                            .row("*")
                            .row("auto")
                            .row("*")
                            .build(),
                    )
                    .child(
                        TextBlock::create()
                            .attach(Grid::row(0))
                            .text("Canvas (render with euc crate)")
                            .element("text-block")
                            .class("h1")
                            .margin(4.0)
                            .build(ctx),
                    )
                    .child(
                        Canvas::create()
                            .attach(Grid::row(1))
                            .render_pipeline(id)
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .text("spin cube")
                            .vertical_alignment("end")
                            .attach(Grid::row(1))
                            .margin(4.0)
                            .on_click(move |states, _| {
                                states.get_mut::<MainViewState>(id).spin();
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::create()
                            .attach(Grid::row(2))
                            .text("Canvas (render with OrbTk)")
                            .element("text-block")
                            .class("h1")
                            .margin(4.0)
                            .build(ctx),
                    )
                    .child(
                        Canvas::create()
                            .attach(Grid::row(3))
                            .render_pipeline(RenderPipeline(Box::new(Graphic2DPipeline::default())))
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - canvas example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
