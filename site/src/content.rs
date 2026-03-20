use std::{borrow::Cow, path::Path};

use rust_pages::{
    a, br, div, h1, h2, p, picture as picture_widget, raw_html,
    page::Page,
    style::Style,
    theme::Theme,
    widget::{picture, Component, ToElement},
};

use crate::logo::{LogoFull, LogoStandalone};

pub const NAME: &str = "Rahn IT-Systemtechnik";
pub const STREET: &str = "Alte Bahn 9";
pub const POSTAL_CODE: &str = "84577";
pub const CITY: &str = "Tüßling";
pub const PHONE: &str = "08633 / 977969 - 0";
pub const MAIL: &str = "info@it-rahn.de";
pub const UST_ID: &str = "DE206337243";
pub const IBAN: &str = "DE95 7601 0085 0059 3418 53";

pub fn asset_path(relative: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(relative)
        .to_string_lossy()
        .into_owned()
}

fn tel_href() -> String {
    format!("tel:+49{}", &PHONE[1..].replace([' ', '/'], ""))
}

pub struct Footer;

impl Component for Footer {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![
            div![
                div![
                    a(LogoStandalone).href("/").class("footer-logo"),
                    p!(NAME),
                    a(format!("Tel: {}", PHONE)).href(tel_href())
                ]
                .class("footer-brand footer-group"),
                div![
                    a("Home").href("/"),
                    a("Notfalldienst").href("/emergency"),
                    a("Dienste").href("/services"),
                    a("E-Mail").href("/services/email"),
                    a("Linux").href("/services/linux")
                ]
                .class("footer-group"),
                div![
                    a("Preise").href("/pricing"),
                    a("Fernwartung").href("/remote"),
                    a("Unsere Prinzipien").href("/about-us"),
                    a("Das Team").href("/about-us/people"),
                    a("Kontakt").href("/contact"),
                    a("Impressum").href("/impressum")
                ]
                .class("footer-group")
            ]
            .class("footer-inner")
        ]
        .class("site-footer")
    }

    fn style(&self, theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".site-footer")
                .width_full()
                .padding("2.5rem 2rem")
                .background_color(theme.primary_color()),
            Style::new(".footer-inner")
                .flex()
                .flex_wrap("wrap")
                .justify_content("flex-start")
                .align_items("flex-start")
                .gap("2rem 6rem")
                .width_full()
                .max_width("80rem")
                .margin("0 auto")
                .font_size("1rem"),
            Style::new(".footer-group")
                .flex()
                .flex_column()
                .gap(".4rem")
                .line_height("1.2")
                .property("align-items", "flex-start"),
            Style::new(".footer-brand")
                .min_width("14rem")
                .gap(".15rem")
                .line_height("1.05"),
            Style::new(".footer-group a").color_inherit().text_decoration_none(),
            Style::new(".footer-group a:hover").text_decoration_underline(),
            Style::new(".footer-group p").margin("0"),
            Style::new(".footer-brand p, .footer-brand a")
                .line_height("1.05"),
            Style::new(".footer-logo")
                .height("3.5rem")
                .padding(".25rem")
                .border_radius(".5rem")
                .block(),
            Style::new(".footer-logo:hover")
                .background_color(theme.interactive_hover_color()),
            Style::new(".footer-logo svg").width("auto").height_full(),
        ]
    }
}

pub struct ServiceCard<'a> {
    title: &'a str,
    image: &'a picture::Handle,
    body_html: &'a str,
}

impl<'a> ServiceCard<'a> {
    pub fn new(title: &'a str, image: &'a picture::Handle, body_html: &'a str) -> Self {
        Self {
            title,
            image,
            body_html,
        }
    }
}

impl Component for ServiceCard<'_> {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![
            div![picture_widget(self.image).class("service-image")].class("service-figure"),
            div![h2!(self.title), raw_html(self.body_html)].class("service-body")
        ]
        .class("service-card")
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".service-card")
                .position_relative()
                .margin("1rem 0")
                .border_radius("1.5rem")
                .property("overflow", "hidden")
                .property("min-height", "24rem")
                .box_shadow("0 1rem 3rem rgba(0,0,0,0.25)"),
            Style::new(".service-figure")
                .position_absolute()
                .property("inset", "0"),
            Style::new(".service-image, .service-image picture, .service-image img")
                .width_full()
                .height_full(),
            Style::new(".service-image img").property("object-fit", "cover"),
            Style::new(".service-body")
                .position_relative()
                .property("z-index", "1")
                .padding("2rem")
                .color("white")
                .background("linear-gradient(135deg, rgba(15,23,42,0.8), rgba(15,23,42,0.45))")
                .property("backdrop-filter", "blur(6px)")
                .property("min-height", "24rem"),
            Style::new(".service-body h2")
                .padding("0 0 1rem 0")
                .margin("0")
                .text_align_left()
                .font_size("2rem"),
            Style::new(".service-body h3")
                .padding("1rem 0 .25rem 0")
                .margin("0")
                .font_size("1.35rem"),
            Style::new(".service-body p").padding(".35rem 0").margin("0"),
            Style::new(".service-body a")
                .color("inherit")
                .text_decoration_underline(),
            Style::new(".service-body .callout")
                .margin("1rem 0 0 0")
                .padding("1rem 1.25rem")
                .border_radius("1rem")
                .background_color("rgba(255,255,255,0.08)"),
        ]
    }
}

pub struct PersonCard<'a> {
    image: &'a picture::Handle,
    name: &'a str,
    caption: &'a str,
    body_html: &'a str,
}

impl<'a> PersonCard<'a> {
    pub fn new(
        image: &'a picture::Handle,
        name: &'a str,
        caption: &'a str,
        body_html: &'a str,
    ) -> Self {
        Self {
            image,
            name,
            caption,
            body_html,
        }
    }
}

impl Component for PersonCard<'_> {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![
            div![picture_widget(self.image).class("person-image")].class("person-figure"),
            div![
                h2!(self.name),
                raw_html(format!("<p class=\"caption\"><i>{}</i></p>{}", self.caption, self.body_html).leak())
            ]
            .class("person-body")
        ]
        .class("person-card")
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".person-card")
                .flex()
                .gap("0")
                .margin("2.5rem 0")
                .border_radius("1.75rem")
                .background_color("rgba(255,255,255,0.06)")
                .property("overflow", "hidden")
                .box_shadow("0 1rem 3rem rgba(0,0,0,0.2)"),
            Style::new(".person-figure").width("22rem").flex_basis("22rem"),
            Style::new(".person-image, .person-image picture, .person-image img")
                .width_full()
                .height_full(),
            Style::new(".person-image img").property("object-fit", "cover"),
            Style::new(".person-body").padding("2rem"),
            Style::new(".person-body h2")
                .padding("0")
                .margin("0")
                .text_align_left(),
            Style::new(".person-body .caption").padding(".5rem 0 1rem 0").margin("0"),
            Style::new(".person-body p").padding(".35rem 0").margin("0"),
        ]
    }
}

macro_rules! simple_page {
    ($name:ident, $path:literal, $title:literal, $view:block, [$($style:expr),* $(,)?]) => {
        pub struct $name;
        impl Page for $name {
            type Data = ();
            fn path(_: &Self::Data) -> std::path::PathBuf { $path.into() }
            fn load_data(&self) -> anyhow::Result<Self::Data> { Ok(()) }
            fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some($title.into()) }
            fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> $view
            fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> { vec![$($style),*] }
        }
    };
}

simple_page!(Emergency, "emergency", "24 Stunden Notdienst", {
    div![
        h1!("Im Notfall immer für Sie da"),
        p!("Steht Ihr Betrieb still weil Ihre IT streikt?", br(), "Wir bieten schnelle und professionelle Hilfe bei dringenden Problemen."),
        p!("Falls es sich nicht um einen dringenden Notfall handelt, nutzen Sie bitte unsere regulären Geschäftszeiten."),
        p!("Mo-Do: 09:00 - 16:30", br(), "Fr: 09:00 - 12:00", br(), "24 Stunden Notdienst", br(), "Auch an Sonn- und Feiertagen"),
        a(format!("Tel: {}", PHONE)).href(tel_href()).class("cta")
    ]
}, [
    Style::new(".cta").display("inline-block").padding("1rem 1.5rem").border_radius("1rem").background_color("rgba(255,255,255,0.08)").text_decoration_none()
]);

simple_page!(Pricing, "pricing", "Preise", {
    div![
        h1!("Unsere Preise sind kein Geheimnis."),
        raw_html(r#"<div class="price-list"><div class="price-row"><div><strong>Anfahrt pro KM</strong></div><div class="amount">0,71 €</div></div><div class="price-row"><div><strong>Dienstleistung pro Stunde</strong><p>Bei einer Anfahrt <b>unter 25 km</b> werden mindestens 0,25 Stunden berechnet.</p><p>Bei einer Anfahrt <b>ab 25 km</b> werden mindestens 0,5 Stunden berechnet.</p></div><div class="amount">107,10 €</div></div></div>"#),
        p!("Unsere Preise für Geräte richten sich nach unseren aktuellen Einkaufspreisen."),
        p!("Für ein Angebot, rufen Sie uns an oder schreiben Sie uns eine E-Mail."),
        raw_html(format!(r#"<p><a href="{}">Tel: {}</a></p><p><a href="mailto:{}">E-Mail: {}</a></p>"#, tel_href(), PHONE, MAIL, MAIL).leak())
    ]
}, [
    Style::new(".price-list").margin("2rem 0"),
    Style::new(".price-row").grid().property("grid-template-columns", "1fr auto").gap("1rem 2rem").padding("1.25rem 0").property("border-top", "1px solid rgba(255,255,255,0.15)"),
    Style::new(".price-row:last-child").property("border-bottom", "1px solid rgba(255,255,255,0.15)"),
    Style::new(".amount").font_size("1.35rem").text_align_right()
]);

simple_page!(Remote, "remote", "Fernwartung", {
    div![
        h1!("Fernwartungs-Software"),
        p!("Hier finden Sie unsere Fernwartungs-Software zum Download."),
        h2!("Schnelle Fernwartung"),
        p!("Diese Software erlaubt es uns, Ihnen zu helfen, sobald Sie die Software starten."),
        a("Download").href("https://get.teamviewer.com/57y9u6n").class("cta"),
        h2!("Dauerhafte Fernwartung"),
        p!("Dieses Installationsprogramm richtet eine dauerhafte Fernwartung ein.", br(), "Dadurch können wir uns auch ohne Ihr Zutun um Ihre Geräte kümmern."),
        a("Download").href("https://get.teamviewer.com/q3zt6wn").class("cta"),
        raw_html(r#"<p class="todo">TODO: Guided download / install flow if interactive behavior is desired.</p>"#)
    ]
}, [
    Style::new(".cta").display("inline-block").margin("1rem 0 2rem 0").padding("1rem 1.5rem").border_radius("1rem").background_color("rgba(255,255,255,0.08)").text_decoration_none(),
    Style::new(".todo").property("opacity", ".75")
]);

simple_page!(Contact, "contact", "Kontakt", {
    div![
        h1!("Kontakt"),
        div![
            div![LogoFull].class("contact-logo"),
            raw_html(format!(r#"<div class="contact-copy"><address><div>{}</div><div>{}</div><div>{} {}</div></address><h2>Kontaktinformationen</h2><p><a href="mailto:{}">E-Mail: {}</a></p><p><a href="{}">Tel: {}</a></p><h2>Geschäftszeiten</h2><p>Mo-Do: 09:00 - 16:30<br>Fr: 09:00 - 12:00<br><b>24 Stunden Notdienst</b></p><p class="todo">TODO: vCard-Download wieder ergänzen.</p></div>"#, NAME, STREET, POSTAL_CODE, CITY, MAIL, MAIL, tel_href(), PHONE).leak())
        ]
        .class("contact-card")
    ]
}, [
    Style::new(".contact-card").flex().margin("3rem 0").border_radius("1.75rem").background_color("rgba(255,255,255,0.06)").property("overflow", "hidden"),
    Style::new(".contact-logo").width("22rem").padding("3rem"),
    Style::new(".contact-logo svg").width_full().height("auto"),
    Style::new(".contact-copy").padding("2rem"),
    Style::new(".contact-copy h2").padding("1.5rem 0 .25rem 0").margin("0"),
    Style::new(".contact-copy a").color_inherit().text_decoration_underline(),
    Style::new(".todo").property("opacity", ".75")
]);

simple_page!(Impressum, "impressum", "Impressum", {
    div![
        h1!("Impressum"),
        raw_html(format!(r#"<table class="legal-table"><tr><td>Vertreter</td><td>Heinz Rahn</td></tr><tr><td>Adresse</td><td>{}<br>{}<br>{} {}</td></tr><tr><td>Telefon</td><td><a href="{}">{}</a></td></tr><tr><td>E-Mail</td><td><a href="mailto:{}">{}</a></td></tr><tr><td>USt-ID</td><td>{}</td></tr><tr><td>IBAN</td><td>{}</td></tr><tr><td>Erstellt mit</td><td>Rust Pages<br>TODO: Technologie-Hinweis bei Bedarf ergänzen.</td></tr></table>"#, NAME, STREET, POSTAL_CODE, CITY, tel_href(), PHONE, MAIL, MAIL, UST_ID, IBAN).leak())
    ]
}, [
    Style::new(".legal-table").width_full().property("border-collapse", "collapse").margin("2rem 0"),
    Style::new(".legal-table td").padding("1rem 0").property("vertical-align", "top").property("border-top", "1px solid rgba(255,255,255,0.15)"),
    Style::new(".legal-table tr:last-child td").property("border-bottom", "1px solid rgba(255,255,255,0.15)"),
    Style::new(".legal-table td:first-child").width("12rem").font_size("1rem")
]);

pub struct About;
pub struct AboutData {
    pub gears: picture::Handle,
    pub transparency: picture::Handle,
    pub cloud: picture::Handle,
}
impl Page for About {
    type Data = AboutData;
    fn path(_: &Self::Data) -> std::path::PathBuf { "about-us".into() }
    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(AboutData {
            gears: picture::Handle::create(&asset_path("images/services/gears.jpg"))?,
            transparency: picture::Handle::create(&asset_path("images/services/transparenz2.jpg"))?,
            cloud: picture::Handle::create(&asset_path("images/services/cloud.jpg"))?,
        })
    }
    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("Unsere Prinzipien".into()) }
    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere Prinzipien"),
            ServiceCard::new("Pragmatische Arbeitsweise", &data.gears, r#"<p>In der IT führen oftmals viele Wege zum Ziel. Selten gibt es die eine Lösung.</p><p>Unsere Ansätze und Lösungen richten sich nach den Bedürfnissen unserer Kunden. Dabei bemühen wir uns um eine gesunde Balance aus erprobten Techniken und modernen Standards.</p><p>Sollte keine unserer etablierten Lösungen passen, finden wir eine neue. Denn IT soll funktionieren und Spaß machen.</p>"#),
            ServiceCard::new("Ehrlichkeit und Transparenz", &data.transparency, r#"<p>Eine erfolgreiche Zusammenarbeit braucht Vertrauen. Aus diesem Grund setzen wir bei unserer Arbeit auf Ehrlichkeit und Offenheit.</p><p>Wir kennen nicht nur die Vorteile, sondern auch die Nachteile unserer Lösungen. Auch unsere Kunden haben ein Recht zu wissen, wo die Schwachstellen ihrer IT liegen.</p><p>Wenn es um die Sicherheit und den Fortbestand Ihres Unternehmens geht, nehmen wir kein Blatt vor den Mund.</p>"#),
            ServiceCard::new("Verantwortungsvoller Cloud-Umgang", &data.cloud, r#"<p>Cloud-Dienste können sinnvoll sein, aber sie sind kein Allheilmittel. Für uns zählt eine saubere Abwägung von Chancen, Risiken und Abhängigkeiten.</p><h3>Sicherheitsrisiken</h3><p>Cloud-Plattformen sind beliebte Angriffsziele. Betreiber und Angreifer können im Ernstfall Zugang zu sensiblen Daten erlangen.</p><h3>Sollten wir also auf Cloud verzichten?</h3><p>Nein. Einige Cloud-Dienste bieten echte Vorteile. Wichtig ist, nicht blind auf Schlagworte zu vertrauen, sondern jede Lösung verantwortungsvoll zu bewerten.</p>"#)
        ]
    }
    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> { vec![] }
}

pub struct Team;
pub struct TeamData {
    pub heinz: picture::Handle,
    pub luca: picture::Handle,
}
impl Page for Team {
    type Data = TeamData;
    fn path(_: &Self::Data) -> std::path::PathBuf { "about-us/people".into() }
    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(TeamData {
            heinz: picture::Handle::create(&asset_path("../old/src/lib/images/people/heinz.jpg"))?,
            luca: picture::Handle::create(&asset_path("../old/src/lib/images/people/luca.jpg"))?,
        })
    }
    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("Unser Team".into()) }
    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unser Team"),
            PersonCard::new(&data.heinz, "Heinz Rahn", "Windows-Administrator, IT-Fachmann seit 1983", r#"<p>Mein erster Computer war der Sinclair ZX81, ein 8-Bit Computer, den ich damals noch per Hand lötete.</p><p>Während meiner Laufbahn bekam ich den Einzug des Computers in alle Lebensbereiche hautnah mit. Nachdem immer mehr Freunde und Bekannte mit ihren Anliegen zu mir kamen, beschloss ich schließlich mein Hobby zum Beruf zu machen.</p><p>2004 machte ich mich schließlich selbstständig und eröffnete Rahn IT-Systemtechnik.</p>"#),
            PersonCard::new(&data.luca, "Luca Wlcek", "Linux-Administrator, Webentwickler", r#"<p>Mit Heinz als Vater bin ich von klein auf mit Computern groß geworden.</p><p>In meiner Jugend kam mein Interesse für Programmierung und die Funktionsweise moderner Computer auf. Während meiner Ausbildung entdeckte ich Linux und die Open-Source Welt.</p><p>Inzwischen bin ich langjähriger Linux-Nutzer und kümmere mich im Betrieb um die Linux-Infrastruktur.</p>"#)
        ]
    }
    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> { vec![] }
}

pub struct ServicesEmail;
pub struct ServicesEmailData {
    pub email: picture::Handle,
    pub mailcow: picture::Handle,
    pub dmarc: picture::Handle,
}
impl Page for ServicesEmail {
    type Data = ServicesEmailData;
    fn path(_: &Self::Data) -> std::path::PathBuf { "services/email".into() }
    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(ServicesEmailData {
            email: picture::Handle::create(&asset_path("images/services/email.png"))?,
            mailcow: picture::Handle::create(&asset_path("images/services/mailcow.jpg"))?,
            dmarc: picture::Handle::create(&asset_path("images/services/dmarc.jpg"))?,
        })
    }
    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("E-Mail".into()) }
    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Unsere E-Mail Dienste"),
            raw_html(r#"<p>E-Mail ist und bleibt der entscheidende Nachrichtenkanal für geschäftliche Kommunikation. Mit unserer Hilfe kommen Ihre E-Mails zuverlässig an.</p>"#),
            ServiceCard::new("Rahn-IT Mail-Gateway", &data.email, r#"<p>Unser E-Mail-Gateway ist die perfekte Ergänzung für Ihren Mailserver.</p><h3>Spam-Filter und Virenscanner</h3><p>Eingehende Mails werden von unserem leistungsstarken Spam-Filter geprüft und vor Viren und Betrugsversuchen geschützt.</p><h3>E-Mails wie aus dem Lehrbuch</h3><p>Unser System sorgt dafür, dass Ihre E-Mails den neuesten Standards entsprechen, digital signiert werden und SPF- und DMARC-Vorgaben erfüllen.</p><h3>Immer einen Schritt voraus</h3><p>Dank fortlaufender Überwachung geht keine E-Mail mehr spurlos verloren.</p>"#),
            ServiceCard::new("On-Premise Mailserver", &data.mailcow, r#"<p>Wir installieren und verwalten Ihren eigenen E-Mail-Server direkt bei Ihnen im Betrieb.</p><p>Wir nutzen das Mailcow System und kombinieren es bei Bedarf mit unserem Mail-Gateway zu einer leistungsstarken Gesamtlösung.</p>"#),
            ServiceCard::new("DMARC-Überwachung", &data.dmarc, r#"<p>Mithilfe von DMARC-Berichten können Sie nachvollziehen, ob Ihre E-Mails ankommen und ob jemand unter Ihrem Namen missbräuchlich handelt.</p><p>Wir übernehmen Einrichtung und Auswertung eingehender DMARC-Berichte.</p>"#),
            h2!("Geht das auch auf Deutsch?"),
            raw_html(r#"<div class="plain-copy"><h3>SMTP</h3><p>Das Protokoll zum Versand von E-Mails. Einfach, alt und ursprünglich ohne gute Absenderprüfung.</p><h3>SPF</h3><p>Legt fest, welche Server unter Ihrem Domainnamen E-Mails versenden dürfen.</p><h3>DKIM</h3><p>Digitale Signaturen für ausgehende E-Mails, damit Nachrichten nicht unbemerkt manipuliert oder gefälscht werden.</p><h3>DMARC</h3><p>Legt fest, was bei fehlgeschlagener Prüfung passiert und liefert Berichte über den Zustand Ihrer Mail-Infrastruktur.</p></div>"#)
        ]
    }
    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}

pub struct ServicesLinux;
pub struct ServicesLinuxData {
    pub proxmox: picture::Handle,
    pub linux: picture::Handle,
}
impl Page for ServicesLinux {
    type Data = ServicesLinuxData;
    fn path(_: &Self::Data) -> std::path::PathBuf { "services/linux".into() }
    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(ServicesLinuxData {
            proxmox: picture::Handle::create(&asset_path("images/services/proxmox.jpg"))?,
            linux: picture::Handle::create(&asset_path("images/services/linux.jpg"))?,
        })
    }
    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("Linux und Proxmox".into()) }
    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Linux und Proxmox"),
            ServiceCard::new("Proxmox-VE", &data.proxmox, r#"<p>Mit unserer Expertise übernehmen wir Einrichtung, Wartung und Überwachung Ihrer Proxmox-Infrastruktur.</p><h3>Hohe Zuverlässigkeit</h3><p>Proxmox basiert auf Debian und KVM und bietet eine robuste Virtualisierungsumgebung.</p><h3>Überlegene Datenintegrität</h3><p>Mit ZFS und dem Proxmox Backup Server erhalten Sie hervorragenden Schutz Ihrer Daten.</p>"#),
            ServiceCard::new("Linux-Server", &data.linux, r#"<p>Sie haben Interesse am Einsatz von Linux oder suchen jemanden, der Ihre vorhandene Infrastruktur betreut? Dann sind Sie bei uns richtig.</p><h3>Flexibilität</h3><p>Von Datenbanken über Webserver bis zur Nextcloud: Linux-Server sind enorm flexibel.</p><h3>Effizienz</h3><p>Linux-Systeme können äußerst ressourcenschonend sein und bestehende Windows-Netzwerke hervorragend ergänzen.</p>"#),
            h2!("Warum Linux?"),
            raw_html(r#"<div class="plain-copy"><h3>Linux ist zuverlässig</h3><p>Unternehmen aus der ganzen Welt setzen auf Linux, darunter Google, Amazon und Microsoft.</p><h3>Linux ist sicher</h3><p>Linux-Systeme bieten starke Rechtekonzepte, Sandbox-Mechanismen und eine geringe Angriffsfläche.</p><h3>Linux ist unabhängig</h3><p>Linux ist Open Source. Niemand kann Ihnen die Lizenz entziehen, und Ihre Infrastruktur bleibt souverän.</p></div>"#)
        ]
    }
    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}
