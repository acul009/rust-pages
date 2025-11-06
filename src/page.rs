use crate::widget::Element;

pub trait Page {
    type Data;

    fn load_data(&self) -> anyhow::Result<Self::Data>;

    fn title<'a>(data: &'a Self::Data) -> Element<'a>;
    fn meta<'a>(data: &'a Self::Data) -> Meta;
    fn render<'a>(data: &'a Self::Data) -> Element<'a>;
}

pub struct Meta {
    pub description: Option<String>,
}
