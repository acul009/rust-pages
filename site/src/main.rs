use std::{fs::File, io::Write};

mod navbar;

use rust_pages::{
    div, nav,
    style::{Style, Stylesheet},
    ul,
    widget::{Component, ContextElement, ToElement, Widget, a},
};

use crate::navbar::NavBar;

fn main() {
    let mut html = String::new();
    let mut stylesheet = Stylesheet::new();
    let tester = ContextElement::<()>::new(Tester::new());
    tester.html(&mut html).unwrap();
    tester.style(&mut stylesheet);
    let mut file = File::create("output.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
    println!("{}", stylesheet.to_css());
}

pub struct Tester {}

impl Tester {
    fn new() -> Self {
        Tester {}
    }
}

impl Component for Tester {
    fn view(&self) -> rust_pages::widget::ContextElement<'_, Self> {
        div![NavBar::new(), div!["Test"].class("myclass")].to_element()
    }

    fn style(&self) -> Vec<rust_pages::style::Style<Self>> {
        vec![Style::new(".myclass").property("background", "red")]
    }
}
