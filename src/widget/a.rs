use std::fmt::{Display, Write};

use crate::{
    html_sanitize,
    widget::{Element, Widget},
};

pub struct A<'a> {
    href: String,
    content: Element<'a>,
}

impl<'a> A<'a> {
    pub fn new(content: impl Into<Element<'a>>) -> A<'a> {
        A {
            href: String::new(),
            content: content.into(),
        }
    }

    pub fn href(mut self, href: impl Display) -> Self {
        self.href = href.to_string();
        self
    }
}

impl<'a> Widget for A<'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<a href=\"{}\">", html_sanitize(&self.href))?;
        self.content.html(f)?;
        write!(f, "</a>")
    }
}
