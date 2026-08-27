use std::borrow::Cow;

use rust_pages::{div, h1, h2, h3, p, page::Page, style::Style, theme::Theme, widget::picture};

use crate::components::{phone::Phone, service_card::ServiceCard, site_data::asset_path};

pub struct ServicesLinux;

pub struct Data {
    pub proxmox: picture::Handle,
    pub linux: picture::Handle,
}

impl Page for ServicesLinux {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf { "services/linux".into() }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            proxmox: picture::Handle::create(&asset_path("images/services/proxmox.jpg"))?,
            linux: picture::Handle::create(&asset_path("images/services/linux.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> { Some("Linux und Proxmox".into()) }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Linux und Proxmox"),
            ServiceCard::<Self>::new("Proxmox-VE", &data.proxmox).body(div![
                p!("Mit unserer Expertise übernehmen wir die Einrichtung, Wartung und Überwachung Ihrer Proxmox-Infrastruktur."),
                h3!("Hohe Zuverlässigkeit"),
                p!("Proxmox basiert auf Debian und KVM und bietet eine robuste Virtualisierungsumgebung."),
                h3!("Überlegene Datenintegrität"),
                p!("Mit ZFS und dem Proxmox Backup Server erhalten Sie hervorragenden Schutz Ihrer Daten.")
            ]),
            ServiceCard::<Self>::new("Linux-Server", &data.linux).body(div![
                p!("Sie haben Interesse am Einsatz von Linux oder suchen jemanden, der Ihre vorhandene Infrastruktur betreut? Dann sind Sie bei uns richtig."),
                h3!("Flexibilität"),
                p!("Von Datenbanken über Webserver bis zur Nextcloud: Linux-Server sind enorm flexibel."),
                h3!("Effizienz"),
                p!("Linux-Systeme können äußerst ressourcenschonend sein und bestehende Windows-Netzwerke hervorragend ergänzen.")
            ]),
            h2!("Warum Linux?"),
            div![
                h3!("Linux ist zuverlässig"),
                p!("Unternehmen aus der ganzen Welt setzen auf Linux, darunter Google, Amazon und Microsoft."),
                h3!("Linux ist sicher"),
                p!("Linux-Systeme bieten starke Rechtekonzepte, Sandbox-Mechanismen und eine geringe Angriffsfläche."),
                h3!("Linux ist unabhängig"),
                p!("Linux ist Open Source. Niemand kann Ihnen die Lizenz entziehen, und Ihre Infrastruktur bleibt souverän."),
                h2!("Noch Fragen?"),
                p!("Sind Sie sich unsicher, ob wir der richtige Ansprechpartner für Ihr Problem sind?"),
                p!("Für Ihre Fragen haben wir immer ein offenes Ohr - und wenn wir doch einmal nicht helfen können, dann kennen wir vielleicht jemanden, der es kann."),
                Phone
            ].class("plain-copy")
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}
