use crate::widget::{ContextElement, ToElement, Widget};
use std::{borrow::Cow, fmt::Write};

pub struct Details<'a, Context> {
    name: Option<Cow<'a, str>>,
    summary: Option<ContextElement<'a, Context>>,
    content: ContextElement<'a, Context>,
}

impl<'a, Context> Details<'a, Context> {
    pub fn new(content: impl ToElement<'a, Context>) -> Self {
        Details {
            name: None,
            summary: None,
            content: content.to_element(),
        }
    }

    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn summary(mut self, summary: impl ToElement<'a, Context>) -> Self {
        self.summary = Some(summary.to_element());
        self
    }

    pub fn content(mut self, content: impl ToElement<'a, Context>) -> Self {
        self.content = content.to_element();
        self
    }
}

impl<Context> Widget<Context> for Details<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<details")?;
        if let Some(name) = &self.name {
            write!(f, " name=\"{}\"", name)?;
        }
        write!(f, "><summary>")?;
        if let Some(summary) = &self.summary {
            summary.html(f)?;
        }
        write!(f, "</summary>")?;
        self.content.html(f)?;
        write!(f, "</details>")
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut crate::style::Stylesheet) {
        if let Some(summary) = &self.summary {
            summary.style(theme, stylesheet);
        }
        self.content.style(theme, stylesheet);
    }
}
