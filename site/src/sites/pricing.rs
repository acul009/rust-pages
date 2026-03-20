use std::borrow::Cow;

use rust_pages::{div, h1, p, page::Page, raw_html, style::Style, theme::Theme};

use crate::components::{mail::Mail, phone::Phone};

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

    fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere Preise sind kein Geheimnis."),
            raw_html(
                r#"<div class="price-list"><div class="price-row"><div><strong>Anfahrt pro KM</strong></div><div class="amount">0,71 Ã¢â€šÂ¬</div></div><div class="price-row"><div><strong>Dienstleistung pro Stunde</strong><p>Bei einer Anfahrt <b>unter 25 km</b> werden mindestens 0,25 Stunden berechnet.</p><p>Bei einer Anfahrt <b>ab 25 km</b> werden mindestens 0,5 Stunden berechnet.</p></div><div class="amount">107,10 Ã¢â€šÂ¬</div></div></div>"#
            ),
            p!(
                "Unsere Preise fÃƒÂ¼r GerÃƒÂ¤te richten sich nach unseren aktuellen Einkaufspreisen."
            ),
            p!("FÃƒÂ¼r ein Angebot, rufen Sie uns an oder schreiben Sie uns eine E-Mail."),
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
