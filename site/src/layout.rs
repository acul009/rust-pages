use rust_pages::{div, layout::Layout, main, style::Style, widget::ToElement};

use crate::navbar::NavBar;

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
        div![NavBar::new(), main![page]].class("page")
    }

    fn style() -> Vec<Style<Self>> {
        vec![
            Style::new(".page")
                .flex()
                .justify_content("center")
                .min_height("100vh")
                .min_width("100vw")
                .font_size("1.1rem"),
            Style::new("main")
                .max_width("80rem")
                .padding("6rem 0 0 0")
                .flex_grow("1"),
        ]
    }
}
