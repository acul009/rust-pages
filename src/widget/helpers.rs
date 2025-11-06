use std::borrow::Cow;

use crate::widget::{Element, a::A, p::P};

#[macro_export]
macro_rules! div {
    () => {
        $crate::widget::Div::new()
    };
    ($($child:expr),*) => {
        $crate::widget::div::Div::with_children([$($crate::widget::Element::from($child)),+])
    };
}

#[macro_export]
macro_rules! nav {
    () => {
        $crate::widget::Div::new()
    };
    ($($child:expr),*) => {
        $crate::widget::nav::Nav::with_children([$($crate::widget::Element::from($child)),+])
    };
}

#[macro_export]
macro_rules! ul {
    () => {
        $crate::widget::Ul::new()
    };
    ($($child:expr),*) => {
        $crate::widget::ul::Ul::with_children([$($crate::widget::Element::from($child)),+])
    };
}

pub fn a<'a>(content: impl Into<Element<'a>>) -> A<'a> {
    A::new(content)
}

pub fn p<'a>(text: impl Into<Cow<'a, str>>) -> P<'a> {
    P::new(text)
}
