use orbtk::prelude::*;
use std::cell::Cell;

use euc::{buffer::Buffer2d, rasterizer, Pipeline, Target};
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

#[derive(Clone, Default, PartialEq)]
struct CubePipeline {
    spin: Cell<f32>,
}

impl render::RenderPipeline for CubePipeline {
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn render::RenderPipeline> {
        Box::new(self.clone())
    }
    fn draw_pipeline(&self, image: &mut render::Image) {
        let mut color = Buffer2d::new([image.width() as usize, image.height() as usize], 0);
        let mut depth = Buffer2d::new([image.width() as usize, image.height() as usize], 1.0);

        let mvp = Mat4::perspective_fov_rh_no(
            1.3,
            image.width() as f32,
            image.height() as f32,
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

        image.draw(color.as_ref());
    }
}

#[derive(Default)]
pub struct MainViewState {
    cube_spin: Cell<f32>,
}

impl MainViewState {
    fn spin(&self) {
        self.cube_spin.set(self.cube_spin.get() + 16.0);
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(cube) = context
            .widget()
            .get_mut::<RenderPipeline>()
            .0
            .as_any()
            .downcast_ref::<CubePipeline>()
        {
            cube.spin.set(self.cube_spin.get());
        }
    }
}

widget!(
    MainView<MainViewState> {
         cube_pipeline: RenderPipeline
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("MainView")
            .cube_pipeline(RenderPipeline(Box::new(CubePipeline::default())))
            .child(
                Grid::create()
                    .rows(Rows::create().row("*").row("*").build())
                    .child(
                        TextBlock::create()
                            .attach(GridRow(0))
                            .text("Canvas (euc crate pipeline)")
                            .selector(SelectorValue::new().with("text-block").class("h1"))
                            .margin(4.0)
                            .build(ctx),
                    )
                    .child(Canvas::create().attach(GridRow(0)).pipeline(id).build(ctx))
                    .child(
                        Button::create()
                            .text("spin cube")
                            .vertical_alignment("End")
                            .attach(GridRow(0))
                            .margin(4.0)
                            .on_click(move |_| {
                                state.spin();
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::create()
                            .attach(GridRow(1))
                            .text("Canvas (OrbTk 2D pipeline)")
                            .selector(SelectorValue::new().with("text-block").class("h1"))
                            .margin(4.0)
                            .build(ctx),
                    )
                    .child(Canvas::create().attach(GridRow(1)).build(ctx))
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
