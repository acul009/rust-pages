use crate::widget::Widget;
use std::fmt::Write;

pub struct Br {}

impl Br {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Context> Widget<Context> for Br {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<br>")
    }

    fn style(&self, _theme: &dyn crate::theme::Theme, _stylesheet: &mut crate::style::Stylesheet) {}
}
