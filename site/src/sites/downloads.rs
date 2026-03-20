use std::borrow::Cow;

use rust_pages::{a, br, div, h1, h2, p, raw_html, page::Page, style::Style, theme::Theme};

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
            h1!("Fernwartungs-Software"),
            p!("Hier finden Sie unsere Fernwartungs-Software zum Download."),
            h2!("Schnelle Fernwartung"),
            p!("Diese Software erlaubt es uns, Ihnen zu helfen, sobald Sie die Software starten."),
            a("Download").href("https://get.teamviewer.com/57y9u6n").class("cta"),
            h2!("Dauerhafte Fernwartung"),
            p!("Dieses Installationsprogramm richtet eine dauerhafte Fernwartung ein.", br(), "Dadurch kÃƒÂ¶nnen wir uns auch ohne Ihr Zutun um Ihre GerÃƒÂ¤te kÃƒÂ¼mmern."),
            a("Download").href("https://get.teamviewer.com/q3zt6wn").class("cta"),
            raw_html(r#"<p class="todo">TODO: Guided download / install flow if interactive behavior is desired.</p>"#)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".cta").display("inline-block").margin("1rem 0 2rem 0").padding("1rem 1.5rem").border_radius("1rem").background_color("rgba(255,255,255,0.08)").text_decoration_none(),
            Style::new(".todo").property("opacity", ".75"),
        ]
    }
}
