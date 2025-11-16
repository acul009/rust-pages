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
            .name("nav1")
            .summary("Leistungen"),
            a("Preise").href("/pricing"),
            a("Fernwartung").href("/remote"),
            details(ul![
                a("Unsere Prinzipien").href("/about_us"),
                a("Das Team").href("/about_us/people")
            ])
            .name("nav1")
            .summary("Unternehmen"),
            a("Kontakt").href("/contact")
        ]]
    }

    fn style(&self, theme: &dyn rust_pages::theme::Theme) -> Vec<rust_pages::style::Style<Self>> {
        vec![
            Style::new("nav,nav details > *").background(theme.primary_color()),
            Style::new("nav")
                .position_fixed()
                .width_full()
                .font_size("1.1rem"),
            Style::new("nav > ul")
                .width_full()
                .flex()
                .items_center()
                .list_style_none(),
            Style::new("nav a").block(),
            Style::new("nav a, nav summary")
                .cursor_pointer()
                .padding(".5rem 1rem")
                .border_radius(".25rem"),
            Style::new("nav a:hover, nav summary:hover")
                .background_color(theme.primary_active_color()),
            Style::new("nav li").position_relative(),
            Style::new("nav li::marker").content(""),
            Style::new("nav li > details > ul")
                .padding(".5rem")
                .position_absolute()
                .margin("2rem 0 0 0")
                .border_radius(".5rem"),
        ]
    }
}
