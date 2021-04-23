use std::marker::*;

use orbclient::{Color, Renderer};

use orbtk_core::*;

use crate::*;

/// Defines a top-level window on the screen.
pub struct Window<S>
where
    S: Default + Clone + PartialEq,
{
    pub(crate) inner: orbclient::Window,
    pub(crate) ui: Ui<S>,
}

impl<S> Window<S>
where
    S: Default + Clone + PartialEq,
{
    pub fn create(state: S) -> WindowBuilder<S> {
        WindowBuilder::new(state)
    }

    /// Drain events and propagate the events to the shell.
    ///
    /// If it return `false` the window should be closed.
    pub fn drain_events(&mut self) -> bool {
        for event in self.inner.events() {
            println!("{:?}", event.to_option());
            // if let Some(shell) = &mut self.shell {
            match event.to_option() {
                orbclient::EventOption::Quit(_) => {
                    println!("close");
                    return false;
                }
                // orbclient::EventOption::Key(e) => {
                //     shell.key(e.scancode, e.pressed);
                // }
                // orbclient::EventOption::TextInput(e) => {
                //     shell.text_input(e.character);
                // }
                // orbclient::EventOption::Mouse(e) => shell.mouse(e.x, e.y),
                // orbclient::EventOption::MouseRelative(_) => println!("no yet implemented"),
                // orbclient::EventOption::Button(e) => {
                //     shell.mouse_button(e.left, e.middle, e.right);
                // }
                // // orbclient::EventOption::Scroll(e) => self.shell.scroll(e.x as f64, e.y as f64),
                // // orbclient::EventOption::Focus(e) => self.shell.active(e.focused),
                // // orbclient::EventOption::Move(e) => self.shell.moved(e.x as f64, e.y as f64),
                // orbclient::EventOption::Resize(_) => {
                //     shell.set_display(
                //         TinySkiaDisplay::new(self.inner.width(), self.inner.height()).unwrap(),
                //     );
                // }
                // orbclient::EventOption::Screen(_) => println!("no yet implemented"),
                // orbclient::EventOption::Clipboard(_) => println!("no yet implemented"),
                // orbclient::EventOption::ClipboardUpdate(_) => println!("no yet implemented"),
                // orbclient::EventOption::Drop(_) => println!("no yet implemented"),
                // orbclient::EventOption::Hover(_) => println!("no yet implemented"),
                // orbclient::EventOption::Unknown(_) => println!("no yet implemented"),
                // orbclient::EventOption::None => println!("no yet implemented"),
                _ => {}
            }
            // }
        }

        true
    }

    /// Swaps the buffer of the orbclient window with the given data. If the len of the source buffer does not match the current
    /// len of the window frame buffer and error will be returned.
    pub fn swap_frame_buffer(&mut self, data: &mut [orbclient::Color]) -> Result<(), Error> {
        if data.len() != self.inner.data_mut().len() {
            return Err(Error::WrongFrameBufferSize);
        }

        self.inner.data_mut().copy_from_slice(data);
        self.inner.sync();

        Ok(())
    }
}

impl<S> Runner for Window<S>
where
    S: Default + Clone + PartialEq,
{
    /// Runs the inner logic of the window.
    fn run(&mut self) -> Result<bool, Error> {
        if !self.drain_events() {
            return Ok(false);
        }

        // if let Some(shell) = &mut self.shell {
        //     shell.run().map_err(|_| Error::ShellRunError)?;

        //     if let Some(display) = shell.display_mut() {
        //         let len = self.inner.data().len() * std::mem::size_of::<orbclient::Color>();
        //         let color_data = unsafe {
        //             std::slice::from_raw_parts_mut(
        //                 self.inner.data_mut().as_mut_ptr() as *mut u8,
        //                 len,
        //             )
        //         };

        //         if color_data.len() == display.data().len() {
        //             display.flip(color_data);
        //         }
        //     }
        //     self.inner.sync();
        // }

        Ok(true)
    }
}
