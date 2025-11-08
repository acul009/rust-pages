use crate::widget::{ContextElement, Widget};
use std::fmt::Write;

pub struct Nav<'a, Context> {
    children: Vec<ContextElement<'a, Context>>,
}

impl<'a, Context> Nav<'a, Context> {
    pub fn new() -> Self {
        Nav {
            children: Vec::new(),
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = ContextElement<'a, Context>>) -> Self {
        Self {
            children: children.into_iter().collect(),
        }
    }

    pub fn child(mut self, child: impl Into<ContextElement<'a, Context>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<Context> Widget<Context> for Nav<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<nav>")?;
        for item in &self.children {
            item.html(f)?;
        }
        write!(f, "</nav>")
    }

    fn style(&self, stylesheet: &mut crate::style::Stylesheet) {
        for child in &self.children {
            child.style(stylesheet);
        }
    }
}
