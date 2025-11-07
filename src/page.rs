use std::borrow::Cow;

// pub trait Page {
//     type Data;

//     fn load_data(&self) -> anyhow::Result<Self::Data>;

//     fn title<'a>(data: &'a Self::Data) -> Cow<'a, str>;
//     fn meta<'a>(data: &'a Self::Data) -> Meta;
//     fn render<'a>(data: &'a Self::Data) -> Element;
// }

pub struct Meta {
    pub description: Option<String>,
}
