use crate::theme::{CloneCell, Selector};

pub trait Style {
    fn selector(&self) -> &CloneCell<Selector>;

    fn with_class<S: Into<String>>(&self, class: S) -> &Self {
        self.selector().set(self.selector().get().with_class(class));
        self
    }

    fn without_class<S: Into<String>>(&self, class: S) -> &Self {
        self.selector()
            .set(self.selector().get().without_class(class));
        self
    }

    fn with_pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self {
        self.selector()
            .set(self.selector().get().with_pseudo_class(pseudo_class));
        self
    }

    fn without_pseudo_class<S: Into<String>>(&self, pseudo_class: S) -> &Self {
        self.selector()
            .set(self.selector().get().without_pseudo_class(pseudo_class));
        self
    }
}
