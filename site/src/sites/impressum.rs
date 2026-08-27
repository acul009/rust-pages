use std::borrow::Cow;

use rust_pages::{
    a, br, div, h1,
    page::Page,
    style::Style,
    theme::Theme,
    widget::{ToElement, container::Container},
};

use crate::components::site_data::{CITY, IBAN, MAIL, NAME, PHONE, POSTAL_CODE, STREET, UST_ID, tel_href};

fn row<'a>(label: impl ToElement<'a, Impressum>, value: impl ToElement<'a, Impressum>) -> Container<'a, Impressum> {
    Container::new("tr")
        .child(Container::new("td").child(label))
        .child(Container::new("td").child(value))
}

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

    fn view(_: &Self::Data) -> impl ToElement<'_, Self> {
        div![
            h1!("Impressum"),
            Container::new("table")
                .class("legal-table")
                .child(row("Vertreter", "Heinz Rahn"))
                .child(row(
                    "Adresse",
                    div![NAME, br(), STREET, br(), format!("{} {}", POSTAL_CODE, CITY)]
                ))
                .child(row(
                    "Telefon",
                    a(PHONE).href(tel_href())
                ))
                .child(row(
                    "E-Mail",
                    a(MAIL).href(format!("mailto:{}", MAIL))
                ))
                .child(row("USt-ID", UST_ID))
                .child(row("IBAN", IBAN))
                .child(row(
                    "Erstellt mit",
                    div!["Rust Pages", br(), "TODO: Technologie-Hinweis bei Bedarf ergänzen."]
                ))
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
