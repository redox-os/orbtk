use orbtk::prelude::*;
use render::three::Pipeline;

#[derive(Clone, PartialEq)]
pub struct Cube {}

impl render::RenderPipeline for Cube {
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
        let mut color = render::three::buffer::Buffer2d::new([image.width() as usize, image.height() as usize], 0);
        let mut depth = render::three::buffer::Buffer2d::new([image.width() as usize, image.height() as usize], 1.0);
        self.draw::<render::three::rasterizer::Triangles<_>, _>(
            &[
                [-1.0, -1.0, 0.0, 1.0],
                [1.0, -1.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
            ],
            &mut color,
            &mut depth,
        );

        image.draw(color.as_ref());
    }
}

impl Pipeline for Cube {
    type Vertex = [f32; 4];
    type VsOut = ();
    type Pixel = u32;

    // Vertex shader
    // - Returns the 3D vertex location, and the VsOut value to be passed to the fragment shader
    #[inline(always)]
    fn vert(&self, pos: &[f32; 4]) -> ([f32; 4], Self::VsOut) {
        (*pos, ())
    }

    // Specify the depth buffer strategy used for each draw call
    #[inline(always)]
    fn get_depth_strategy(&self) -> render::three::DepthStrategy {
        render::three::DepthStrategy::None
    }

    // Fragment shader
    // - Returns (in this case) a u32
    #[inline(always)]
    fn frag(&self, _: &Self::VsOut) -> Self::Pixel {
        let bytes = [255, 0, 0, 255]; // Red

        (bytes[2] as u32) << 0
            | (bytes[1] as u32) << 8
            | (bytes[0] as u32) << 16
            | (bytes[3] as u32) << 24
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Container::create()
                        .width(100.0)
                        .height(100.0)
                        .border_radius(2.0)
                        .border_thickness(1.0)
                        .border_brush("#000000")
                        .child(
                            CanvasThree::create()
                                .pipeline(RenderPipeline(Box::new(Cube {})))
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
