use rust_pages::{a, div, h1, p, page::Page, style::Style, theme::Theme, widget::picture};

use crate::{
    components::{phone::Phone, service_card::ServiceCard, site_data::asset_path},
};

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

    fn path(_: &Self::Data) -> std::path::PathBuf { "services".into() }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            beratung: picture::Handle::create(&asset_path("images/services/beratung.jpg"))?,
            administration: picture::Handle::create(&asset_path("images/services/administration.jpg"))?,
            netzwerk: picture::Handle::create(&asset_path("images/services/netzwerk.jpg"))?,
            backup: picture::Handle::create(&asset_path("images/services/festplatte.jpg"))?,
            security: picture::Handle::create(&asset_path("images/services/security.jpg"))?,
            telefonie: picture::Handle::create(&asset_path("images/services/telefonie.jpg"))?,
            videoueberwachung: picture::Handle::create(&asset_path("images/services/videoueberwachung.jpg"))?,
            programmierung: picture::Handle::create(&asset_path("images/services/programmierung.jpg"))?,
        })
    }

    fn settings(_: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings.title("Leistungen");
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Leistungen"),
            ServiceCard::<Self>::new("Persönliche Beratung", &data.beratung).body(div![
                p!("Eine erfolgreiche Zusammenarbeit braucht Vertrauen."),
                p!("Egal welches Problem oder welche Fragen Sie haben, wir haben immer ein offenes Ohr."),
                p!("Bei uns bekommen Sie eine persönliche und ehrliche Beratung."),
                p!("Unser Ziel ist es, Sie und Ihr Unternehmen zu unterstützen."),
                p!("Dafür gehen wir auf Ihre individuellen Bedürfnisse ein."),
                p!("Noch Fragen?"),
                p!("Rufen Sie uns einfach an:"),
                Phone
            ]),
            ServiceCard::<Self>::new("Administration", &data.administration).body(div![
                p!("In der digitalen Welt ist eine reibungslose IT von entscheidender Bedeutung für Ihr Unternehmen."),
                p!("Überlassen Sie die komplexität Ihrer Infrastruktur uns und konzentrieren Sie sich auf das was zählt!"),
                p!("Wir übernehmen die Wartung und Überwachung Ihrer Server, Arbeitsplätze und Software."),
                p!("Neu, alt, Windows oder Linux - unsere Kompetenzen sind weit gefächert.")
            ]),
            ServiceCard::<Self>::new("Netzwerk", &data.netzwerk).body(div![
                p!("Ihr Netzwerk ist das Rückrad ihrer Infrastruktur. Egal wo Sie arbeiten wollen, Sie benötigen eine Verbindung."),
                p!("Dabei können die Herausforderungen vielseitig sein. Wir unterstützen Sie dabei, einsatzbereit zu bleiben."),
                p!("Wir übernehmen die Planung, Installation und Instandhaltung Ihres Netzwerks."),
                p!("Dabei setzen wir auf intelligente Systeme, um Ihren Bedürfnissen gerecht zu werden.")
            ]),
            ServiceCard::<Self>::new("Backup", &data.backup).body(div![
                p!("Auf das Beste hoffen, auf das Schlimmste vorbereitet sein"),
                p!("Egal wie gut Sie sich vorbereiten, ein Datenverlust kann immer auftreten und bedeutet im schlimmsten Fall das Ende Ihres Unternehmens."),
                p!("Wir setzen mehrschichtige Backups und Redundanzen ein um Ihre Daten zu sichern."),
                p![a("Mit uns sind Sie bereit für den Fall der Fälle - unser Notfalldienst ist immer für Sie da.").href("/emergency")],
                p!("Egal wann, egal wo, im Katastrophenfall stehen wir fest an Ihrer Seite.")
            ]),
            ServiceCard::<Self>::new("Cyber-Sicherheit", &data.security).body(div![
                p!("Von Jahr zu Jahr steigt die Anzahl der Angriffe auf IT-Systeme."),
                p!("Ein erfolgreicher Angriff kann großen Schaden anrichten."),
                p!("Wir sichern Ihre Infrastruktur und minimieren Ihre Angriffsfläche."),
                p!("Dabei setzen wir auf moderne Sicherheitslösungen und intelligente Firewalls."),
                p!("Die Sicherheit Ihres Unternehmens ist Sache aller Beteiligten."),
                p!("Wir beantworten Fragen und schulen Sie und Ihr Personal.")
            ]),
            ServiceCard::<Self>::new("Telefonie", &data.telefonie).body(div![
                p!("Ein erfolgreiches Unternehmen braucht eine vielseitige und zuverlässige Telefonie-Lösung."),
                p!("Wir planen und installieren Ihre Telefonanlage und Ihre Telefoniegeräte."),
                p!("Als Agfeo-Partner bieten wir die nötige Erfahrung und Kompetenz um individuelle Lösungen zu erstellen."),
                p!("Aber auch Ihre vorhandene Infrastruktur können wir pflegen und erweitern - egal von welchem Hersteller.")
            ]),
            ServiceCard::<Self>::new("Videoüberwachung", &data.videoueberwachung).body(div![
                p!("Nicht nur die digitale Sicherheit ist relevant."),
                p!("Behalten Sie von überall aus eine wachsamen Auge auf Ihre Geschäftgebäude."),
                p!("Wir planen und installieren moderne Videoüberwachungssysteme.")
            ]),
            ServiceCard::<Self>::new("Programmierung", &data.programmierung).body(div![
                p!("Manchmal reicht eine fertige Lösung nicht aus."),
                p!("Sie brauchen Software, die speziell auf Sie zugeschnitten ist."),
                p!("Wir entwickeln individuelle Software-Lösungen ganz nach Ihren Bedürfnissen."),
                p!("Dabei setzen wir auf eine Balance aus erprobten Techniken und modernen Standards."),
                p!("Sie sind mit Ihrem Anliegen nicht alleine?"),
                p!("Vereinen Sie Ihre Kräfte mit Anderen und finanzieren Sie gemeinsam eine Open-Source Lösung.")
            ])
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> { vec![] }
}
