use std::borrow::Cow;

use rust_pages::{div, h1, h2, h3, p, page::Page, style::Style, theme::Theme, widget::picture};

use crate::components::{service_card::ServiceCard, site_data::asset_path};

pub struct ServicesEmail;

pub struct Data {
    pub email: picture::Handle,
    pub mailcow: picture::Handle,
    pub dmarc: picture::Handle,
}

impl Page for ServicesEmail {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "services/email".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            email: picture::Handle::create(&asset_path("images/services/email.png"))?,
            mailcow: picture::Handle::create(&asset_path("images/services/mailcow.jpg"))?,
            dmarc: picture::Handle::create(&asset_path("images/services/dmarc.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("E-Mail".into())
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere E-Mail Dienste"),
            p!("E-Mail ist und bleibt der entscheidende Nachrichtenkanal für geschäftliche Kommunikation. Mit unserer Hilfe kommen Ihre E-Mails zuverlässig an."),
            ServiceCard::<Self>::new("Rahn-IT Mail-Gateway", &data.email).body( div![
                p!("Unser E-Mail-Gateway ist die perfekte Ergänzung für Ihren Mailserver."),
                h3!("Spam-Filter und Virenscanner"),
                p!("Eingehende Mails werden von unserem leistungsstarken Spam-Filter geprüft und vor Viren und Betrugsversuchen geschützt."),
                h3!("E-Mails wie aus dem Lehrbuch"),
                p!("Unser System sorgt dafür, dass Ihre E-Mails den neuesten Standards entsprechen, digital signiert werden und SPF- und DMARC-Vorgaben erfüllen."),
                h3!("Immer einen Schritt voraus"),
                p!("Dank fortlaufender Überwachung geht keine E-Mail mehr spurlos verloren.")
            ]),
            ServiceCard::<Self>::new("On-Premise Mailserver", &data.mailcow).body( div![
                p!("Wir installieren und verwalten Ihren eigenen E-Mail-Server direkt bei Ihnen im Betrieb."),
                p!("Wir nutzen das Mailcow System und kombinieren es bei Bedarf mit unserem Mail-Gateway zu einer leistungsstarken Gesamtlösung.")
            ]),
            ServiceCard::<Self>::new("DMARC-Überwachung", &data.dmarc).body( div![
                p!("Mithilfe von DMARC-Berichten können Sie nachvollziehen, ob Ihre E-Mails ankommen und ob jemand unter Ihrem Namen missbräuchlich handelt."),
                p!("Wir übernehmen Einrichtung und Auswertung eingehender DMARC-Berichte.")
            ]),
            h2!("Geht das auch auf Deutsch?"),
            div![
                h3!("SMTP"),
                p!("Das Protokoll zum Versand von E-Mails. Einfach, alt und ursprünglich ohne gute Absenderprüfung."),
                h3!("SPF"),
                p!("Legt fest, welche Server unter Ihrem Domainnamen E-Mails versenden dürfen."),
                h3!("DKIM"),
                p!("Digitale Signaturen für ausgehende E-Mails, damit Nachrichten nicht unbemerkt manipuliert oder gefälscht werden."),
                h3!("DMARC"),
                p!("Legt fest, was bei fehlgeschlagener Prüfung passiert und liefert Berichte über den Zustand Ihrer Mail-Infrastruktur.")
            ]
            .class("plain-copy")
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".plain-copy h3")
                .padding("1.5rem 0 .25rem 0")
                .margin("0"),
        ]
    }
}
