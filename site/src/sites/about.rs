use std::borrow::Cow;

use rust_pages::{div, h1, h3, p, page::Page, style::Style, theme::Theme, widget::picture};

use crate::components::{service_card::ServiceCard, site_data::asset_path};

pub struct About;

pub struct Data {
    pub gears: picture::Handle,
    pub transparency: picture::Handle,
    pub cloud: picture::Handle,
}

impl Page for About {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "about-us".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            gears: picture::Handle::create(&asset_path("images/services/gears.jpg"))?,
            transparency: picture::Handle::create(&asset_path("images/services/transparenz2.jpg"))?,
            cloud: picture::Handle::create(&asset_path("images/services/cloud.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Unsere Prinzipien".into())
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere Prinzipien"),
            ServiceCard::<Self>::new("Pragmatische Arbeitsweise", &data.gears).body( div![
                p!("In der IT führen oftmals viele Wege zum Ziel. Selten gibt es die eine Lösung."),
                p!("Unsere Ansätze und Lösungen richten sich nach den Bedürfnissen unserer Kunden. Dabei bemühen wir uns um eine gesunde Balance aus erprobten Techniken und modernen Standards."),
                p!("Sollte keine unserer etablierten Lösungen passen, finden wir eine neue. Denn IT soll funktionieren und Spaß machen.")
            ]),
            ServiceCard::<Self>::new("Ehrlichkeit und Transparenz", &data.transparency).body( div![
                p!("Eine erfolgreiche Zusammenarbeit braucht Vertrauen. Aus diesem Grund setzen wir bei unserer Arbeit auf Ehrlichkeit und Offenheit."),
                p!("Wir kennen nicht nur die Vorteile, sondern auch die Nachteile unserer Lösungen. Auch unsere Kunden haben ein Recht zu wissen, wo die Schwachstellen ihrer IT liegen."),
                p!("Wenn es um die Sicherheit und den Fortbestand Ihres Unternehmens geht, nehmen wir kein Blatt vor den Mund.")
            ]),
            ServiceCard::<Self>::new("Verantwortungsvoller Cloud-Umgang", &data.cloud).body( div![
                p!("Cloud-Dienste können sinnvoll sein, aber sie sind kein Allheilmittel. Für uns zählt eine saubere Abwägung von Chancen, Risiken und Abhängigkeiten."),
                h3!("Sicherheitsrisiken"),
                p!("Cloud-Plattformen sind beliebte Angriffsziele. Betreiber und Angreifer können im Ernstfall Zugang zu sensiblen Daten erlangen."),
                h3!("Sollten wir also auf Cloud verzichten?"),
                p!("Nein. Einige Cloud-Dienste bieten echte Vorteile. Wichtig ist, nicht blind auf Schlagworte zu vertrauen, sondern jede Lösung verantwortungsvoll zu bewerten.")
            ])
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
