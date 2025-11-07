use std::borrow::Cow;
use std::fmt::Write;

use crate::html_sanitize;
use crate::widget::Widget;

pub struct P<'a> {
    text: Cow<'a, str>,
}

impl<'a> P<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self { text: text.into() }
    }
}

impl<'a, Context> Widget<Context> for P<'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<p>{}</p>", html_sanitize(self.text.as_ref()))
    }
}
