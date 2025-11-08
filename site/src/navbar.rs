use rust_pages::{
    div, nav,
    style::Style,
    ul,
    widget::{Component, ToElement, a, details},
};

pub struct NavBar {}

impl NavBar {
    pub fn new() -> Self {
        NavBar {}
    }
}

impl Component for NavBar {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![nav!(ul![
            a("Notdienst").href("/emergency"),
            details(ul![
                a("Dienste").href("/services"),
                a("E-Mail").href("/services/email"),
                a("Linux & Proxmox").href("/services/linux")
            ])
            .summary("Leistungen"),
            a("Preise").href("/pricing"),
            a("Fernwartung").href("/remote"),
            details(ul![
                a("Unsere Prinzipien").href("/about_us"),
                a("Das Team").href("/about_us/people")
            ])
            .summary("Unternehmen"),
            a("Kontakt").href("/contact")
        ])]
        .class("test2")
    }

    fn style(&self) -> Vec<rust_pages::style::Style<Self>> {
        vec![Style::new("nav").property("width", "100%")]
    }
}
