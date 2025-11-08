mod index;
mod navbar;

use rust_pages::{
    builder::SiteBuilder,
    div, nav,
    style::{Style, Stylesheet},
    widget::{Component, ContextElement, ToElement, Widget, a},
};

use crate::{index::Index, navbar::NavBar};

fn main() {
    let builder = SiteBuilder::new().title("Rahn-IT");
    builder.build_page(Index).unwrap();
}

pub struct Tester {}

impl Tester {
    fn new() -> Self {
        Tester {}
    }
}

impl Component for Tester {
    fn view(&self) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![NavBar::new(), div!["Test"].class("myclass")]
    }

    fn style(&self) -> Vec<rust_pages::style::Style<Self>> {
        vec![Style::new(".myclass").property("background", "red")]
    }
}
