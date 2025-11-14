use rust_pages::{
    nav,
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
        nav![ul![
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
        ]]
    }

    fn style(&self) -> Vec<rust_pages::style::Style<Self>> {
        vec![
            Style::new("nav")
                .position_fixed()
                .width_full()
                .background("oklch(0.2326 0.014 253.1)")
                .font_size("1.1rem"),
            Style::new("nav > ul")
                .width_full()
                .flex()
                .items_center()
                .list_style_none(),
            Style::new("nav > ul > li > a, nav > ul > li > details > summary")
                .padding(".5rem 1.5rem")
                .border_radius(".25rem")
                .cursor_pointer(),
            Style::new("nav > ul > li > a:hover, nav > ul > li > details > summary:hover")
                .background("oklab(0.97807 -0.00659901 -0.0282392 / 0.1)"),
        ]
    }
}
