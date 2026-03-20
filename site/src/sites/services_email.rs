use std::borrow::Cow;

use rust_pages::{div, h1, h2, raw_html, page::Page, style::Style, theme::Theme, widget::picture};

use crate::{
    components::{service_card::ServiceCard, site_data::asset_path},
};

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
            raw_html(r#"<p>E-Mail ist und bleibt der entscheidende Nachrichtenkanal fÃƒÂ¼r geschÃƒÂ¤ftliche Kommunikation. Mit unserer Hilfe kommen Ihre E-Mails zuverlÃƒÂ¤ssig an.</p>"#),
            ServiceCard::new("Rahn-IT Mail-Gateway", &data.email, r#"<p>Unser E-Mail-Gateway ist die perfekte ErgÃƒÂ¤nzung fÃƒÂ¼r Ihren Mailserver.</p><h3>Spam-Filter und Virenscanner</h3><p>Eingehende Mails werden von unserem leistungsstarken Spam-Filter geprÃƒÂ¼ft und vor Viren und Betrugsversuchen geschÃƒÂ¼tzt.</p><h3>E-Mails wie aus dem Lehrbuch</h3><p>Unser System sorgt dafÃƒÂ¼r, dass Ihre E-Mails den neuesten Standards entsprechen, digital signiert werden und SPF- und DMARC-Vorgaben erfÃƒÂ¼llen.</p><h3>Immer einen Schritt voraus</h3><p>Dank fortlaufender ÃƒÅ“berwachung geht keine E-Mail mehr spurlos verloren.</p>"#),
            ServiceCard::new("On-Premise Mailserver", &data.mailcow, r#"<p>Wir installieren und verwalten Ihren eigenen E-Mail-Server direkt bei Ihnen im Betrieb.</p><p>Wir nutzen das Mailcow System und kombinieren es bei Bedarf mit unserem Mail-Gateway zu einer leistungsstarken GesamtlÃƒÂ¶sung.</p>"#),
            ServiceCard::new("DMARC-ÃƒÅ“berwachung", &data.dmarc, r#"<p>Mithilfe von DMARC-Berichten kÃƒÂ¶nnen Sie nachvollziehen, ob Ihre E-Mails ankommen und ob jemand unter Ihrem Namen missbrÃƒÂ¤uchlich handelt.</p><p>Wir ÃƒÂ¼bernehmen Einrichtung und Auswertung eingehender DMARC-Berichte.</p>"#),
            h2!("Geht das auch auf Deutsch?"),
            raw_html(r#"<div class="plain-copy"><h3>SMTP</h3><p>Das Protokoll zum Versand von E-Mails. Einfach, alt und ursprÃƒÂ¼nglich ohne gute AbsenderprÃƒÂ¼fung.</p><h3>SPF</h3><p>Legt fest, welche Server unter Ihrem Domainnamen E-Mails versenden dÃƒÂ¼rfen.</p><h3>DKIM</h3><p>Digitale Signaturen fÃƒÂ¼r ausgehende E-Mails, damit Nachrichten nicht unbemerkt manipuliert oder gefÃƒÂ¤lscht werden.</p><h3>DMARC</h3><p>Legt fest, was bei fehlgeschlagener PrÃƒÂ¼fung passiert und liefert Berichte ÃƒÂ¼ber den Zustand Ihrer Mail-Infrastruktur.</p></div>"#)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}
