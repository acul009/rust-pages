use crate::{html_sanitize, widget::Widget};
use std::fmt::Write;

impl<Context> Widget<Context> for &str {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "{}", html_sanitize(self))
    }

    fn style(&self, _stylesheet: &mut crate::style::Stylesheet) {}
}
