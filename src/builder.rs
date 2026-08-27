use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;

mod server;

use crate::{
    builder::server::Server,
    html_sanitize,
    layout::{Layout, LayoutLoader, LayoutLoaderWrapper, LayoutWrapper},
    page::{Page, PageLoader, PageLoaderWrapper, PageSettings, PageWrapper},
    style::{Style, Stylesheet},
    widget::picture,
};

pub struct SiteBuilder<Title, Theme> {
    default_title: Title,
    base_url: Option<String>,
    output_dir: PathBuf,
    pages: Vec<Box<dyn PageLoaderWrapper>>,
    layouts: Vec<Box<dyn LayoutLoaderWrapper>>,
    styles: Vec<Style<()>>,
    scripts: Vec<PathBuf>,
    theme: Theme,
}

impl SiteBuilder<(), ()> {
    pub fn new() -> SiteBuilder<(), ()> {
        SiteBuilder {
            default_title: (),
            base_url: None,
            output_dir: PathBuf::from("./build"),
            pages: Vec::new(),
            layouts: Vec::new(),
            styles: Vec::new(),
            scripts: Vec::new(),
            theme: (),
        }
    }
}

impl<Title, Theme> SiteBuilder<Title, Theme> {
    pub fn title(self, title: impl Display) -> SiteBuilder<String, Theme> {
        SiteBuilder {
            default_title: title.to_string(),
            base_url: self.base_url,
            output_dir: self.output_dir,
            pages: self.pages,
            layouts: self.layouts,
            styles: self.styles,
            scripts: self.scripts,
            theme: self.theme,
        }
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn page<P: Page + 'static>(mut self, page: P) -> SiteBuilder<Title, Theme> {
        let loader = PageLoader::new(page);
        self.pages.push(Box::new(loader));
        self
    }

    pub fn layout<L: Layout + 'static>(mut self, layout: L) -> SiteBuilder<Title, Theme> {
        let loader = LayoutLoader::new(layout);
        self.layouts.push(Box::new(loader));
        self
    }

    pub fn styles(
        mut self,
        styles: impl IntoIterator<Item = Style<()>>,
    ) -> SiteBuilder<Title, Theme> {
        self.styles.extend(styles);
        self
    }

    pub fn script(mut self, path: impl Into<PathBuf>) -> Self {
        self.scripts.push(path.into());
        self
    }

    pub fn theme<T>(self, theme: T) -> SiteBuilder<Title, T> {
        SiteBuilder {
            default_title: self.default_title,
            base_url: self.base_url,
            output_dir: self.output_dir,
            pages: self.pages,
            layouts: self.layouts,
            styles: self.styles,
            scripts: self.scripts,
            theme,
        }
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

impl<Theme: crate::theme::Theme> SiteBuilder<String, Theme> {
    pub fn build(&self) -> anyhow::Result<()> {
        let args = Args::parse();

        if std::fs::exists(self.output_dir.as_path()).context("Error checking for output dir")? {
            std::fs::remove_dir_all(self.output_dir.as_path())
                .context("Error deleting output dir")?;
        }
        std::fs::create_dir_all(self.output_dir.as_path()).context("Error creating output dir")?;

        let script_urls = self.copy_scripts().context("error copying scripts")?;
        let _picture_context = picture::BuildContext::new(self.output_dir.as_path());

        let mut stylesheet = Stylesheet::new();
        stylesheet.add_styles(self.theme.css().as_slice());
        let pages = self.load_pages().context("error during page load")?;
        let layouts = self.load_layouts().context("error loading layouts")?;

        stylesheet.add_styles(self.styles.as_slice());

        let mut sitemap_paths = Vec::new();
        for page in pages {
            let path = page.path();
            println!("Building: /{}", path.display());
            let mut settings = PageSettings::new(self.default_title.clone());
            page.settings(&mut settings);

            if settings.show_in_sitemap {
                sitemap_paths.push(path.clone());
            }

            let mut finished_html = String::new();

            {
                use std::fmt::Write;

                finished_html.push_str("<!DOCTYPE html><html><head>");
                finished_html.push_str("<meta charset=\"utf-8\">");
                finished_html.push_str(
                    "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">",
                );
                if let Some(header) = &settings.custom_header {
                    finished_html.push_str(&header);
                }
                finished_html.push_str("<link rel=\"stylesheet\" href=\"/styles.css\">");
                for script_url in &script_urls {
                    write!(
                        &mut finished_html,
                        "<script src=\"{}\" defer></script>",
                        html_sanitize(script_url)
                    )?;
                }

                write!(
                    &mut finished_html,
                    "<title>{}</title>",
                    html_sanitize(&settings.title)
                )?;
                finished_html.push_str("</head><body>");

                let mut page_html = String::new();
                page.html(&mut page_html)
                    .context("error building page html")?;
                page.style(&self.theme, &mut stylesheet);
                for layout in &layouts {
                    if path.starts_with(layout.path()) {
                        let mut new = String::new();
                        layout
                            .html(&mut new, &page_html)
                            .context("error building layout html")?;
                        layout.style(&self.theme, &mut stylesheet);
                        page_html = new;
                    }
                }
                finished_html.push_str(&page_html);

                finished_html.push_str("</body></html>");
            }

            let folder_path = self.output_dir.join(path);
            fs::create_dir_all(folder_path.as_path()).context("error creating page dir")?;
            let mut file =
                File::create(folder_path.join("index.html")).context("error creating page file")?;
            file.write_all(finished_html.as_bytes())
                .context("error writing page")?;
            file.flush().context("error during flush")?;
        }

        if let Some(base_url) = &self.base_url {
            println!("Building sitemap");
            let base_url = base_url.trim_end_matches('/');
            let mut sitemap = String::from(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
            );
            for path in sitemap_paths {
                let path = path.to_string_lossy().replace('\\', "/");
                let location = if path.trim_matches('/').is_empty() {
                    format!("{base_url}/")
                } else {
                    format!("{base_url}/{}/", path.trim_matches('/'))
                };
                sitemap.push_str("  <url><loc>");
                sitemap.push_str(&xml_escape(&location));
                sitemap.push_str("</loc></url>\n");
            }
            sitemap.push_str("</urlset>\n");

            let mut file = File::create(self.output_dir.join("sitemap.xml"))
                .context("error creating sitemap file")?;
            file.write_all(sitemap.as_bytes())
                .context("error writing sitemap file")?;
            file.flush().context("error flushing sitemap file")?;
        } else {
            println!("[ERROR] Cannot generate sitemap without a base_url")
        }

        println!("Building stylesheet");
        let mut file = File::create(self.output_dir.join("styles.css"))
            .context("error creating stylesheet file")?;
        file.write_all(stylesheet.to_css().as_bytes())
            .context("error writing stylesheet")?;
        file.flush().context("error flushing stylesheet")?;

        match args.command {
            Command::Build => println!("Done!"),
            Command::Serve => {
                let server = Server::new(3971, self.output_dir.as_path())
                    .context("Failed to create server")?;
                server.run().context("Error while running server")?;
            }
        }
        Ok(())
    }

    fn copy_scripts(&self) -> anyhow::Result<Vec<String>> {
        if self.scripts.is_empty() {
            return Ok(Vec::new());
        }

        let destination_dir = self.output_dir.join("assets/scripts");
        fs::create_dir_all(&destination_dir)
            .context("error creating scripts output directory")?;

        self.scripts
            .iter()
            .map(|source| {
                let file_name = source
                    .file_name()
                    .context("script path has no file name")?;
                let destination = destination_dir.join(file_name);
                fs::copy(source, &destination).with_context(|| {
                    format!("error copying script {}", source.display())
                })?;
                Ok(format!(
                    "/assets/scripts/{}",
                    file_name.to_string_lossy()
                ))
            })
            .collect()
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

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Build,
    Serve,
}
