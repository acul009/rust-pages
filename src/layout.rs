use crate::{
    style::{Style, Stylesheet},
    widget::ToElement,
};

pub trait Layout {
    type Data;

    fn path(_data: &Self::Data) -> std::path::PathBuf;

    fn load_data(&self) -> anyhow::Result<Self::Data>;

    fn view<'a>(
        data: &'a Self::Data,
        page: impl ToElement<'a, Self>,
    ) -> impl crate::widget::ToElement<'a, Self>;

    fn style(&self) -> Vec<Style<Self>>;
}

pub trait LayoutWrapper {
    fn path(&self) -> std::path::PathBuf;
    fn html(&self, f: &mut String, page: &str) -> std::fmt::Result;
    fn style(&self, stylesheet: &mut Stylesheet);
}

pub struct LayoutLoader<L: Layout> {
    layout: L,
}

impl<L: Layout> LayoutLoader<L> {
    pub fn new(layout: L) -> Self {
        Self { layout }
    }
}

pub trait LayoutLoaderWrapper {
    fn load(&self) -> anyhow::Result<Box<dyn LayoutWrapper>>;
}

impl<L: Layout + 'static> LayoutLoaderWrapper for LayoutLoader<L> {
    fn load(&self) -> anyhow::Result<Box<dyn LayoutWrapper>> {
        let data = self.layout.load_data()?;
        let container = LayoutContainer::<L> { data };
        Ok(Box::new(container))
    }
}

pub struct LayoutContainer<P: Layout> {
    data: P::Data,
}

impl<L: Layout> LayoutWrapper for LayoutContainer<L> {
    fn path(&self) -> std::path::PathBuf {
        L::path(&self.data)
    }

    fn html(&self, f: &mut String, page: &str) -> std::fmt::Result {
        let view = L::view(&self.data, page).to_element();
        view.html(f)
    }

    fn style(&self, stylesheet: &mut Stylesheet) {
        let view = L::view(&self.data, "").to_element();
        view.style(stylesheet);
    }
}
