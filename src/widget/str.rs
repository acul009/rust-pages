use crate::{html_sanitize, widget::Widget};
use std::fmt::Write;

impl Widget for &str {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "{}", html_sanitize(self))
    }
}
