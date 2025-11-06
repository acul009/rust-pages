use crate::widget::{Element, Widget};
use std::fmt::Write;

pub struct Details<'a> {
    summary: Option<Element<'a>>,
    content: Element<'a>,
}

pub fn details<'a>(content: impl Into<Element<'a>>) -> Details<'a> {
    Details::new(content.into())
}

impl<'a> Details<'a> {
    pub fn new(content: Element<'a>) -> Self {
        Details {
            summary: None,
            content,
        }
    }

    pub fn summary(mut self, summary: impl Into<Element<'a>>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn content(mut self, content: impl Into<Element<'a>>) -> Self {
        self.content = content.into();
        self
    }
}

impl<'a> Widget for Details<'a> {
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
