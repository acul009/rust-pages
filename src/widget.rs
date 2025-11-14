pub mod a;
pub mod container;
pub mod details;
mod helpers;
pub mod p;
pub mod raw_html;
pub mod str;
pub mod ul;

use std::ops::Deref;

pub use helpers::*;

use crate::style::{Style, Stylesheet};

pub trait Component {
    fn view(&self) -> impl crate::widget::ToElement<'_, Self>;
    fn style(&self, theme: &dyn crate::theme::Theme) -> Vec<Style<Self>>;
}

pub trait Widget<Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result;
    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut crate::style::Stylesheet);
}

impl<Context, C: Component + 'static> Widget<Context> for C {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        Component::view(self).to_element().html(f)
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut crate::style::Stylesheet) {
        if !stylesheet.contains::<C>() {
            let styles = Component::style(self, theme);
            stylesheet.add_styles(&styles);
        }
        Component::view(self).to_element().style(theme, stylesheet);
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

    pub fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut Stylesheet) {
        self.widget.style(theme, stylesheet);
    }
}

pub trait ToElement<'a, Context: ?Sized> {
    fn to_element(self) -> ContextElement<'a, Context>;
}

impl<'a, Context, W: Widget<Context> + 'a> ToElement<'a, Context> for W {
    fn to_element(self) -> ContextElement<'a, Context> {
        ContextElement::new(self)
    }
}
