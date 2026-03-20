use std::borrow::Cow;

use rust_pages::{div, h1, page::Page, style::Style, theme::Theme, widget::picture};

use crate::content::{asset_path, ServiceCard};

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
            administration: picture::Handle::create(&asset_path("images/services/administration.jpg"))?,
            netzwerk: picture::Handle::create(&asset_path("images/services/netzwerk.jpg"))?,
            backup: picture::Handle::create(&asset_path("images/services/festplatte.jpg"))?,
            security: picture::Handle::create(&asset_path("images/services/security.jpg"))?,
            telefonie: picture::Handle::create(&asset_path("images/services/telefonie.jpg"))?,
            videoueberwachung: picture::Handle::create(&asset_path("images/services/videoueberwachung.jpg"))?,
            programmierung: picture::Handle::create(&asset_path("images/services/programmierung.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Leistungen".into())
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Leistungen"),
            ServiceCard::new("Persönliche Beratung", &data.beratung, r#"<p>Eine erfolgreiche Zusammenarbeit braucht Vertrauen. Egal welches Problem oder welche Fragen Sie haben, wir haben immer ein offenes Ohr.</p><p>Bei uns bekommen Sie eine <b>persönliche</b> und <b>ehrliche</b> Beratung. Unser Ziel ist es, Sie und Ihr Unternehmen zu unterstützen.</p><div class="callout"><p>Noch Fragen? Rufen Sie uns einfach an.</p></div>"#),
            ServiceCard::new("Administration", &data.administration, r#"<p>In der digitalen Welt ist eine reibungslose IT von entscheidender Bedeutung für Ihr Unternehmen.</p><p>Wir übernehmen die Wartung und Überwachung Ihrer Server, Arbeitsplätze und Software. Neu, alt, Windows oder Linux: unsere Kompetenzen sind weit gefächert.</p>"#),
            ServiceCard::new("Netzwerk", &data.netzwerk, r#"<p>Ihr Netzwerk ist das Rückgrat Ihrer Infrastruktur. Egal wo Sie arbeiten wollen, Sie benötigen eine Verbindung.</p><p>Wir übernehmen die Planung, Installation und Instandhaltung Ihres Netzwerks und setzen auf intelligente Systeme, um Ihren Bedürfnissen gerecht zu werden.</p>"#),
            ServiceCard::new("Backup", &data.backup, r#"<p><i>Auf das Beste hoffen, auf das Schlimmste vorbereitet sein.</i></p><p>Ein Datenverlust kann im schlimmsten Fall das Ende Ihres Unternehmens bedeuten. Wir setzen mehrschichtige Backups und Redundanzen ein, um Ihre Daten zu sichern.</p><p><a href="/emergency">Mit uns sind Sie bereit für den Fall der Fälle.</a></p>"#),
            ServiceCard::new("Cyber-Sicherheit", &data.security, r#"<p>Von Jahr zu Jahr steigt die Anzahl der Angriffe auf IT-Systeme. Ein erfolgreicher Angriff kann großen Schaden anrichten.</p><p>Wir sichern Ihre Infrastruktur, minimieren Ihre Angriffsfläche und beantworten Fragen rund um moderne Sicherheitslösungen.</p>"#),
            ServiceCard::new("Telefonie", &data.telefonie, r#"<p>Ein erfolgreiches Unternehmen braucht eine vielseitige und zuverlässige Telefonie-Lösung.</p><p>Wir planen und installieren Ihre Telefonanlage und Ihre Telefoniegeräte. Als Agfeo-Partner bieten wir Erfahrung für individuelle Lösungen.</p>"#),
            ServiceCard::new("Videoüberwachung", &data.videoueberwachung, r#"<p>Nicht nur die digitale Sicherheit ist relevant.</p><p>Wir planen und installieren moderne Videoüberwachungssysteme, damit Sie von überall aus ein wachsames Auge auf Ihre Geschäftsgebäude behalten.</p>"#),
            ServiceCard::new("Programmierung", &data.programmierung, r#"<p>Manchmal reicht eine fertige Lösung nicht aus. Sie brauchen Software, die speziell auf Sie zugeschnitten ist.</p><p>Wir entwickeln individuelle Software-Lösungen ganz nach Ihren Bedürfnissen und setzen dabei auf eine Balance aus erprobten Techniken und modernen Standards.</p>"#)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![]
    }
}
