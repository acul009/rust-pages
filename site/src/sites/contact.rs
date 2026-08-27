use rust_pages::{
    a, br, div, h1, h2, p,
    page::Page,
    style::Style,
    theme::Theme,
    widget::{ToElement, container::Container},
};

use crate::{
    components::{
        browser_content::BrowserContent,
        link_button::LinkButton,
        site_data::{CITY, MAIL, NAME, PHONE, POSTAL_CODE, STREET, tel_href},
    },
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

    fn settings(_: &Self::Data, settings: &mut rust_pages::page::PageSettings) {
        settings
            .title("Kontakt")
            .description("Kontaktieren Sie uns per E-Mail oder Telefon.");
    }

    fn view(_: &Self::Data) -> impl ToElement<'_, Self> {
        div![
            h1!("Kontakt"),
            div![
                div![LogoFull].class("contact-logo"),
                div![
                    Container::new("address")
                        .child(div![NAME])
                        .child(div![STREET])
                        .child(div![format!("{} {}", POSTAL_CODE, CITY)]),
                    h2!("Kontaktinformationen"),
                    p![BrowserContent::new(
                        a(format!("E-Mail: {}", MAIL)).href(format!("mailto:{}", MAIL))
                    )],
                    p![BrowserContent::new(
                        a(format!("Tel: {}", PHONE)).href(tel_href())
                    )],
                    h2!("Geschäftszeiten"),
                    p![
                        "Mo-Do: 09:00 - 16:30",
                        br(),
                        "Fr: 09:00 - 12:00",
                        br(),
                        Container::new("b").child("24 Stunden Notdienst")
                    ],
                    div![BrowserContent::new(
                        LinkButton::new()
                            .label("Zu Kontakten hinzufügen")
                            .href("/Rahn-IT.vcf")
                            .download("Rahn-IT.vcf")
                    )]
                    .class("contact-actions")
                ]
                .class("contact-copy")
            ]
            .class("contact-card")
        ]
    }

    fn style(_theme: &dyn Theme) -> Vec<Style<Self>> {
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
            Style::new(".contact-copy address")
                .property("font-style", "normal"),
            Style::new(".contact-copy h2")
                .padding("1.5rem 0 .25rem 0")
                .margin("0"),
            Style::new(
                ".contact-copy a[href^=\"mailto:\"], .contact-copy a[href^=\"tel:\"]",
            )
            .color_inherit()
            .text_decoration_underline(),
            Style::new(".contact-actions").margin("2rem 0 0 0"),
        ]
    }
}
