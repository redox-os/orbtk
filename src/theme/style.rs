use crate::theme::{CloneCell, Selector};

pub trait Style {
    fn selector(&self) -> &CloneCell<Selector>;

    fn class<S: Into<String>>(&self, class: S) -> &Self {
        self.selector().set(self.selector().get().class(class));
        self
    }

    fn without_class<S: Into<String>>(&self, class: S) -> &Self {
        self.selector()
            .set(self.selector().get().without_class(class));
        self
    }

    fn pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self {
        self.selector()
            .set(self.selector().get().pseudo_class(pseudo_class));
        self
    }

    fn without_pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self {
        self.selector()
            .set(self.selector().get().without_pseudo_class(pseudo_class));
        self
    }
}
