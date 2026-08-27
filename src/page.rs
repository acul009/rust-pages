use crate::{
    style::{Style, Stylesheet},
    widget::ToElement,
};

pub trait Page {
    type Data;

    fn path(_data: &Self::Data) -> std::path::PathBuf;

    fn load_data(&self) -> anyhow::Result<Self::Data>;

    fn settings<'a>(_data: &'a Self::Data, _settings: &mut PageSettings) {}
    fn view<'a>(data: &'a Self::Data) -> impl crate::widget::ToElement<'a, Self>;
    fn style(theme: &dyn crate::theme::Theme) -> Vec<Style<Self>>;
}

pub trait PageWrapper {
    fn path(&self) -> std::path::PathBuf;
    fn settings(&self, settings: &mut PageSettings);
    fn html(&self, f: &mut String) -> std::fmt::Result;
    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut Stylesheet);
}

pub struct PageSettings {
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) custom_header: Option<String>,
    pub(crate) show_in_sitemap: bool,
}

impl PageSettings {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            custom_header: None,
            show_in_sitemap: true,
        }
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn custom_header(&mut self, custom_header: impl Into<String>) -> &mut Self {
        self.custom_header = Some(custom_header.into());
        self
    }

    pub fn show_in_sitemap(&mut self) -> &mut Self {
        self.show_in_sitemap = true;
        self
    }

    pub fn hide_in_sitemap(&mut self) -> &mut Self {
        self.show_in_sitemap = false;
        self
    }
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

    fn settings(&self, settings: &mut PageSettings) {
        P::settings(&self.data, settings);
    }

    fn html(&self, f: &mut String) -> std::fmt::Result {
        let view = P::view(&self.data).to_element();
        view.html(f)
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut Stylesheet) {
        let styles = P::style(theme);
        stylesheet.add_styles(&styles);
        let view = P::view(&self.data).to_element();
        view.style(theme, stylesheet);
    }
}
