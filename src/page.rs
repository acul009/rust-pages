use std::borrow::Cow;

use crate::{
    style::{Style, Stylesheet},
    widget::ToElement,
};

pub trait Page {
    type Data;

    fn path(_data: &Self::Data) -> std::path::PathBuf;

    fn load_data(&self) -> anyhow::Result<Self::Data>;

    fn title<'a>(_data: &'a Self::Data) -> Option<Cow<'a, str>> {
        None
    }
    fn view(data: &Self::Data) -> impl crate::widget::ToElement<'_, Self>;
    fn style(&self, theme: &dyn crate::theme::Theme) -> Vec<Style<Self>>;
}

pub trait PageWrapper {
    fn path(&self) -> std::path::PathBuf;
    fn title(&self) -> Option<Cow<'_, str>>;
    fn html(&self, f: &mut String) -> std::fmt::Result;
    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut Stylesheet);
}

pub struct PageLoader<P: Page> {
    page: P,
}

impl<P: Page> PageLoader<P> {
    pub fn new(page: P) -> Self {
        Self { page }
    }
}

pub trait PageLoaderWrapper {
    fn load(&self) -> anyhow::Result<Box<dyn PageWrapper>>;
}

impl<P: Page + 'static> PageLoaderWrapper for PageLoader<P> {
    fn load(&self) -> anyhow::Result<Box<dyn PageWrapper>> {
        let data = self.page.load_data()?;
        let container = PageContainer::<P> { data };
        Ok(Box::new(container))
    }
}

pub struct PageContainer<P: Page> {
    data: P::Data,
}

impl<P: Page> PageWrapper for PageContainer<P> {
    fn path(&self) -> std::path::PathBuf {
        P::path(&self.data)
    }

    fn title(&self) -> Option<Cow<'_, str>> {
        P::title(&self.data)
    }

    fn html(&self, f: &mut String) -> std::fmt::Result {
        let view = P::view(&self.data).to_element();
        view.html(f)
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut Stylesheet) {
        let view = P::view(&self.data).to_element();
        view.style(theme, stylesheet);
    }
}
