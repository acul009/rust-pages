use rust_pages::{
    a, div, p,
    style::Style,
    theme::Theme,
    widget::{Component, ToElement},
};

use crate::{
    components::{phone::Phone, site_data::NAME},
    logo::LogoStandalone,
};

pub struct Footer;

impl Component for Footer {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![
            div![
                div![
                    a(LogoStandalone).href("/").class("footer-logo"),
                    p!(NAME),
                    Phone
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
                    a("Downloads").href("/downloads"),
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
                .margin("2rem 0 0 0")
                .padding("2.5rem 2rem")
                .background_color(theme.primary_color())
                .border_box(),
            Style::new(".footer-inner")
                .flex()
                .flex_column()
                .justify_content("flex-start")
                .align_items("flex-start")
                .gap("2rem")
                .width_full()
                .max_width("80rem")
                .margin("0 auto")
                .font_size("1.125rem"),
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
            Style::new(".footer-group a")
                .color_inherit()
                .text_decoration_none(),
            Style::new(".footer-group a:hover").text_decoration_underline(),
            Style::new(".footer-group p").margin("0"),
            Style::new(".footer-brand p, .footer-brand a").line_height("1.05"),
            Style::new(".footer-logo")
                .height("3.5rem")
                .padding(".25rem")
                .border_radius(".5rem")
                .block(),
            Style::new(".footer-logo:hover").background_color(theme.interactive_hover_color()),
            Style::new(".footer-logo svg").width("auto").height_full(),
            Style::media_query(
                "(min-width: 64rem)",
                [Style::new(".footer-inner")
                    .flex_row()
                    .flex_wrap("nowrap")
                    .gap("2rem 12rem")
                    .font_size("1rem")],
            ),
        ]
    }
}
