use rust_pages::{div, page::Page, style::Style};

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

    fn style(&self) -> Vec<Style<Self>> {
        vec![]
    }
}
