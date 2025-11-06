pub mod a;
pub mod details;
pub mod div;
mod helpers;
pub mod nav;
pub mod p;
pub mod str;
pub mod ul;

pub use helpers::*;

pub trait Widget {
    fn html(&self, f: &mut String) -> std::fmt::Result;
}

pub struct Element<'a> {
    widget: Box<dyn Widget + 'a>,
}

impl<'a> Element<'a> {
    pub fn new(widget: impl Widget + 'a) -> Self {
        Element {
            widget: Box::new(widget),
        }
    }

    pub fn html(&self, f: &mut String) -> std::fmt::Result {
        self.widget.html(f)
    }
}

impl<'a, W: Widget + 'a> From<W> for Element<'a> {
    fn from(widget: W) -> Self {
        Element::new(widget)
    }
}
