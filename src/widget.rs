pub mod a;
pub mod details;
pub mod div;
mod helpers;
pub mod nav;
pub mod p;
pub mod str;
pub mod ul;

use std::ops::Deref;

pub use helpers::*;
use itertools::Itertools;

use crate::style::{Style, Stylesheet};

pub trait Component {
    fn view(&self) -> ContextElement<'_, Self>;
    fn style(&self) -> Vec<Style<Self>>;
}

pub trait Widget<Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result;
    fn style(&self, stylesheet: &mut crate::style::Stylesheet);
}

impl<Context, C: Component + 'static> Widget<Context> for C {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        Component::view(self).html(f)
    }

    fn style(&self, stylesheet: &mut Stylesheet) {
        if !stylesheet.contains::<C>() {
            let styles = Component::style(self);
            stylesheet.add_styles(&styles);
        }
        Component::view(self).style(stylesheet);
    }
}

pub trait WidgetWrapper {
    fn html(&self, f: &mut String) -> std::fmt::Result;
}

impl<'a, Context> WidgetWrapper for Box<dyn Widget<Context> + 'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        Widget::html(self.deref(), f)
    }
}

pub struct ContextElement<'a, Context: ?Sized> {
    widget: Box<dyn Widget<Context> + 'a>,
}

impl<'a, Context> ContextElement<'a, Context> {
    pub fn new<W: Widget<Context> + 'a>(widget: W) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn html(&self, f: &mut String) -> std::fmt::Result {
        self.widget.html(f)
    }

    pub fn style(&self, stylesheet: &mut Stylesheet) {
        self.widget.style(stylesheet);
    }
}

pub trait ToElement<'a, Context> {
    fn to_element(self) -> ContextElement<'a, Context>;
}

impl<'a, Context, W: Widget<Context> + 'a> ToElement<'a, Context> for W {
    fn to_element(self) -> ContextElement<'a, Context> {
        ContextElement::new(self)
    }
}
