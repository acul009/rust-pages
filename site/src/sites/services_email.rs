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
            p!("E-Mail ist und bleibt der entscheidende Nachrichtenkanal fÃƒÆ’Ã‚Â¼r geschÃƒÆ’Ã‚Â¤ftliche Kommunikation. Mit unserer Hilfe kommen Ihre E-Mails zuverlÃƒÆ’Ã‚Â¤ssig an."),
            ServiceCard::new("Rahn-IT Mail-Gateway", &data.email, div![
                p!("Unser E-Mail-Gateway ist die perfekte ErgÃƒÆ’Ã‚Â¤nzung fÃƒÆ’Ã‚Â¼r Ihren Mailserver."),
                h3!("Spam-Filter und Virenscanner"),
                p!("Eingehende Mails werden von unserem leistungsstarken Spam-Filter geprÃƒÆ’Ã‚Â¼ft und vor Viren und Betrugsversuchen geschÃƒÆ’Ã‚Â¼tzt."),
                h3!("E-Mails wie aus dem Lehrbuch"),
                p!("Unser System sorgt dafÃƒÆ’Ã‚Â¼r, dass Ihre E-Mails den neuesten Standards entsprechen, digital signiert werden und SPF- und DMARC-Vorgaben erfÃƒÆ’Ã‚Â¼llen."),
                h3!("Immer einen Schritt voraus"),
                p!("Dank fortlaufender ÃƒÆ’Ã…â€œberwachung geht keine E-Mail mehr spurlos verloren.")
            ]),
            ServiceCard::new("On-Premise Mailserver", &data.mailcow, div![
                p!("Wir installieren und verwalten Ihren eigenen E-Mail-Server direkt bei Ihnen im Betrieb."),
                p!("Wir nutzen das Mailcow System und kombinieren es bei Bedarf mit unserem Mail-Gateway zu einer leistungsstarken GesamtlÃƒÆ’Ã‚Â¶sung.")
            ]),
            ServiceCard::new("DMARC-ÃƒÆ’Ã…â€œberwachung", &data.dmarc, div![
                p!("Mithilfe von DMARC-Berichten kÃƒÆ’Ã‚Â¶nnen Sie nachvollziehen, ob Ihre E-Mails ankommen und ob jemand unter Ihrem Namen missbrÃƒÆ’Ã‚Â¤uchlich handelt."),
                p!("Wir ÃƒÆ’Ã‚Â¼bernehmen Einrichtung und Auswertung eingehender DMARC-Berichte.")
            ]),
            h2!("Geht das auch auf Deutsch?"),
            div![
                h3!("SMTP"),
                p!("Das Protokoll zum Versand von E-Mails. Einfach, alt und ursprÃƒÆ’Ã‚Â¼nglich ohne gute AbsenderprÃƒÆ’Ã‚Â¼fung."),
                h3!("SPF"),
                p!("Legt fest, welche Server unter Ihrem Domainnamen E-Mails versenden dÃƒÆ’Ã‚Â¼rfen."),
                h3!("DKIM"),
                p!("Digitale Signaturen fÃƒÆ’Ã‚Â¼r ausgehende E-Mails, damit Nachrichten nicht unbemerkt manipuliert oder gefÃƒÆ’Ã‚Â¤lscht werden."),
                h3!("DMARC"),
                p!("Legt fest, was bei fehlgeschlagener PrÃƒÆ’Ã‚Â¼fung passiert und liefert Berichte ÃƒÆ’Ã‚Â¼ber den Zustand Ihrer Mail-Infrastruktur.")
            ]
            .class("plain-copy")
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}
