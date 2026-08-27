use rust_pages::{a, widget::Component};

use crate::components::browser_content::BrowserContent;

pub struct Mail;

pub const MAIL: &str = "info@it-rahn.de";

impl Component for Mail {
    fn view<'a>(&'a self) -> impl rust_pages::widget::ToElement<'a, Self> {
        BrowserContent::new(a(MAIL).href(format!("mailto:{}", MAIL)))
    }

    fn style(&self, _theme: &dyn rust_pages::theme::Theme) -> Vec<rust_pages::style::Style<Self>> {
        vec![]
    }
}
