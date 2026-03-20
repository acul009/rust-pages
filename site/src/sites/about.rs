use std::borrow::Cow;

use rust_pages::{div, h1, page::Page, style::Style, theme::Theme, widget::picture};

use crate::{
    components::{service_card::ServiceCard, site_data::asset_path},
};

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
            ServiceCard::new("Pragmatische Arbeitsweise", &data.gears, r#"<p>In der IT fÃƒÂ¼hren oftmals viele Wege zum Ziel. Selten gibt es die eine LÃƒÂ¶sung.</p><p>Unsere AnsÃƒÂ¤tze und LÃƒÂ¶sungen richten sich nach den BedÃƒÂ¼rfnissen unserer Kunden. Dabei bemÃƒÂ¼hen wir uns um eine gesunde Balance aus erprobten Techniken und modernen Standards.</p><p>Sollte keine unserer etablierten LÃƒÂ¶sungen passen, finden wir eine neue. Denn IT soll funktionieren und SpaÃƒÅ¸ machen.</p>"#),
            ServiceCard::new("Ehrlichkeit und Transparenz", &data.transparency, r#"<p>Eine erfolgreiche Zusammenarbeit braucht Vertrauen. Aus diesem Grund setzen wir bei unserer Arbeit auf Ehrlichkeit und Offenheit.</p><p>Wir kennen nicht nur die Vorteile, sondern auch die Nachteile unserer LÃƒÂ¶sungen. Auch unsere Kunden haben ein Recht zu wissen, wo die Schwachstellen ihrer IT liegen.</p><p>Wenn es um die Sicherheit und den Fortbestand Ihres Unternehmens geht, nehmen wir kein Blatt vor den Mund.</p>"#),
            ServiceCard::new("Verantwortungsvoller Cloud-Umgang", &data.cloud, r#"<p>Cloud-Dienste kÃƒÂ¶nnen sinnvoll sein, aber sie sind kein Allheilmittel. FÃƒÂ¼r uns zÃƒÂ¤hlt eine saubere AbwÃƒÂ¤gung von Chancen, Risiken und AbhÃƒÂ¤ngigkeiten.</p><h3>Sicherheitsrisiken</h3><p>Cloud-Plattformen sind beliebte Angriffsziele. Betreiber und Angreifer kÃƒÂ¶nnen im Ernstfall Zugang zu sensiblen Daten erlangen.</p><h3>Sollten wir also auf Cloud verzichten?</h3><p>Nein. Einige Cloud-Dienste bieten echte Vorteile. Wichtig ist, nicht blind auf Schlagworte zu vertrauen, sondern jede LÃƒÂ¶sung verantwortungsvoll zu bewerten.</p>"#)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
