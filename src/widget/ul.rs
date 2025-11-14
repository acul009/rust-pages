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
            write!(f, "<li>")?;
            element.html(f)?;
            write!(f, "</li>")?;
        }
        write!(f, "</ul>")?;
        Ok(())
    }

    fn style(&self, _theme: &dyn crate::theme::Theme, stylesheet: &mut crate::style::Stylesheet) {
        for child in &self.children {
            child.style(_theme, stylesheet);
        }
    }
}
