use rust_pages::{
    a, details, div, nav,
    style::Style,
    ul,
    widget::{Component, ToElement, a, details},
};

use crate::logo::LogoFull;

pub struct NavBar {}

impl NavBar {
    pub fn new() -> Self {
        NavBar {}
    }
}

impl Component for NavBar {
    fn view(&self) -> impl ToElement<'_, Self> {
        nav![
            div![a(LogoFull).href("/")],
            ul![
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
            ],
            div![]
        ]
    }

    fn style(&self, theme: &dyn rust_pages::theme::Theme) -> Vec<rust_pages::style::Style<Self>> {
        vec![
            Style::new("nav,nav details > *:not(summary)")
                .background(theme.primary_color())
                .box_shadow("0 .5rem 2rem rgba(0,0,0,0.5)"),
            Style::new("nav")
                .height("5rem")
                .flex()
                .justify_content("center")
                .align_items("center")
                .position_fixed()
                .width_full()
                .font_size("1.1rem"),
            Style::new("nav > div")
                .flex_grow("1")
                .flex_basis("0")
                .padding(".5rem 1rem"),
            Style::new("nav > div a")
                .padding(".2rem 1rem 0 1rem")
                .height("fit-content")
                .width("fit-content")
                .border_radius(".5rem")
                .block(),
            Style::new("nav svg").height("3.75rem").width("auto"),
            Style::new("nav > ul")
                .padding("0")
                .flex()
                .items_center()
                .list_style_none(),
            Style::new("nav > ul a, nav summary")
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
