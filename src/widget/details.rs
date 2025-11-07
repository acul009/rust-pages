use crate::widget::{ContextElement, ToElement, Widget};
use std::fmt::Write;

pub struct Details<'a, Context> {
    summary: Option<ContextElement<'a, Context>>,
    content: ContextElement<'a, Context>,
}

impl<'a, Context> Details<'a, Context> {
    pub fn new(content: impl ToElement<'a, Context>) -> Self {
        Details {
            summary: None,
            content: content.to_element(),
        }
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
        write!(f, "<details><summary>")?;
        if let Some(summary) = &self.summary {
            summary.html(f)?;
        }
        write!(f, "</summary>")?;
        self.content.html(f)?;
        write!(f, "</details>")
    }
}
