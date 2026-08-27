use rust_pages::{
    a, div, picture, span,
    style::Style,
    widget::{Component, ToElement, picture},
};

pub struct Company {
    name: &'static str,
    href: &'static str,
    background: Background,
    class: &'static str,
    image: picture::Handle,
}

#[derive(Clone, Copy)]
pub enum Background {
    White,
    Dark,
}

impl Company {
    pub fn new(
        name: &'static str,
        href: &'static str,
        background: Background,
        class: &'static str,
        image: picture::Handle,
    ) -> Self {
        Self {
            name,
            href,
            background,
            class,
            image,
        }
    }

    fn background_class(&self) -> &'static str {
        match self.background {
            Background::White => "bg-white",
            Background::Dark => "bg-dark",
        }
    }
}

pub struct CompanyReferences<'a> {
    companies: &'a [Company],
}

impl<'a> CompanyReferences<'a> {
    pub fn new(companies: &'a [Company]) -> Self {
        Self { companies }
    }
}

impl<'a> Component for CompanyReferences<'a> {
    fn view(&self) -> impl ToElement<'_, Self> {
        self.companies
            .iter()
            .fold(div!().class("company-grid"), |grid, company| {
                grid.child(
                    a(div![
                        div![
                            picture(&company.image)
                                .alt(company.name)
                                .class("company-logo")
                        ]
                        .class(company.background_class())
                        .class(company.class),
                        span!(company.name).class("company-name")
                    ]
                    .class("company-card"))
                    .href(company.href),
                )
            })
    }

    fn style(&self, theme: &dyn rust_pages::theme::Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".company-grid")
                .flex()
                .flex_wrap("wrap")
                .justify_content("center")
                .align_items("flex-start")
                .gap("2rem 1.5rem")
                .padding("1rem 0 2rem 0"),
            Style::new(".company-grid > a")
                .display("flex")
                .height("8.75rem")
                .color_inherit()
                .text_decoration_none(),
            Style::new(".company-card")
                .border_box()
                .flex()
                .flex_column()
                .align_items_center()
                .justify_content("flex-start")
                .gap(".75rem")
                .padding(".5rem .7rem")
                .border_radius("1rem")
                .height_full()
                .property("width", "max-content"),
            Style::new(".company-card:hover").background_color(theme.interactive_hover_color()),
            Style::new(".company-card > div")
                .border_box()
                .height("6rem")
                .min_width("100%")
                .border_radius(".75rem")
                .flex()
                .align_items_center()
                .justify_content("center"),
            Style::new(".company-logo").border_box().height("4rem"),
            Style::new(".company-logo img")
                .height("4rem")
                .width("auto")
                .property("object-fit", "contain"),
            Style::new(".company-name")
                .height("1.5rem")
                .text_align_center()
                .line_height("1.5rem")
                .property("white-space", "nowrap"),
            Style::new(".bg-white").background_color("white"),
            Style::new(".bg-dark").background_color("rgba(255,255,255,0.08)"),
            Style::new(".p-8").padding("2rem"),
            Style::new(".p-10").padding("2.5rem"),
            Style::new(".px-8").padding("1rem 2rem"),
        ]
    }
}
