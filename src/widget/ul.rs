use crate::widget::{Element, Widget};
use std::fmt::Write;

pub struct Ul<'a> {
    children: Vec<Element<'a>>,
}

impl<'a> Ul<'a> {
    pub fn new() -> Ul<'a> {
        Ul {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = Element<'a>>) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }

    pub fn li(mut self, content: impl Into<Element<'a>>) -> Self {
        self.children.push(content.into());
        self
    }
}

impl Widget for Ul<'_> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<ul>")?;
        for element in &self.children {
            element.html(f)?;
        }
        write!(f, "</ul>")?;
        Ok(())
    }
}
