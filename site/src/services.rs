use std::borrow::Cow;

use rust_pages::{div, h1, page::Page, style::Style, theme::Theme, widget::picture};

use crate::components::{service_card::ServiceCard, site_data::asset_path};

pub struct Services;

pub struct Data {
    beratung: picture::Handle,
    administration: picture::Handle,
    netzwerk: picture::Handle,
    backup: picture::Handle,
    security: picture::Handle,
    telefonie: picture::Handle,
    videoueberwachung: picture::Handle,
    programmierung: picture::Handle,
}

impl Page for Services {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "services".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            beratung: picture::Handle::create(&asset_path("images/services/beratung.jpg"))?,
            administration: picture::Handle::create(&asset_path(
                "images/services/administration.jpg",
            ))?,
            netzwerk: picture::Handle::create(&asset_path("images/services/netzwerk.jpg"))?,
            backup: picture::Handle::create(&asset_path("images/services/festplatte.jpg"))?,
            security: picture::Handle::create(&asset_path("images/services/security.jpg"))?,
            telefonie: picture::Handle::create(&asset_path("images/services/telefonie.jpg"))?,
            videoueberwachung: picture::Handle::create(&asset_path(
                "images/services/videoueberwachung.jpg",
            ))?,
            programmierung: picture::Handle::create(&asset_path(
                "images/services/programmierung.jpg",
            ))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Leistungen".into())
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Leistungen"),
            ServiceCard::<Self>::new("Persönliche Beratung", &data.beratung),
            ServiceCard::<Self>::new("Administration", &data.administration,),
            ServiceCard::<Self>::new("Netzwerk", &data.netzwerk,),
            ServiceCard::<Self>::new("Backup", &data.backup,),
            ServiceCard::<Self>::new("Cyber-Sicherheit", &data.security,),
            ServiceCard::<Self>::new("Telefonie", &data.telefonie,),
            ServiceCard::<Self>::new("Videoüberwachung", &data.videoueberwachung,),
            ServiceCard::<Self>::new("Programmierung", &data.programmierung,)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
