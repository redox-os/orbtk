use std::{collections::HashSet, ops::Add};

#[derive(Clone, Debug)]
pub enum SelectorRelation {
    Ancestor(SelectorValue),
    Parent(SelectorValue),
}

/// Describes the specificity of a selector.
///
/// The indexes are as follows:
/// 0 - number of IDs (most important)
/// 1 - number of classes and pseudo-classes
/// 2 - number of elements (least important)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity([u8; 3]);

impl Add<Self> for Specificity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Specificity([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

#[derive(Debug, Default)]
pub struct SelectorValue {
    pub id: Option<String>,
    pub element: Option<String>,
    pub classes: HashSet<String>,
    pub pseudo_classes: HashSet<String>,
    pub relation: Option<Box<SelectorRelation>>,
    pub dirty: bool,
}

impl SelectorValue {
    pub fn new() -> Self {
        SelectorValue {
            id: None,
            element: None,
            classes: HashSet::new(),
            pseudo_classes: HashSet::new(),
            relation: None,
            dirty: true,
        }
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn specificity(&self) -> Specificity {
        let s = Specificity([
            if self.id.is_some() { 1 } else { 0 },
            (self.classes.len() + self.pseudo_classes.len()) as u8,
            if self.element.is_some() { 1 } else { 0 },
        ]);

        if let Some(ref relation) = self.relation {
            match **relation {
                SelectorRelation::Ancestor(ref x) | SelectorRelation::Parent(ref x) => {
                    return x.specificity() + s;
                }
            }
        }

        s
    }

    pub fn matches(&self, other: &SelectorValue) -> bool {
        if self.id.is_some() && self.id != other.id {
            return false;
        }

        if self.element.is_some() && self.element != other.element {
            return false;
        }

        if !other.classes.is_superset(&self.classes) {
            return false;
        }

        if !other.pseudo_classes.is_superset(&self.pseudo_classes) {
            return false;
        }

        true
    }

    pub fn with<S: Into<String>>(mut self, element: S) -> Self {
        self.element = Some(element.into());
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn class<S: Into<String>>(mut self, class: S) -> Self {
        self.classes.insert(class.into());
        self
    }

    pub fn without_class<S: Into<String>>(mut self, class: S) -> Self {
        self.classes.remove(&class.into());
        self
    }

    pub fn pseudo_class<S: Into<String>>(mut self, pseudo_class: S) -> Self {
        self.pseudo_classes.insert(pseudo_class.into());
        self
    }

    pub fn without_pseudo_class<S: Into<String>>(mut self, pseudo_class: S) -> Self {
        self.pseudo_classes.remove(&pseudo_class.into());
        self
    }
}

impl PartialEq for SelectorValue {
    fn eq(&self, other: &SelectorValue) -> bool {
        self.id == other.id
    }
}

impl SelectorValue {
    pub fn is_empty(&self) -> bool {
        self.element.is_none()
            && self.id.is_none()
            && self.classes.is_empty()
            && self.pseudo_classes.is_empty()
    }
}

impl Clone for SelectorValue {
    fn clone(&self) -> Self {
        SelectorValue {
            id: self.id.clone(),
            element: self.element.clone(),
            classes: self.classes.clone(),
            pseudo_classes: self.pseudo_classes.clone(),
            relation: self.relation.clone(),
            dirty: self.dirty,
        }
    }
}

property!(
    /// `Selector` describes the css selector of a widget.
    Selector(SelectorValue)
);