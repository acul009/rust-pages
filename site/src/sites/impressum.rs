use std::borrow::Cow;

use rust_pages::{div, h1, raw_html, page::Page, style::Style, theme::Theme};

use crate::components::site_data::{CITY, IBAN, MAIL, NAME, PHONE, POSTAL_CODE, STREET, UST_ID, tel_href};

pub struct Impressum;

impl Page for Impressum {
    type Data = ();

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "impressum".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Impressum".into())
    }

    fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Impressum"),
            raw_html(format!(r#"<table class="legal-table"><tr><td>Vertreter</td><td>Heinz Rahn</td></tr><tr><td>Adresse</td><td>{}<br>{}<br>{} {}</td></tr><tr><td>Telefon</td><td><a href="{}">{}</a></td></tr><tr><td>E-Mail</td><td><a href="mailto:{}">{}</a></td></tr><tr><td>USt-ID</td><td>{}</td></tr><tr><td>IBAN</td><td>{}</td></tr><tr><td>Erstellt mit</td><td>Rust Pages<br>TODO: Technologie-Hinweis bei Bedarf ergÃƒÂ¤nzen.</td></tr></table>"#, NAME, STREET, POSTAL_CODE, CITY, tel_href(), PHONE, MAIL, MAIL, UST_ID, IBAN).leak())
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".legal-table").width_full().property("border-collapse", "collapse").margin("2rem 0"),
            Style::new(".legal-table td").padding("1rem 0").property("vertical-align", "top").property("border-top", "1px solid rgba(255,255,255,0.15)"),
            Style::new(".legal-table tr:last-child td").property("border-bottom", "1px solid rgba(255,255,255,0.15)"),
            Style::new(".legal-table td:first-child").width("12rem").font_size("1rem"),
        ]
    }
}
