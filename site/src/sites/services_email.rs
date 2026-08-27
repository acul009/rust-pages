use std::borrow::Cow;

use rust_pages::{a, div, h1, h2, h3, p, page::Page, style::Style, theme::Theme, widget::picture};

use crate::components::{phone::Phone, service_card::ServiceCard, site_data::asset_path};

pub struct ServicesEmail;

pub struct Data {
    pub email: picture::Handle,
    pub mailcow: picture::Handle,
    pub dmarc: picture::Handle,
}

impl Page for ServicesEmail {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf { "services/email".into() }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            email: picture::Handle::create(&asset_path("images/services/email.png"))?,
            mailcow: picture::Handle::create(&asset_path("images/services/mailcow.jpg"))?,
            dmarc: picture::Handle::create(&asset_path("images/services/dmarc.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("E-Mail".into()) }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere E-Mail Dienste"),
            p!("E-Mail ist und bleibt der entscheidende Nachrichtenkanal für geschäftliche Kommunikation. Mit unserer Hilfe kommen Ihre E-Mails zuverlässig an."),
            ServiceCard::<Self>::new("Rahn-IT Mail-Gateway", &data.email).body(div![
                p!("Unser E-Mail-Gateway ist die perfekte Ergänzung für Ihren Mailserver."),
                h3!("Spam-Filter und Virenscanner"),
                p!("Eingehende Mails werden von unserem leistungsstarken Spam-Filter unter die Lupe genommen."),
                p!("Dieser sortiert nicht nur große Mengen an ungewollter Werbung aus, sondern schützt auch vor Viren und Betrugsversuchen."),
                h3!("E-Mails wie aus dem Lehrbuch"),
                p!("Unser System sorgt dafür, dass Ihre E-Mails den neuesten Standarts entsprechen."),
                p!("Jede E-Mail wird mittels dem DKIM-Verfahren digital signiert und entspricht den nötigen SPF- und DMARC-Vorgaben."),
                p!("So können Sie sichergehen, dass Ihre E-Mails nie wieder grundlos im Spam-Ordner landen."),
                h3!("Immer einen Schritt voraus"),
                p!("Dank fortlaufender Überwachung geht keine E-Mail mehr spurlos verloren."),
                p!("Wir behalten ein wachsames Auge auf Ihren Mailverkehr, passen den Spam-Filter laufend an und schalten uns bei auffallenden Problemen selbstständig ein.")
            ]),
            ServiceCard::<Self>::new("On-Premise Mailserver", &data.mailcow).body(div![
                p!("Sie benötigen einen eigenen Mailserver?"),
                p!("Wir installieren und verwalten ihren ganz eigenen E-Mail-Server direkt bei Ihnen im Betrieb."),
                p!("So bleiben Sie unabhängig von großen Anbietern und können so viele E-Mails speichern, wie auf Ihre Festplatten passen."),
                p!("Wir nutzen das hervorragende Mailcow System und kombinieren es bei Bedarf mit unserem Mail-Gateway zu einer leistungsstarken Gesamtlösung."),
                p!("Mailcow setzt auf in der Branche verbreitete und Bewährte Software wie Postfix und Dovecot. Und bietet die perfekte Mischung aus Bedienbarkeit und, Flexibilität und Zuverlässigkeit."),
                p!("In Kombination mit unserem E-Mail-Gateway erhalten Sie eine leistungsstarke E-Mail-Lösung.")
            ]),
            ServiceCard::<Self>::new("DMARC-Überwachung", &data.dmarc).body(div![
                p!("Haben Sie kein Interesse an unserem Gateway, aber Ihre E-Mails landen trotzdem im Spam-Ordner?"),
                p!("Mithilfe von DMARC-Berichten können Sie nicht nur feststellen, ob Ihre E-Mails auch ankommen, sondern kommen auch Betrügern, die unter Ihrem Namen handeln schnell auf die Schliche."),
                p!("Wir übernehmen für Sie die Einrichtung und Auswertung eingehender DMARC-Berichte."),
                p!("Dadurch sind Sie stets informiert, sollte etwas an Ihrem Mail-System nicht stimmen.")
            ]),
            h2!("Geht das auch auf Deutsch?"),
            p!("Wer nicht gerade einen Mail-Server betreibt mag sich wundern, was denn die ganzen Abkürzungen bedeuten sollen. Damit Sie nicht im Dunkeln tappen, haben wir die ausgeschriebenen Abkürzungen noch einmal sinngemäß übersetzt."),
            div![
                h3!("SMTP – Einfaches Post-Transport-Protokoll"),
                p!("SMTP wird zum Versand von E-Mails verwendet. Das alte und bewusst einfache Protokoll bietet von sich aus keine Möglichkeit zu prüfen, ob der Absender korrekt ist."),
                h3!("SPF – Rahmenwerk zur Absender-Prüfung"),
                p!("Als Besitzer einer Internet-Domäne können Sie angeben, welche Server berechtigt sind, unter Ihrem Namen E-Mails zu versenden. Der Empfänger gleicht den sendenden Server mit dieser Liste ab."),
                h3!("DKIM – Von der Domäne signierte E-Mails"),
                p!("Mit DKIM werden ausgehende E-Mails digital signiert. So lässt sich erkennen, ob Nachrichten manipuliert oder gefälscht wurden."),
                h3!("DMARC – Berichterstattung und Konformität"),
                p!("DMARC legt fest, was bei fehlgeschlagener Verifizierung passiert, und liefert Berichte über den Zustand Ihrer Mail-Infrastruktur."),
                h2!("Noch Fragen?"),
                p!("Sind Sie sich unsicher, ob Sie einen unserer Dienste benötigen? Rufen Sie uns an:"),
                Phone,
                p![a("Mehr über Mailcow").href("https://mailcow.email/")]
            ].class("plain-copy")
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}
