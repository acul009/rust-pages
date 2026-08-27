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

    fn settings(_: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings
            .title("Preise")
            .description("Unsere Preise sind kein Geheimnis.");
    }

    fn view(_: &Self::Data) -> impl ToElement<'_, Self> {
        div![
            h1!("Unsere Preise sind kein Geheimnis."),
            div![
                div![
                    div![strong("Anfahrt pro KM")],
                    div!["0,71 €"].class("amount")
                ]
                .class("price-row"),
                div![
                    div![
                        strong("Dienstleistung pro Stunde"),
                        p!(
                            "Bei einer Anfahrt ",
                            Container::new("b").child("unter 25 km"),
                            " werden mindestens 0,25 Stunden berechnet."
                        ),
                        p!(
                            "Bei einer Anfahrt ",
                            Container::new("b").child("ab 25 km"),
                            " werden mindestens 0,5 Stunden berechnet."
                        )
                    ],
                    div!["107,10 €"].class("amount")
                ]
                .class("price-row")
            ]
            .class("price-list"),
            p!("Unsere Preise für Geräte richten sich nach unseren aktuellen Einkaufspreisen."),
            p!("Für ein Angebot, rufen Sie uns an oder schreiben Sie uns eine E-Mail."),
            p!(Phone),
            p!(Mail)
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> {
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
