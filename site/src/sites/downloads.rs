use std::borrow::Cow;

use rust_pages::{br, div, h1, h2, p, page::Page, style::Style, theme::Theme};

use crate::components::link_button::LinkButton;

pub struct Downloads;

impl Page for Downloads {
    type Data = ();

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "downloads".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Downloads".into())
    }

    fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Downloads"),
            h2!("Schnelle Fernwartung"),
            p!("Diese Software erlaubt es uns, Ihnen zu helfen, sobald Sie die Software starten."),
            div![LinkButton::new().label("Download").href("https://get.teamviewer.com/57y9u6n")]
                .class("download-action"),
            h2!("Dauerhafte Fernwartung"),
            p!(
                "Dieses Installationsprogramm richtet eine dauerhafte Fernwartung ein.",
                br(),
                "Dadurch kÃƒÆ’Ã‚Â¶nnen wir uns auch ohne Ihr Zutun um Ihre GerÃƒÆ’Ã‚Â¤te kÃƒÆ’Ã‚Â¼mmern."
            ),
            div![LinkButton::new().label("Download").href("https://get.teamviewer.com/q3zt6wn")]
                .class("download-action"),
            p!("TODO: Guided download / install flow if interactive behavior is desired.")
                .class("todo")
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".download-action").margin("1rem 0 2rem 0"),
            Style::new(".todo").property("opacity", ".75"),
        ]
    }
}
