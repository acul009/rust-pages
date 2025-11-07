use crate::widget::{ContextElement, Widget};
use std::fmt::Write;

pub struct Ul<'a, Context> {
    children: Vec<ContextElement<'a, Context>>,
}

impl<'a, Context> Ul<'a, Context> {
    pub fn new() -> Self {
        Ul {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = ContextElement<'a, Context>>) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }

    pub fn li(mut self, content: impl Into<ContextElement<'a, Context>>) -> Self {
        self.children.push(content.into());
        self
    }
}

impl<'a, Context> Widget<Context> for Ul<'a, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<ul>")?;
        for element in &self.children {
            element.html(f)?;
        }
        write!(f, "</ul>")?;
        Ok(())
    }
}
