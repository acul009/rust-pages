use chrono::{NaiveDate, Utc};
use rust_pages::{
    b, div, h1, h2, p,
    page::Page,
    style::Style,
    theme::Theme,
    ul,
    widget::{br, picture},
};

use crate::no_cookies::NoCookies;

pub struct Index;

pub struct Data {
    age: u32,
    autowelt: picture::Handle,
}

impl Page for Index {
    type Data = Data;

    fn path(_data: &Self::Data) -> std::path::PathBuf {
        "".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        let founded = NaiveDate::from_ymd_opt(2004, 4, 1).unwrap();
        let age = Utc::now().date_naive().years_since(founded).unwrap();

        let administration = picture::Handle::create("images/refs/autowelt.png")?;

        Ok(Data {
            age,
            autowelt: administration,
        })
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Willkommen bei Rahn IT-Systemtechnik"),
            p!(
                "Wir betreuen Seit ",
                b!(data.age),
                " Jahren professionell und zuverlässig die IT unserer Kunden.",
                br(),
                "Profitieren auch Sie von unserem umfangreichen Wissen, und unserer langjährige Erfahrung."
            ),
            p!(
                "Unser Kundenstamm ist dabei so vielseitig wie unsere Leistungen,",
                br(),
                "In welcher Branche Sie auch tätig sind, zögern Sie nicht uns zu kontaktieren."
            ),
            h2!("Unser Einsatzgebiet"),
            p!("Per Fernwartung sind wir gerne deutschlandweit für Sie da."),
            p!("In folgenden Gebieten sind wir gerne persönlich für Sie da:"),
            ul!(
                "Landkreis Altötting",
                "Landkreis Mühldorf",
                "Landkreis Rosenheim",
                "Landkreis Rottal-Inn",
                "Landkreis Traunstein"
            ),
            h2!("Unsere Kunden"),
            div![picture(&data.autowelt)],
            h2!("Cookies? Nicht mit uns!"),
            NoCookies,
            p!(
                "Cookie-Banner sind nicht nur nervig, oft werden Marketing- und Tracking-Cookies installiert, welche die Privatsphäre beeinträchtigen.",
                br(),
                "Wir stellen uns gegen diesen Trend: Unsere Webseite funktioniert ",
                b!("komplett ohne Cookies!")
            )
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
