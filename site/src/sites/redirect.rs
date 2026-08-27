use rust_pages::{a, div, h1, html_sanitize, p, page::Page};

#[derive(Clone)]
pub struct Redirect {
    from: String,
    to: String,
}

impl Redirect {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

impl Page for Redirect {
    type Data = Self;

    fn path(data: &Self::Data) -> std::path::PathBuf {
        data.from.clone().into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(self.clone())
    }

    fn settings(data: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings
            .title("Seite umgezogen")
            .custom_header(format!(
                r#"<meta http-equiv="refresh" content="0; url={}" />"#,
                html_sanitize(&data.to)
            ))
            .hide_in_sitemap();
    }

    fn view<'a>(data: &'a Self::Data) -> impl rust_pages::widget::ToElement<'a, Self> {
        div![
            h1!("Diese Seite ist umgezogen."),
            p!(
                a("Falls Sie nicht automatisch umgeleitet werden, klicken sie hier!")
                    .href(&data.to)
                    .class("link")
            )
        ]
    }

    fn style(&self, _theme: &dyn rust_pages::theme::Theme) -> Vec<rust_pages::style::Style<Self>> {
        vec![]
    }
}
