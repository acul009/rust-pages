use std::borrow::Cow;

use rust_pages::{div, h1, page::Page, style::Style, theme::Theme, widget::picture};

use crate::{
    components::{person_card::PersonCard, site_data::asset_path},
};

pub struct Team;

pub struct Data {
    pub heinz: picture::Handle,
    pub luca: picture::Handle,
}

impl Page for Team {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "about-us/people".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            heinz: picture::Handle::create(&asset_path("../old/src/lib/images/people/heinz.jpg"))?,
            luca: picture::Handle::create(&asset_path("../old/src/lib/images/people/luca.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Unser Team".into())
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unser Team"),
            PersonCard::new(&data.heinz, "Heinz Rahn", "Windows-Administrator, IT-Fachmann seit 1983", r#"<p>Mein erster Computer war der Sinclair ZX81, ein 8-Bit Computer, den ich damals noch per Hand lÃƒÂ¶tete.</p><p>WÃƒÂ¤hrend meiner Laufbahn bekam ich den Einzug des Computers in alle Lebensbereiche hautnah mit. Nachdem immer mehr Freunde und Bekannte mit ihren Anliegen zu mir kamen, beschloss ich schlieÃƒÅ¸lich mein Hobby zum Beruf zu machen.</p><p>2004 machte ich mich schlieÃƒÅ¸lich selbststÃƒÂ¤ndig und erÃƒÂ¶ffnete Rahn IT-Systemtechnik.</p>"#),
            PersonCard::new(&data.luca, "Luca Wlcek", "Linux-Administrator, Webentwickler", r#"<p>Mit Heinz als Vater bin ich von klein auf mit Computern groÃƒÅ¸ geworden.</p><p>In meiner Jugend kam mein Interesse fÃƒÂ¼r Programmierung und die Funktionsweise moderner Computer auf. WÃƒÂ¤hrend meiner Ausbildung entdeckte ich Linux und die Open-Source Welt.</p><p>Inzwischen bin ich langjÃƒÂ¤hriger Linux-Nutzer und kÃƒÂ¼mmere mich im Betrieb um die Linux-Infrastruktur.</p>"#)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
