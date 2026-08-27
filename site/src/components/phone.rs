use rust_pages::{a, widget::Component};

use crate::components::browser_content::BrowserContent;

pub struct Phone;

pub const PHONE: &str = "08633 / 977969 - 0";

impl Component for Phone {
    fn view<'a>(&'a self) -> impl rust_pages::widget::ToElement<'a, Self> {
        BrowserContent::new(
            a(PHONE).href(format!("tel:+49{}", &PHONE[1..].replace([' ', '/'], ""))),
        )
    }

    fn style(&self, _theme: &dyn rust_pages::theme::Theme) -> Vec<rust_pages::style::Style<Self>> {
        vec![]
    }
}
