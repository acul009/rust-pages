use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

use crate::{
    html_sanitize,
    widget::{ContextElement, ToElement, Widget},
};

pub struct A<'a, Context> {
    href: Cow<'a, str>,
    content: ContextElement<'a, Context>,
}

impl<'a, Context> A<'a, Context> {
    pub fn new(content: impl ToElement<'a, Context>) -> A<'a, Context> {
        A {
            href: Cow::Borrowed(""),
            content: content.to_element(),
        }
    }

    pub fn href(mut self, href: impl Into<Cow<'a, str>>) -> Self {
        self.href = href.into();
        self
    }
}

impl<Context> Widget<Context> for A<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<a href=\"{}\">", html_sanitize(&self.href))?;
        self.content.html(f)?;
        write!(f, "</a>")
    }
}
