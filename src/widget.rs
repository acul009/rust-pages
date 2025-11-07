pub mod a;
pub mod details;
pub mod div;
mod helpers;
pub mod nav;
pub mod p;
pub mod str;
pub mod ul;

use std::{marker::PhantomData, ops::Deref};

pub use helpers::*;
use itertools::Itertools;

use crate::html_sanitize;

pub trait Component {
    fn view(&self) -> ContextElement<'_, Self>;
}

pub trait Widget<Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result;
}

impl<Context, C: Component> Widget<Context> for C {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        Component::view(self).html(f)
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
}

pub trait ToElement<'a, Context> {
    fn to_element(self) -> ContextElement<'a, Context>;
}

impl<'a, Context, W: Widget<Context> + 'a> ToElement<'a, Context> for W {
    fn to_element(self) -> ContextElement<'a, Context> {
        ContextElement::new(self)
    }
}

pub trait Class<Context> {
    // basically just appends the type id to the class name
    fn resolve(&self) -> String;
}

impl<Context> Class<Context> for &str {
    fn resolve(&self) -> String {
        let scope_name = std::any::type_name::<Context>().replace(':', "_");
        self.split_ascii_whitespace()
            .map(|class| format!("{}-{}", scope_name, html_sanitize(class)))
            .join(" ")
    }
}
