use std::borrow::Cow;

use rust_pages::{div, h1, p, page::Page, style::Style, theme::Theme, widget::picture};

use crate::components::{person_card::PersonCard, site_data::asset_path};

pub struct Team;

pub struct Data {
    pub heinz: picture::Handle,
    pub luca: picture::Handle,
}

impl Page for Team {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf { "about-us/people".into() }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            heinz: picture::Handle::create(&asset_path("images/people/heinz.jpg"))?,
            luca: picture::Handle::create(&asset_path("images/people/luca.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("Unser Team".into()) }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unser Team"),
            PersonCard::new(
                &data.heinz,
                "Heinz Rahn",
                "Windows-Administrator, IT-Fachmann seit 1983",
                div![
                    p!("Mein erster Computer war der Sinclair ZX81, ein 8-Bit Computer, den ich damals noch per Hand lötete."),
                    p!("Während meiner Laufbahn bekam ich den Einzug des Computers in alle Lebensbereiche hautnah mit. Vom Commodore 64 über PCs mit OS/2 bis hin zum Einzug von Windows. Nachdem immer mehr Freunde und Bekannte mit ihren Anliegen zu mir kamen, beschloss ich schließlich mein Hobby zum Beruf zu machen."),
                    p!("Bis in in die 2000er Jahre arbeitete ich bei Gemeinden, Systemhäusern und war Mitglied einiger Interessengemeinschaften."),
                    p!("2004 machte ich mich schließlich selbstständig und eröffnete Rahn IT-Systemtechnik.")
                ]
            ),
            PersonCard::new(
                &data.luca,
                "Luca Wlcek",
                "Linux-Administrator, Webentwickler",
                div![
                    p!("Mit Heinz als Vater bin ich von klein auf mit Computern groß geworden."),
                    p!("Als ich 7 Jahre alt war bekam ich meinen ersten eigenen Computer."),
                    p!("In meiner Jugend kam mein Interesse für Programmierung und die Funktionsweise moderner Computer auf. Ich beschäftigte mich mit logischen Schaltkreisen und simpler Rechnerarchitektur."),
                    p!("Während meiner Ausbildung entdeckte ich Linux und die Open-Source Welt. Inzwischen bin ich langjähriger Linux-Nutzer und kümmere mich im Betrieb um die Linux-Infrastruktur.")
                ]
            )
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> { vec![] }
}
