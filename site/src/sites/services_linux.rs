use std::borrow::Cow;

use rust_pages::{div, h1, h2, raw_html, page::Page, style::Style, theme::Theme, widget::picture};

use crate::{
    components::{service_card::ServiceCard, site_data::asset_path},
};

pub struct ServicesLinux;

pub struct Data {
    pub proxmox: picture::Handle,
    pub linux: picture::Handle,
}

impl Page for ServicesLinux {
    type Data = Data;

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "services/linux".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(Data {
            proxmox: picture::Handle::create(&asset_path("images/services/proxmox.jpg"))?,
            linux: picture::Handle::create(&asset_path("images/services/linux.jpg"))?,
        })
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Linux und Proxmox".into())
    }

    fn view(data: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Linux und Proxmox"),
            ServiceCard::new("Proxmox-VE", &data.proxmox, r#"<p>Mit unserer Expertise ÃƒÂ¼bernehmen wir Einrichtung, Wartung und ÃƒÅ“berwachung Ihrer Proxmox-Infrastruktur.</p><h3>Hohe ZuverlÃƒÂ¤ssigkeit</h3><p>Proxmox basiert auf Debian und KVM und bietet eine robuste Virtualisierungsumgebung.</p><h3>ÃƒÅ“berlegene DatenintegritÃƒÂ¤t</h3><p>Mit ZFS und dem Proxmox Backup Server erhalten Sie hervorragenden Schutz Ihrer Daten.</p>"#),
            ServiceCard::new("Linux-Server", &data.linux, r#"<p>Sie haben Interesse am Einsatz von Linux oder suchen jemanden, der Ihre vorhandene Infrastruktur betreut? Dann sind Sie bei uns richtig.</p><h3>FlexibilitÃƒÂ¤t</h3><p>Von Datenbanken ÃƒÂ¼ber Webserver bis zur Nextcloud: Linux-Server sind enorm flexibel.</p><h3>Effizienz</h3><p>Linux-Systeme kÃƒÂ¶nnen ÃƒÂ¤uÃƒÅ¸erst ressourcenschonend sein und bestehende Windows-Netzwerke hervorragend ergÃƒÂ¤nzen.</p>"#),
            h2!("Warum Linux?"),
            raw_html(r#"<div class="plain-copy"><h3>Linux ist zuverlÃƒÂ¤ssig</h3><p>Unternehmen aus der ganzen Welt setzen auf Linux, darunter Google, Amazon und Microsoft.</p><h3>Linux ist sicher</h3><p>Linux-Systeme bieten starke Rechtekonzepte, Sandbox-Mechanismen und eine geringe AngriffsflÃƒÂ¤che.</p><h3>Linux ist unabhÃƒÂ¤ngig</h3><p>Linux ist Open Source. Niemand kann Ihnen die Lizenz entziehen, und Ihre Infrastruktur bleibt souverÃƒÂ¤n.</p></div>"#)
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![Style::new(".plain-copy h3").padding("1.5rem 0 .25rem 0").margin("0")]
    }
}
