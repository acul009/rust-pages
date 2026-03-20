use std::borrow::Cow;

use rust_pages::{
    div, h1, page::Page, raw_html, style::Style, theme::Theme, widget::container::Container,
};

use crate::{
    components::site_data::{CITY, MAIL, NAME, PHONE, POSTAL_CODE, STREET, tel_href},
    logo::LogoFull,
};

pub struct Contact;

impl Page for Contact {
    type Data = ();

    fn path(_: &Self::Data) -> std::path::PathBuf {
        "contact".into()
    }

    fn load_data(&self) -> anyhow::Result<Self::Data> {
        Ok(())
    }

    fn title<'a>(_: &'a Self::Data) -> Option<Cow<'a, str>> {
        Some("Kontakt".into())
    }

    fn view(_: &Self::Data) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
            h1!("Kontakt"),
            div![
                div![LogoFull].class("contact-logo"),
                div![
                    Container::new("address")
                        .child(div![NAME])
                        .child(div![STREET])
                        .child(div![POSTAL_CODE])
                        .child(div![CITY])
                        .child(div![MAIL])
                        .child(tel_href())
                ].class("contact-copy"),
                raw_html(format!(r#"<div class="contact-copy"><address><div>{}</div><div>{}</div><div>{} {}</div></address><h2>Kontaktinformationen</h2><p><a href="mailto:{}">E-Mail: {}</a></p><p><a href="{}">Tel: {}</a></p><h2>GeschÃƒÂ¤ftszeiten</h2><p>Mo-Do: 09:00 - 16:30<br>Fr: 09:00 - 12:00<br><b>24 Stunden Notdienst</b></p><p class="todo">TODO: vCard-Download wieder ergÃƒÂ¤nzen.</p></div>"#, NAME, STREET, POSTAL_CODE, CITY, MAIL, MAIL, tel_href(), PHONE).leak())
            ]
            .class("contact-card")
        ]
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".contact-card")
                .flex()
                .margin("3rem 0")
                .border_radius("1.75rem")
                .background_color("rgba(255,255,255,0.06)")
                .property("overflow", "hidden"),
            Style::new(".contact-logo").width("22rem").padding("3rem"),
            Style::new(".contact-logo svg").width_full().height("auto"),
            Style::new(".contact-copy").padding("2rem"),
            Style::new(".contact-copy h2")
                .padding("1.5rem 0 .25rem 0")
                .margin("0"),
            Style::new(".contact-copy a")
                .color_inherit()
                .text_decoration_underline(),
            Style::new(".todo").property("opacity", ".75"),
        ]
    }
}
