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

use crate::html_sanitize;

pub trait Renderable {
    fn html(&self, f: &mut String) -> std::fmt::Result;
}

pub trait Component {
    fn view(&self) -> ContextElement<'_, Self>;
    fn style(&self) -> Option<&'static str>;
}

pub trait Widget<Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result;
}

pub trait WidgetWrapper {
    fn html(&self, f: &mut String) -> std::fmt::Result;
}

impl<'a, Context> WidgetWrapper for Box<dyn Widget<Context> + 'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        Widget::html(self.deref(), f)
    }
}

pub trait ComponentWrapper {
    fn view(&self) -> ElementWrapper<'_>;
    fn style(&self) -> Option<&'static str>;
}

pub enum ContextElement<'a, Context: ?Sized> {
    Widget(Box<dyn Widget<Context> + 'a>),
    Component(Box<dyn ComponentWrapper + 'a>),
}

impl<'a, Context> ContextElement<'a, Context> {
    pub fn new<W: Widget<Context> + 'a>(widget: W) -> Self {
        ContextElement::Widget(Box::new(widget))
    }

    pub fn html(&self, f: &mut String) -> std::fmt::Result {
        match self {
            ContextElement::Widget(widget) => widget.html(f),
            ContextElement::Component(component) => component.view().html(f),
        }
    }
}

pub enum ElementWrapper<'a> {
    Widget(Box<dyn WidgetWrapper + 'a>),
    Component(Box<dyn ComponentWrapper + 'a>),
}

impl<'a> ElementWrapper<'a> {
    pub fn new<Context: 'a>(element: ContextElement<'a, Context>) -> Self {
        match element {
            ContextElement::Widget(widget) => ElementWrapper::Widget(Box::new(widget)),
            ContextElement::Component(component) => ElementWrapper::Component(component),
        }
    }

    pub fn html(&'a self, f: &mut String) -> std::fmt::Result {
        match self {
            ElementWrapper::Widget(widget) => widget.html(f),
            ElementWrapper::Component(component) => component.view().html(f),
        }
    }
}

pub trait ToElement<'a, Context> {
    fn to_element(self) -> ContextElement<'a, Context>;
}

impl<'a, Context, W: Widget<Context> + 'a> ToElement<'a, Context> for W {
    fn to_element(self) -> ContextElement<'a, Context> {
        ContextElement::Widget(Box::new(self))
    }
}

pub trait Class<Context> {
    // basically just appends the type id to the class name
    fn resolve(&self) -> String;
}

impl<Context> Class<Context> for &str {
    fn resolve(&self) -> String {
        html_sanitize(format!("{}-{}", std::any::type_name::<Context>(), self))
    }
}
