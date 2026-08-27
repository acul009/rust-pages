use chrono::{NaiveDate, Utc};
use rust_pages::{
    b, br, div, h1, h2, p, page::Page, style::Style, theme::Theme, ul, widget::picture,
};

use crate::{
    components::{no_cookies::NoCookies, site_data::asset_path},
};
use company_references::{Background, Company, CompanyReferences};

mod company_references;

pub struct Index;

pub struct Data {
    age: u32,
    company_references: Vec<Company>,
}

impl Page for Index {
    type Data = Data;

    fn path(_data: &Self::Data) -> std::path::PathBuf {
        "".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        let founded = NaiveDate::from_ymd_opt(2004, 4, 1).unwrap();
        let age = Utc::now().date_naive().years_since(founded).unwrap();

        Ok(Data {
            age,
            company_references: vec![
                Company::new(
                    "Fischer",
                    "https://spenglerei-fischer.de",
                    Background::Dark,
                    "p-10",
                    picture::Handle::create(&asset_path("images/refs/fischer.png"))?,
                ),
                Company::new(
                    "Autowelt Ostermaier",
                    "https://autowelt-ostermaier.de",
                    Background::White,
                    "",
                    picture::Handle::create(&asset_path("images/refs/autowelt.png"))?,
                ),
                Company::new(
                    "Erbenermittlung Mayer",
                    "https://erben-mayer.de/",
                    Background::White,
                    "",
                    picture::Handle::create(&asset_path("images/refs/mayer.jpg"))?,
                ),
                Company::new(
                    "Danzl Gartentechnik",
                    "https://www.danzl-gartentechnik.de/",
                    Background::White,
                    "",
                    picture::Handle::create(&asset_path("images/refs/danzl.jpg"))?,
                ),
                Company::new(
                    "Langlechner",
                    "https://langlechner-haustechnik.de/",
                    Background::White,
                    "px-8",
                    picture::Handle::create(&asset_path("images/refs/langlechner.png"))?,
                ),
                Company::new(
                    "Fitworld",
                    "https://www.fitworldts.de/",
                    Background::Dark,
                    "p-8",
                    picture::Handle::create(&asset_path("images/refs/fitworld.png"))?,
                ),
                Company::new(
                    "Rechtsanwälte Heiß",
                    "https://www.rechtsanwaelte-heiss.de/",
                    Background::White,
                    "",
                    picture::Handle::create(&asset_path("images/refs/heiss.png"))?,
                ),
                Company::new(
                    "Biostein",
                    "https://www.biostein.com/",
                    Background::Dark,
                    "p-8",
                    picture::Handle::create(&asset_path("images/refs/biostein.png"))?,
                ),
                Company::new(
                    "Mauerberger",
                    "https://www.mauerberger-tore.de/",
                    Background::White,
                    "px-8",
                    picture::Handle::create(&asset_path("images/refs/mauerberger.png"))?,
                ),
                Company::new(
                    "Solarbau Chiemgau",
                    "https://solarbau-chiemgau.de/",
                    Background::Dark,
                    "p-8",
                    picture::Handle::create(&asset_path("images/refs/solarbau-chiemgau.png"))?,
                ),
                Company::new(
                    "Zimmermann Transporte",
                    "https://www.zimmermann-transporte.com/",
                    Background::Dark,
                    "p-8",
                    picture::Handle::create(&asset_path("images/refs/zimmermann.png"))?,
                ),
                Company::new(
                    "Schlosserei-Brand",
                    "https://www.schlosserei-brand.de/",
                    Background::White,
                    "p-8",
                    picture::Handle::create(&asset_path("images/refs/brand.jpg"))?,
                ),
            ],
        })
    }

    fn view<'a>(data: &'a Self::Data) -> impl rust_pages::widget::ToElement<'a, Self> {
        div![
            h1!("Willkommen bei Rahn IT-Systemtechnik"),
            p!(
                "Wir betreuen seit ",
                b!(data.age),
                " Jahren professionell und zuverlässig die IT unserer Kunden.",
                br(),
                "Profitieren auch Sie von unserem umfangreichen Wissen und unserer langjährigen Erfahrung."
            ),
            p!(
                "Unser Kundenstamm ist so vielseitig wie unsere Leistungen.",
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
            CompanyReferences::new(&data.company_references),
            h2!("Cookies? Nicht mit uns!"),
            NoCookies,
            p!(
                "Cookie-Banner sind nicht nur nervig. Oft werden Marketing- und Tracking-Cookies installiert, welche die Privatsphäre beeinträchtigen.",
                br(),
                "Wir stellen uns gegen diesen Trend: Unsere Webseite funktioniert ",
                b!("komplett ohne Cookies!")
            )
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
