//! This module contains a platform specific implementation of the window shell.

lazy_static! {
    pub static ref CONSOLE: Console = Console;
}

pub struct Console;

impl Console {
    pub fn time(&self, _name: impl Into<String>) {
        // js! {
        //     console.time(@{&name.into()})
        // }
    }

    pub fn time_end(&self, _name: impl Into<String>) {
        // js! {
        //     console.timeEnd(@{&name.into()})
        // }
    }

    pub fn log(&self, message: impl Into<String>) {
        #[cfg(feature = "log")]
        js! {
            console.log(@{&message.into()});
        }
    }
}
