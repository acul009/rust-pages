use rust_pages::{div, page::Page, style::Style, theme::Theme};

pub struct Index;

impl Page for Index {
    type Data = ();

    fn path(_data: &Self::Data) -> std::path::PathBuf {
        "".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn view(_data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div!("Index")
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
