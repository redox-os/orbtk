
use orbgl_api::Canvas;

use crate::event::*;

pub trait WindowAdapter {
    fn render(&mut self, _canvas: &mut Canvas) {}
    fn update(&mut self) {}
    fn resize(&mut self, _width: f64, _height: f64) {}
    fn mouse(&mut self, _x: f64, _y: f64) {}
    fn mouse_event(&mut self, _event: MouseEvent) {}
    fn key_event(&mut self, _event: KeyEvent) {}
    fn quite_event(&mut self) {}
}

pub trait WindowShell {
    fn adapter(&mut self) -> Option<&mut WindowAdapter>;
    fn drain_events(&mut self);
}

pub trait ShellRunner {
    fn run(&mut self);
}

pub trait Updater {
    fn update(&mut self);
}
