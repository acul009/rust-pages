use std::borrow::Cow;

use crate::{
    style::Class,
    widget::{ContextElement, Widget},
};

pub struct Container<'a, Context> {
    tag: Cow<'a, str>,
    children: Vec<ContextElement<'a, Context>>,
    class: Option<String>,
}

impl<'a, Context> Container<'a, Context> {
    pub fn new(tag: impl Into<Cow<'a, str>>) -> Self {
        Self {
            tag: tag.into(),
            children: Vec::new(),
            class: None,
        }
    }

    pub fn with_children(
        tag: impl Into<Cow<'a, str>>,
        children: impl IntoIterator<Item = ContextElement<'a, Context>>,
    ) -> Self {
        Self {
            tag: tag.into(),
            children: children.into_iter().collect(),
            class: None,
        }
    }

    pub fn child(mut self, child: impl Into<ContextElement<'a, Context>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn class(mut self, class: impl Class<Context>) -> Self {
        self.class = Some(class.resolve());
        self
    }
}

impl<Context> Widget<Context> for Container<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        f.push('<');
        f.push_str(&self.tag);
        if let Some(class) = &self.class {
            f.push_str(" class=\"");
            f.push_str(class);
            f.push_str("\"");
        }
        f.push('>');
        for child in &self.children {
            child.html(f)?;
        }
        f.push_str("</");
        f.push_str(&self.tag);
        f.push('>');
        Ok(())
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut crate::style::Stylesheet) {
        for child in &self.children {
            child.style(theme, stylesheet);
        }
    }
}
