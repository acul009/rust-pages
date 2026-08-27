use rust_pages::{
    a, details, div, nav, raw_html,
    style::Style,
    ul,
    widget::{Component, ToElement},
};

use crate::logo::LogoFull;

pub struct NavBar;

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
                a("Downloads").href("/downloads"),
                details(ul![
                    a("Unsere Prinzipien").href("/about-us"),
                    a("Das Team").href("/about-us/people")
                ])
                .name("nav1")
                .summary("Unternehmen"),
                a("Kontakt").href("/contact")
            ],
            div![
                details(ul![
                    a("Home").href("/"),
                    a("Notdienst").href("/emergency"),
                    details(ul![
                        a("Dienste").href("/services"),
                        a("E-Mail").href("/services/email"),
                        a("Linux & Proxmox").href("/services/linux")
                    ])
                    .name("mobile-nav")
                    .summary("Leistungen"),
                    a("Preise").href("/pricing"),
                    a("Downloads").href("/downloads"),
                    details(ul![
                        a("Unsere Prinzipien").href("/about-us"),
                        a("Das Team").href("/about-us/people")
                    ])
                    .name("mobile-nav")
                    .summary("Unternehmen"),
                    a("Kontakt").href("/contact")
                ])
                .summary(raw_html(
                    r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16"/></svg>"#
                ))
            ]
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
                .property("top", "0")
                .property("left", "0")
                .property("z-index", "1000")
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
                .margin("0")
                .flex()
                .items_center()
                .list_style_none(),
            Style::new("nav ul")
                .padding("0")
                .margin("0")
                .list_style_none(),
            Style::new("nav > ul a, nav summary")
                .cursor_pointer()
                .padding(".5rem 1rem")
                .border_radius(".25rem")
                .text_decoration_none(),
            Style::new("nav > div a:hover, nav > ul a:hover, nav summary:hover")
                .background_color(theme.interactive_hover_color()),
            Style::new("nav > ul > li").position_relative(),
            Style::new("nav > ul > li > details > ul")
                .padding(".5rem")
                .position_absolute()
                .property("left", "0")
                .property("top", "100%")
                .margin("0")
                .min_width("13rem")
                .box_sizing("border-box")
                .border_radius(".5rem"),
            Style::new("nav > ul > li > details > ul > li").width_full(),
            Style::new("nav > ul > li > details > ul a")
                .display("block")
                .width_full()
                .box_sizing("border-box")
                .property("white-space", "nowrap")
                .text_decoration_none(),
            Style::new("nav > div:last-child > details").display_none(),
            Style::media_query(
                "(max-width: 79.999rem)",
                [
                    Style::new("nav").justify_content("space-between"),
                    Style::new("nav > ul").display_none(),
                    Style::new("nav > div:first-child")
                        .flex_grow("1")
                        .flex_basis("auto")
                        .padding(".5rem"),
                    Style::new("nav > div:first-child a").padding(".2rem .25rem 0 .25rem"),
                    Style::new("nav > div:first-child svg").height("2.75rem"),
                    Style::new("nav > div:last-child")
                        .flex_grow("0")
                        .flex_basis("auto")
                        .position_relative(),
                    Style::new("nav > div:last-child > details")
                        .display("block")
                        .position_relative(),
                    Style::new("nav > div:last-child > details > summary")
                        .padding(".5rem")
                        .height("3rem")
                        .width("3rem")
                        .box_sizing("border-box")
                        .border_radius(".5rem"),
                    Style::new("nav > div:last-child > details > summary::after").display_none(),
                    Style::new("nav > div:last-child > details > summary svg")
                        .height_full()
                        .width_full(),
                    Style::new("nav > div:last-child > details > ul")
                        .position_absolute()
                        .property("right", "0")
                        .property("top", "calc(100% + .5rem)")
                        .width("20rem")
                        .max_width("calc(100vw - 2rem)")
                        .property("max-height", "calc(100vh - 6rem)")
                        .property("overflow-y", "auto")
                        .padding("1rem")
                        .box_sizing("border-box")
                        .border_radius(".75rem"),
                    Style::new("nav > div:last-child > details > ul > li")
                        .width_full()
                        .position_relative(),
                    Style::new("nav > div:last-child > details > ul a, nav > div:last-child > details > ul summary")
                        .display("block")
                        .width_full()
                        .box_sizing("border-box")
                        .padding(".65rem 1rem")
                        .text_decoration_none(),
                    Style::new("nav > div:last-child > details > ul details > summary")
                        .display("flex"),
                    Style::new("nav > div:last-child > details > ul details > ul")
                        .position("static")
                        .width_full()
                        .padding("0 0 0 1rem")
                        .box_sizing("border-box")
                        .box_shadow("none"),
                ],
            ),
        ]
    }
}
