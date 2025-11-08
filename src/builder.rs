use std::{
    borrow::Cow,
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::page::{Page, PageContainer, PageWrapper};

pub struct SiteBuilder<Title> {
    title: Title,
    output_dir: PathBuf,
}

impl SiteBuilder<()> {
    pub fn new() -> SiteBuilder<()> {
        SiteBuilder {
            title: (),
            output_dir: "./build".into(),
        }
    }
}

impl<Title> SiteBuilder<Title> {
    pub fn title(self, title: impl Display) -> SiteBuilder<String> {
        SiteBuilder {
            title: title.to_string(),
            output_dir: self.output_dir,
        }
    }
}

impl SiteBuilder<String> {
    pub fn build_page<P: Page>(&self, page: P) -> Result<(), anyhow::Error> {
        let container = PageContainer::new(page)?;
        let title = container
            .title()
            .unwrap_or_else(|| Cow::Borrowed(&self.title));
        let mut html = format!(
            "<DOCTYPE html><html><head><title>{}</title></head><body>",
            title
        );

        container.html(&mut html)?;

        html.push_str("</body></html>");

        let output_path = self.output_dir.join(container.path()).join("index.html");

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        std::fs::write(output_path, html)?;
        Ok(())
    }
}
