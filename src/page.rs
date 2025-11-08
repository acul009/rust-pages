use std::{borrow::Cow, path::PathBuf};

use crate::{
    style::{Style, Stylesheet},
    widget::ContextElement,
    widget::ToElement,
};

pub trait Page {
    type Data;

    fn path(_data: &Self::Data) -> std::path::PathBuf;

    fn load_data(&self) -> anyhow::Result<Self::Data>;

    fn title<'a>(_data: &'a Self::Data) -> Option<Cow<'a, str>> {
        None
    }
    fn meta(_data: &Self::Data) -> Meta {
        Meta::default()
    }
    fn view(data: &Self::Data) -> impl crate::widget::ToElement<'_, Self>;
    fn style(&self) -> Vec<Style<Self>>;
}

pub trait PageWrapper {
    fn path(&self) -> std::path::PathBuf;

    fn title(&self) -> Option<Cow<'_, str>>;
    fn meta(&self) -> Meta;
    fn html(&self, f: &mut String) -> std::fmt::Result;
    fn style(&self, stylesheet: &mut Stylesheet);
}

pub struct PageContainer<P: Page> {
    page: P,
    data: P::Data,
}

impl<P: Page> PageContainer<P> {
    pub fn new(page: P) -> anyhow::Result<Self> {
        let data = page.load_data()?;
        Ok(Self { page, data })
    }
}

impl<P: Page> PageWrapper for PageContainer<P> {
    fn path(&self) -> std::path::PathBuf {
        P::path(&self.data)
    }

    fn title(&self) -> Option<Cow<'_, str>> {
        P::title(&self.data)
    }

    fn meta(&self) -> Meta {
        P::meta(&self.data)
    }

    fn html(&self, f: &mut String) -> std::fmt::Result {
        let view = P::view(&self.data).to_element();
        view.html(f)
    }

    fn style(&self, stylesheet: &mut Stylesheet) {
        let view = P::view(&self.data).to_element();
        view.style(stylesheet);
    }
}

#[derive(Default)]
pub struct Meta {
    pub description: Option<String>,
}
