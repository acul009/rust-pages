use rust_pages::{div, layout::Layout, main, style::Style, widget::ToElement};

use crate::components::{footer::Footer, navbar::NavBar};

pub struct MainLayout;

impl Layout for MainLayout {
    type Data = ();

    fn path(_data: &Self::Data) -> std::path::PathBuf {
        "".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn view<'a>(
        _data: &'a Self::Data,
        page: impl ToElement<'a, Self>,
    ) -> impl rust_pages::widget::ToElement<'a, Self> {
        div![NavBar, main![div![page].class("main-inner")], Footer].class("page")
    }

    fn style() -> Vec<Style<Self>> {
        vec![
            Style::new(".page")
                .flex()
                .flex_column()
                .justify_content("center")
                .min_height("100vh")
                .min_width("100vw")
                .font_size("1.1rem"),
            Style::new("main")
                .width_full()
                .border_box()
                .padding("6rem 2rem 0 2rem")
                .flex_grow("1"),
            Style::new(".main-inner")
                .width_full()
                .max_width("80rem")
                .margin("0 auto"),
        ]
    }
}
