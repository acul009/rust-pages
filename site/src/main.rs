use std::{fs::File, io::Write};

mod navbar;

use rust_pages::{
    div, nav, render, ul,
    widget::{Component, Element, ToElement, a, details::details},
};

use crate::navbar::NavBar;

fn main() {
    let html = render(div![navbar(), "Hello 2"]);

    let mut file = File::create("output.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
    println!("{}", html);
}

pub struct Page {}

impl Component for Page {
    fn view(&self) -> rust_pages::widget::ContextElement<'_, Self> {
        div!["Test", NavBar::new()].to_element()
    }

    fn style(&self) -> Option<&'static str> {
        todo!()
    }
}
