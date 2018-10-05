use std::rc::Rc;
use std::cell::RefCell;

use {Rect, OrbitalBackend, Backend, Theme, OrbitalBackendRunner};

use orbclient::Window as OrbWindow;

pub fn target_backend(title: &str, bounds: Rect, theme: Theme) -> (Box<OrbitalBackendRunner>, Rc<RefCell<Backend>>) {
    let backend = Rc::new(RefCell::new(OrbitalBackend::new(theme, OrbWindow::new_flags(bounds.x, bounds.y, bounds.width, bounds.height, title, &[]).unwrap())));

    let backend_runner = Box::new(OrbitalBackendRunner { backend: backend.clone(), world: None });

    (backend_runner, backend)
}