// use orbfont;
use std::cell::Cell;
use std::sync::{mpsc::Receiver, mpsc::Sender, Arc};

use orbclient::{Color, Window as OrbWindow};
use orbclient::{Mode, Renderer as OrbRenderer};

use {
    Backend, EventManager, MouseButton, MouseEvent, Rect, RenderContainer, Renderer,
    Selector, SystemEvent, Theme, WindowEvent
};

pub struct OrbitalBackend {
    inner: OrbWindow,
    theme: Arc<Theme>,
    mouse_buttons: (bool, bool, bool),
    event_sender: Option<Sender<EventManager>>,
    render_receiver: Option<Receiver<Vec<RenderContainer>>>,
    running: bool,
}

impl Renderer for OrbWindow {
    fn render(&mut self, theme: &Arc<Theme>) {
        // render window background
        self.set(theme.color("background", &"window".into()));
    }

    fn render_rectangle(
        &mut self,
        bounds: &Rect,
        selector: &Selector,
        offset: (i32, i32),
        theme: &Arc<Theme>,
    ) {
        let b_r = theme.uint("border-radius", selector);

        let fill = theme.color("background", selector);

        self.rounded_rect(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
            b_r,
            true,
            fill,
        );

        if theme.uint("border-width", selector) > 0 {
            let border_color = theme.color("border-color", selector);

            self.rounded_rect(
                bounds.x + offset.0,
                bounds.y + offset.1,
                bounds.width,
                bounds.height,
                b_r,
                false,
                border_color,
            );
        }
    }

    fn render_text(
        &mut self,
        text: &str,
        bounds: &Rect,
        selector: &Selector,
        offset: (i32, i32),
        theme: &Arc<Theme>,
    ) {
        // if let Some(font) = &self.font {
        //     let line = font.render(text, 64.0);
        //     line.draw(&mut self.inner, 20, 20, Color::rgb(0, 0, 0));
        // } else {
        let rect = Rect::new(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
        );
        let mut current_rect = Rect::new(
            bounds.x + offset.0,
            bounds.y + offset.1,
            bounds.width,
            bounds.height,
        );
        let x = rect.x;

        for c in text.chars() {
            if c == '\n' {
                current_rect.x = x;
                current_rect.y += 16;
            } else {
                if current_rect.x + 8 <= rect.x + rect.width as i32
                    && current_rect.y + 16 <= rect.y + rect.height as i32
                {
                    self.char(
                        current_rect.x,
                        current_rect.y,
                        c,
                        theme.color("color", selector),
                    );
                }
                current_rect.x += 8;
            }
        }
        // }
    }
}

impl OrbitalBackend {
    pub fn new(theme: Arc<Theme>) -> OrbitalBackend {
        OrbitalBackend {
            inner: OrbWindow::new_flags(0, 0, 0, 0, "", &[]).unwrap(),
            theme,
            mouse_buttons: (false, false, false),
            event_sender: None,
            render_receiver: None,
            running: false,
        }
    }
}

impl OrbRenderer for OrbitalBackend {
    fn width(&self) -> u32 {
        self.inner.width()
    }

    fn height(&self) -> u32 {
        self.inner.height()
    }

    fn data(&self) -> &[Color] {
        self.inner.data()
    }

    fn data_mut(&mut self) -> &mut [Color] {
        self.inner.data_mut()
    }

    fn sync(&mut self) -> bool {
        self.inner.sync()
    }

    fn mode(&self) -> &Cell<Mode> {
        &self.inner.mode()
    }

    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        // if let Some(ref font) = self.font {
        //     let mut buf = [0; 4];
        //     font.render(&c.encode_utf8(&mut buf), 16.0)
        //         .draw(&mut self.inner, x, y, color)
        // } else {
        self.inner.char(x, y, c, color);
        // }
    }
}

impl Drop for OrbitalBackend {
    fn drop(&mut self) {
        self.inner.sync();
    }
}

impl Backend for OrbitalBackend {
    fn drain_events(&mut self) {
        self.inner.sync();

        let mut event_manager = EventManager::default();
        let mut running = self.running;

        for event in self.inner.events() {
            match event.to_option() {
                orbclient::EventOption::Mouse(mouse) => {
                    event_manager.register_event(MouseEvent::Move((mouse.x, mouse.y)));
                }
                orbclient::EventOption::Button(button) => {
                    if !button.left && !button.middle && !button.right {
                        let button = {
                            if self.mouse_buttons.0 {
                                MouseButton::Left
                            } else if self.mouse_buttons.1 {
                                MouseButton::Middle
                            } else {
                                MouseButton::Right
                            }
                        };
                        event_manager.register_event(MouseEvent::Up(button))
                    } else {
                        let button = {
                            if button.left {
                                MouseButton::Left
                            } else if button.middle {
                                MouseButton::Middle
                            } else {
                                MouseButton::Right
                            }
                        };
                        event_manager.register_event(MouseEvent::Down(button))
                    }

                    self.mouse_buttons = (button.left, button.middle, button.right);
                }
                orbclient::EventOption::Resize(resize) => {
                    event_manager.register_event(WindowEvent::Resize((resize.width, resize.height)));
                },
                orbclient::EventOption::Quit(_quit_event) => {
                    event_manager.register_event(SystemEvent::Quit);
                    running = false;
                }
                _ => {}
            }
        }

        if let Some(event_sender) = &self.event_sender {
            if let Err(err) = event_sender.send(event_manager) {
                println!("Orbital Backend: {}", err);
            }
        }

        self.running = running;
    }

    fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    fn bounds(&mut self, bounds: &Rect) {
        self.inner.set_pos(bounds.x, bounds.y);
        self.inner.set_size(bounds.width, bounds.height);
    }

    fn run(&mut self) {
        self.running = true;
        while self.running {
            self.inner.render(&self.theme);

            if let Some(render_receiver) = &self.render_receiver {
                if let Ok(render_containers) = render_receiver.recv() {
                    for container in render_containers {
                        container.render_object.render(
                            &container.bounds,
                            &container.selector,
                            &mut self.inner,
                            container.offset,
                            &self.theme,
                            container.content,
                        );
                    }
                }
            }

            self.drain_events();
        }
    }

    fn event_sender(&mut self, event_sender: Sender<EventManager>) {
        self.event_sender = Some(event_sender);
    }

    fn render_receiver(&mut self, render_receiver: Receiver<Vec<RenderContainer>>) {
        self.render_receiver = Some(render_receiver);
    }
}
