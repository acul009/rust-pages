use std::{borrow::Cow, fmt::Write};

use crate::{
    html_sanitize,
    style::{Class, Stylesheet},
    widget::{ContextElement, ToElement, Widget},
};

pub struct A<'a, Context> {
    href: Cow<'a, str>,
    content: ContextElement<'a, Context>,
    class: Option<String>,
}

impl<'a, Context> A<'a, Context> {
    pub fn new(content: impl ToElement<'a, Context>) -> A<'a, Context> {
        A {
            href: Cow::Borrowed(""),
            content: content.to_element(),
            class: None,
        }
    }

    pub fn href(mut self, href: impl Into<Cow<'a, str>>) -> Self {
        self.href = href.into();
        self
    }

    pub fn class(mut self, class: impl Class<Context>) -> Self {
        self.class = Some(class.resolve());
        self
    }
}

impl<Context> Widget<Context> for A<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<a href=\"{}\"", html_sanitize(&self.href))?;
        if let Some(class) = &self.class {
            write!(f, " class=\"{}\"", class)?;
        }
        write!(f, ">")?;
        self.content.html(f)?;
        write!(f, "</a>")
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut Stylesheet) {
        self.content.style(theme, stylesheet);
    }
}
