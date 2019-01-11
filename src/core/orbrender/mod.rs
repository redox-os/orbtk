pub use self::shapes::*;
pub use self::structs::*;
pub use self::render_context_2d::*;

mod shapes;
mod structs;
mod render_context_2d;

// todos
//
// Step One Render Object refactoring
//
// *[*] Rename backend to core
// *[*] Add render context sub module to core (enums, traits, basis struct)
// *[*] Move Render Context related stuff to render context module
// *[*] Move render shapes to folders
// *[ ] Implement render shapes: Rectangle, Text, Image, Canvas
// *[ ] FontBuilder from RenderContext -> build from configuration, build frome file. Store fonts.
// *[ ] Canvas widgets needs something like instruction builder for the user
// *[ ] Think about how to implement shadows
// *[ ] Adjust Template to work with new render objects e.g. .with_render_shape(RectangleBuilder::new())
// *[ ] Store render shapes in ecs
// *[*] Implement RenderContext for OrbClient
// *[*] Cache images
// *[ ] Remove old Renderer trait
// *[ ] Adjust Render system to work with render context trait
// *[ ] Write tests for e.g. render shapes
// *[ ] Test everthing
// *[ ] Render cache, store for each shape a layer. Layer part of pixel buffer with shape. Store layer id in shape. 
// *[ ] Dirty flag for shapes. Only dirty shape will redraw. All other will draw by its layers. Dirty after any change and at startup.
// *[ ] MR
//
// Step Two Expand Window
//
// *[ ] is_resizable
// *[ ] min_size
// *[ ] max_size
// *[ ] fix position on windows
// *[ ] Eventuelly refactor backend struct
// *[ ] Test everything
// *[ ] MR
//
// Step Three Web backend
//
// *[ ] implement web backend
// *[ ] Test everything
// *[ ] MR
//
// Step Four switch to OrbGL for OrbClient backend
//
// *[ ] convert more drawing functions to OrbGl in OrbClient backend
// *[ ] Test everything
// *[ ] MR
//
// Optional
//
// *[ ] Switch to winit for default client

