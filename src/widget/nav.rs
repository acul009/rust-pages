use crate::widget::{Element, Widget};
use std::fmt::Write;

pub struct Nav<'a> {
    children: Vec<Element<'a>>,
}

impl<'a> Nav<'a> {
    pub fn new() -> Self {
        Nav {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = Element<'a>>) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }

    pub fn child(mut self, child: impl Into<Element<'a>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a> Widget for Nav<'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<nav>")?;
        for item in &self.children {
            item.html(f)?;
        }
        write!(f, "</nav>")
    }
}
