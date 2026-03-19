use crate::widget::{
    ToElement,
    a::A,
    details::Details,
    picture::{Handle, Picture},
    raw_html::RawHtml,
};

#[macro_export]
macro_rules! div {
    () => {
        $crate::widget::container::Container::new("div")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("div", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! nav {
    () => {
        $crate::widget::container::Container::new("nav")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("nav", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! span {
    () => {
        $crate::widget::container::Container::new("span")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("span", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! main {
    () => {
        $crate::widget::container::Container::new("main")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("main", [$($crate::widget::ToElement::to_element($child)),+])
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

pub fn raw_html<'a>(html: &'a str) -> RawHtml<'a> {
    RawHtml::new(html)
}

pub fn picture<'a>(handle: &'a Handle) -> Picture<'a> {
    Picture::new(handle)
}

#[macro_export]
macro_rules! p {
    () => {
        $crate::widget::container::Container::new("p")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("p", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! h1 {
    () => {
        $crate::widget::container::Container::new("h1")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("h1", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! h2 {
    () => {
        $crate::widget::container::Container::new("h2")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("h2", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! h3 {
    () => {
        $crate::widget::container::Container::new("h3")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("h3", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! h4 {
    () => {
        $crate::widget::container::Container::new("h4")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("h4", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! h5 {
    () => {
        $crate::widget::container::Container::new("h5")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("h5", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! h6 {
    () => {
        $crate::widget::container::Container::new("h6")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("h6", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

#[macro_export]
macro_rules! b {
    () => {
        $crate::widget::container::Container::new("b")
    };
    ($($child:expr),*) => {
        $crate::widget::container::Container::with_children("b", [$($crate::widget::ToElement::to_element($child)),+])
    };
}

pub fn br() -> crate::widget::br::Br {
    crate::widget::br::Br::new()
}
