

use crate::{Update, Runner};

impl Runner for Update {
    fn run(&mut self) {
        loop {
            if !(self.update)() {
                break;
            }
        }
    }
}