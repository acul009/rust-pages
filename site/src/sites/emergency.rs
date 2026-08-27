use rust_pages::{a, br, div, h1, p, page::Page, style::Style, theme::Theme};

use crate::components::{
    browser_content::BrowserContent,
    site_data::{PHONE, tel_href},
};

pub struct Emergency;

impl Page for Emergency {
    type Data = ();

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "emergency".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn settings(_: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings.title("24 Stunden Notdienst");
    }

    fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Im Notfall immer für Sie da"),
            p!(
                "Steht Ihr Betrieb still weil Ihre IT streikt?",
                br(),
                "Wir bieten schnelle und professionelle Hilfe bei dringenden Problemen."
            ),
            p!(
                "Falls es sich nicht um einen dringenden Notfall handelt, nutzen Sie bitte unsere regulären Geschäftszeiten."
            ),
            p!(
                "Mo-Do: 09:00 - 16:30",
                br(),
                "Fr: 09:00 - 12:00",
                br(),
                "24 Stunden Notdienst",
                br(),
                "Auch an Sonn- und Feiertagen"
            ),
            BrowserContent::new(
                a(format!("Tel: {}", PHONE)).href(tel_href()).class("cta")
            )
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".cta")
                .display("inline-block")
                .padding("1rem 1.5rem")
                .border_radius("1rem")
                .background_color("rgba(255,255,255,0.08)")
                .text_decoration_none(),
        ]
    }
}
