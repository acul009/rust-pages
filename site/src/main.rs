use std::{fs::File, io::Write};

mod navbar;

use rust_pages::{
    div, nav, ul,
    widget::{Component, ToElement, a},
};

use crate::navbar::NavBar;

fn main() {
    let mut html = String::new();
    Tester::new().view().html(&mut html).unwrap();
    let mut file = File::create("output.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
    println!("{}", html);
}

pub struct Tester {}

impl Tester {
    fn new() -> Self {
        Tester {}
    }
}

impl Component for Tester {
    fn view(&self) -> rust_pages::widget::ContextElement<'_, Self> {
        div!["Test"].class("myclass").to_element()
    }
}
