use rust_pages::{
    a, br, div, h1, h2, p, page::Page, style::Style, theme::Theme, ul, widget::ToElement,
};

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

    fn settings(_: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings
            .title("Downloads")
            .description("Hier finden Sie unsere Fernwartungs-Software und weitere Downloads.");
    }

    fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        //
        div![
            h1!("Downloads"),
            download_item(
                "Schnelle Fernwartung",
                p!(
                    "Diese Software erlaubt es uns, Ihnen zu helfen, sobald Sie die Software starten."
                ),
                "https://get.teamviewer.com/57y9u6n"
            ),
            download_item(
                "Dauerhafte Fernwartung",
                p!(
                    "Dieses Installationsprogramm richtet eine dauerhafte Fernwartung ein.",
                    br(),
                    "Dadurch können wir uns auch ohne Ihr Zutun um Ihre Geräte kümmern."
                ),
                "https://get.teamviewer.com/57y9u6n"
            ),
            download_item(
                "Rahn-IT Toolbox",
                p!(
                    "Unsere Open-Source toolbox mit diversen Werkzeugen für unseren IT-Alltag",
                    br(),
                    "Darunter auch:",
                    ul![
                        "Schnellinstallation für ausgewählte Programme",
                        "Pfadlängenprüfer für überlange Windows-Pfade",
                        "Decoder für einige Encodierungen"
                    ],
                    "Den Quellcode finden Sie unter: ",
                    a("https://github.com/Rahn-IT/toolbox")
                        .href("https://github.com/Rahn-IT/toolbox")
                        .class("link"),
                ),
                "https://github.com/Rahn-IT/toolbox/releases/latest/download/toolbox.exe"
            ),
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".download-action").margin("1rem 0 3rem 0")]
    }
}

fn download_item<'a, E: 'a>(
    title: &'a str,
    description: impl ToElement<'a, E>,
    link: &'a str,
) -> impl ToElement<'a, E> {
    div![
        h2!(title),
        description,
        div![LinkButton::new().label("Download").href(link)].class("download-action")
    ]
}
