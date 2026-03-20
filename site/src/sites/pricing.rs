use std::borrow::Cow;

use rust_pages::{
    div, h1, p,
    page::Page,
    style::Style,
    theme::Theme,
    widget::{ToElement, container::Container},
};

use crate::components::{mail::Mail, phone::Phone};

fn strong<'a>(text: &'a str) -> Container<'a, Pricing> {
    Container::new("strong").child(text)
}

pub struct Pricing;

impl Page for Pricing {
    type Data = ();

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "pricing".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Preise".into())
    }

    fn view(_: &Self::Data) -> impl ToElement<'_, Self> {
        div![
            h1!("Unsere Preise sind kein Geheimnis."),
            div![
                div![
                    div![strong("Anfahrt pro KM")],
                    div!["0,71 Ã¢â€šÂ¬"].class("amount")
                ]
                .class("price-row"),
                div![
                    div![
                        strong("Dienstleistung pro Stunde"),
                        p!("Bei einer Anfahrt ", Container::new("b").child("unter 25 km"), " werden mindestens 0,25 Stunden berechnet."),
                        p!("Bei einer Anfahrt ", Container::new("b").child("ab 25 km"), " werden mindestens 0,5 Stunden berechnet.")
                    ],
                    div!["107,10 Ã¢â€šÂ¬"].class("amount")
                ]
                .class("price-row")
            ]
            .class("price-list"),
            p!("Unsere Preise fÃƒÆ’Ã‚Â¼r GerÃƒÆ’Ã‚Â¤te richten sich nach unseren aktuellen Einkaufspreisen."),
            p!("FÃƒÆ’Ã‚Â¼r ein Angebot, rufen Sie uns an oder schreiben Sie uns eine E-Mail."),
            p!(Phone),
            p!(Mail)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".price-list").margin("2rem 0"),
            Style::new(".price-row")
                .grid()
                .property("grid-template-columns", "1fr auto")
                .gap("1rem 2rem")
                .padding("1.25rem 0")
                .property("border-top", "1px solid rgba(255,255,255,0.15)"),
            Style::new(".price-row:last-child")
                .property("border-bottom", "1px solid rgba(255,255,255,0.15)"),
            Style::new(".amount")
                .font_size("1.35rem")
                .text_align_right(),
        ]
    }
}
