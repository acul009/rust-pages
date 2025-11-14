mod index;
mod navbar;

use rust_pages::{
    builder::SiteBuilder,
    div,
    layout::Layout,
    style::{
        Style,
        predone::{animated_details, remove_default_styles},
    },
    theme::Dark,
    widget::ToElement,
};

use crate::{index::Index, navbar::NavBar};

fn main() {
    let builder = SiteBuilder::new()
        .title("Rahn-IT")
        .layout(MainLayout)
        .page(Index)
        .theme(Dark {})
        .styles(remove_default_styles())
        .styles(animated_details());

    builder.build().unwrap();
}

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
        div![NavBar::new(), page]
    }

    fn style(&self) -> Vec<Style<Self>> {
        todo!()
    }
}
