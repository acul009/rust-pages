use rust_pages::{a, div, h1, h3, p, page::Page, style::Style, theme::Theme, widget::picture};

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

    fn settings(_: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings
            .title("Unsere Prinzipien")
            .description("Erfahren Sie, was uns wichtig ist.");
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere Prinzipien"),
            ServiceCard::<Self>::new("Pragmatische Arbeitsweise", &data.gears).body(div![
                p!("In der IT führen oftmals viele Wege zum Ziel. Selten gibt es \"Die eine Lösung\"."),
                p!("Unsere Ansätze und Lösungen richten sich nach den Bedürfnissen unserer Kunden. Dabei bemühen wir uns um eine gesunde Balance aus erprobten Techniken und modernen Standards. Von uns hören sie kein \"Das haben wir schon immer so gemacht\" und neue Lösungen werden von uns nur dann angeboten, wenn sie es wirklich wert sind."),
                p!("Sollte keine unserer etablierten Lösungen passen, finden wir eine Neue."),
                p!("Denn IT soll funktionieren und Spaß machen.")
            ]),
            ServiceCard::<Self>::new("Ehrlichkeit und Transparenz", &data.transparency).body(div![
                p!("Eine erfolgreiche Zusammenarbeit braucht Vertrauen."),
                p!("Aus diesem Grund setzen wir bei unserer Arbeit auf Ehrlichkeit und Offenheit."),
                p!("Die selbe Ehrlichkeit erwarten wir auch von unseren Kunden."),
                p!("Wir kennen nicht nur die Vorteile, sondern auch die Nachteile unserer Lösungen."),
                p!("Auch unsere Kunden haben ein Recht zu wissen, wo die Schwachstellen Ihrer IT liegen,"),
                p!("denn nur so ist eine informierte Entscheidung möglich."),
                p!("Der Erfolg unserer Kunden ist auch unser Erfolg."),
                p!("Wenn es um die Sicherheit und den Fortbestand Ihres Unternehmens geht, nehmen wir kein Blatt vor den Mund.")
            ]),
            ServiceCard::<Self>::new("Verantwortungsvoller Cloud-Umgang", &data.cloud).body(div![
                p!("Große Firmen wie Microsoft oder Google werben gerne mit Sätzen wie \"Die Cloud ist die Zukunft\" oder \"Ihre Daten sind in der Cloud sicher, geschützt und immer zugänglich\". Aber was ist denn eigentlich \"die Cloud\"?"),
                p!("\"Die Cloud\" ist nichts anderes, als eine Gruppe an Computern, welche nicht Ihnen gehören."),
                p!("Meist stehen die entsprechenden Geräte in einem Rechenzentrum."),
                p!("Für Sie bringt das den Vorteil, dass Sie sich um nichts kümmern müssen. Die technischen Details werden Ihnen abgenommen und versteckt. Dank gutem Marketing wird der Begriff \"Cloud\" gerne mit Sicherheit verbunden."),
                h3!("Sicherheitsrisiken"),
                p!("Doch \"Cloud\" bedeutet nicht automatisch sicher und zuverlässig. Cloud-Dienste sind ein beliebtes Angriffsziel für Cyberkriminelle und Ihre Daten sind dort oftmals nicht verschlüsselt. Sowohl der Betreiber, als auch erfolgreiche Angreifer können also Zugriff auf Ihre Daten bekommen."),
                p!("Ein Beispiel hierfür ist der Diebstahl eines kryptographischen Signing-Schlüssels von Microsoft. Bei dem genannten Vorfall erlangte eine Hackergruppe einen mutmaßlich weitreichenden Zugriff auf die Daten von Microsofts Cloud-Kunden."),
                p![a("Heise: Gestohlener Cloud-Master-Key: Microsoft schweigt – so fragen Sie selbst").href("https://www.heise.de/news/Gestohlener-Cloud-Master-Key-Microsoft-schweigt-so-fragen-Sie-selber-9229395.html")],
                h3!("Sollten wir also auf Cloud verzichten?"),
                p!("So schwarz-weiß ist es natürlich nicht. Einige Cloud-Dienste bieten tolle Vorteile."),
                p!("Selbst wir, als scheinbar große Kritiker, greifen auf Cloud-Dienste zurück und betreiben sogar eigene Cloud-basierte Angebote."),
                p!("Es ist wichtig die Vor- und Nachteile in jedem Fall sorgfältig abzuwägen anstatt blindlings auf das Eine oder das Andere zu setzen.")
            ])
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
