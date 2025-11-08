use std::{borrow::Cow, fmt::Display, fs::File, io::Write, path::PathBuf};

use anyhow::Context;

use crate::{
    layout::{Layout, LayoutLoader, LayoutLoaderWrapper, LayoutWrapper},
    page::{Page, PageLoader, PageLoaderWrapper, PageWrapper},
    style::Stylesheet,
};

pub struct SiteBuilder<Title> {
    default_title: Title,
    output_dir: PathBuf,
    pages: Vec<Box<dyn PageLoaderWrapper>>,
    layouts: Vec<Box<dyn LayoutLoaderWrapper>>,
}

impl SiteBuilder<()> {
    pub fn new() -> SiteBuilder<()> {
        SiteBuilder {
            default_title: (),
            output_dir: "./build".into(),
            pages: Vec::new(),
            layouts: Vec::new(),
        }
    }
}

impl<Title> SiteBuilder<Title> {
    pub fn title(self, title: impl Display) -> SiteBuilder<String> {
        SiteBuilder {
            default_title: title.to_string(),
            output_dir: self.output_dir,
            pages: self.pages,
            layouts: self.layouts,
        }
    }

    pub fn page<P: Page + 'static>(mut self, page: P) -> SiteBuilder<Title> {
        let loader = PageLoader::new(page);
        self.pages.push(Box::new(loader));
        self
    }

    pub fn layout<L: Layout + 'static>(mut self, layout: L) -> SiteBuilder<Title> {
        let loader = LayoutLoader::new(layout);
        self.layouts.push(Box::new(loader));
        self
    }
}

impl SiteBuilder<String> {
    pub fn build(&self) -> anyhow::Result<()> {
        let mut stylesheet = Stylesheet::new();
        let pages = self.load_pages()?;
        let layouts = self.load_layouts()?;

        if std::fs::exists(self.output_dir.as_path()).context("Error checking for output dir")? {
            std::fs::remove_dir_all(self.output_dir.as_path())
                .context("Error deleting output dir")?;
        }
        std::fs::create_dir_all(self.output_dir.as_path()).context("Error creating output dir")?;

        for page in pages {
            let path = page.path();
            println!("Building: /{}", path.display());
            let title = page
                .title()
                .unwrap_or_else(|| Cow::Borrowed(&self.default_title));

            let mut finished_html = String::new();

            {
                use std::fmt::Write;

                finished_html.push_str("<!DOCTYPE html><html><head>");
                finished_html.push_str("<meta charset=\"utf-8\">");
                finished_html.push_str(
                    "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">",
                );
                finished_html.push_str("<link rel=\"stylesheet\" href=\"/styles.css\">");

                write!(&mut finished_html, "<title>{}</title>", title)?;
                finished_html.push_str("</head><body>");

                let mut page_html = String::new();
                page.html(&mut page_html)?;
                for layout in &layouts {
                    if path.starts_with(layout.path()) {
                        let mut new = String::new();
                        layout.html(&mut new, &page_html)?;
                        page_html = new;
                    }
                }
                finished_html.push_str(&page_html);

                finished_html.push_str("</body></html>");
            }

            page.style(&mut stylesheet);

            let mut file = File::create(self.output_dir.join(path).join("index.html"))?;
            file.write_all(finished_html.as_bytes())?;
            file.flush()?;
        }

        println!("Building stylesheet");
        let mut file = File::create(self.output_dir.join("styles.css"))?;
        file.write_all(stylesheet.to_css().as_bytes())?;
        file.flush()?;

        println!("Done!");
        Ok(())
    }

    fn load_pages(&self) -> anyhow::Result<Vec<Box<dyn PageWrapper>>> {
        let mut loaded_pages = Vec::new();
        for page in &self.pages {
            let loaded = page.load()?;
            loaded_pages.push(loaded);
        }
        Ok(loaded_pages)
    }

    fn load_layouts(&self) -> anyhow::Result<Vec<Box<dyn LayoutWrapper>>> {
        let mut loaded_layouts = Vec::new();
        for layout in &self.layouts {
            let loaded = layout.load()?;
            loaded_layouts.push(loaded);
        }
        loaded_layouts.sort_by(|layout1, layout2| {
            let len1 = layout1.path().display().to_string().len();
            let len2 = layout2.path().display().to_string().len();
            if len1 > len2 {
                std::cmp::Ordering::Greater
            } else if len1 == len2 {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        });
        Ok(loaded_layouts)
    }
}
