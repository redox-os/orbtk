use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Property {
    inner: Vec<Expression>,
}

impl Property {
    pub fn brush(&self) -> Option<Brush> {
        let mut brushes = Vec::new();
        for e in self.inner.iter() {
            let brush = match e.brush() {
                Some(b) => b,
                None => {
                    return None;
                }
            };
            brushes.push(brush);
        }
        if brushes.is_empty() {
            None
        } else if brushes.len() == 1 {
            Some(brushes.first().unwrap().clone())
        } else {
            Some(Brush::Stacked(brushes))
        }
    }
}

impl From<&str> for Property {
    fn from(s: &str) -> Property {
        let mut inner = Vec::new();
        let mut chrs = s.chars().peekable();
        while let Some(_) = chrs.peek() {
            while let Some(true) = chrs.peek().map(|x| x.is_whitespace()) {
                chrs.next().unwrap();
            }
            if let Some(',') = chrs.peek() {
                break; // Double comma or comma at property start
            }
            let e = match parse_expression_with_complex(&mut chrs) {
                Some(e) => e,
                None => {
                    break;
                }
            };
            inner.push(e);
            while let Some(true) = chrs.peek().map(|x| x.is_whitespace()) {
                chrs.next().unwrap();
            }
            if let Some(',') = chrs.peek() {
                chrs.next().unwrap();
            }
        }
        Property { inner }
    }
}

impl Default for Property {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}
