use std::{collections::HashMap, sync::Mutex, time::Instant};

lazy_static! {
    pub static ref CONSOLE: Console = Console {
        instants: Mutex::new(HashMap::new())
    };
}

pub struct Console {
    instants: Mutex<HashMap<String, Instant>>,
}

impl Console {
    pub fn time(&self, name: impl Into<String>) {
        self.instants
            .lock()
            .unwrap()
            .insert(name.into(), Instant::now());
    }

    pub fn time_end(&self, name: impl Into<String>) {
        if let Some((_k, _v)) = self.instants.lock().unwrap().remove_entry(&name.into()) {
            #[cfg(feature = "log")]
            println!("{} {}ms - timer ended", _k, _v.elapsed().as_millis());
        }
    }

    #[allow(unused_variables)]
    pub fn log(&self, message: impl Into<String>) {
        #[cfg(feature = "log")]
        println!("{}", message.into());
    }
}
