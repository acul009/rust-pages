use std::borrow::Cow;

use crate::widget::{ToElement, a::A, details::Details, p::P, raw_html::RawHtml};

#[macro_export]
macro_rules! div {
    () => {
        $crate::widget::div::Div::new()
    };
    ($($child:expr),*) => {
        $crate::widget::div::Div::with_children([$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! nav {
    () => {
        $crate::widget::Div::new()
    };
    ($($child:expr),*) => {
        $crate::widget::nav::Nav::with_children([$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! ul {
    () => {
        $crate::widget::Ul::new()
    };
    ($($child:expr),*) => {
        $crate::widget::ul::Ul::with_children([$($crate::widget::ToElement::to_element($child)),+])
    };
}

pub fn details<'a, Context>(content: impl ToElement<'a, Context>) -> Details<'a, Context> {
    Details::new(content)
}

pub fn a<'a, Context>(content: impl ToElement<'a, Context>) -> A<'a, Context> {
    A::new(content)
}

pub fn p<'a>(text: impl Into<Cow<'a, str>>) -> P<'a> {
    P::new(text)
}

pub fn raw_html<'a>(html: &'a str) -> RawHtml<'a> {
    RawHtml::new(html)
}
