use rust_pages::{
    a, div, picture, span,
    style::Style,
    widget::{Component, ToElement, picture},
};

pub struct Company {
    name: &'static str,
    href: &'static str,
    class: &'static str,
    image: picture::Handle,
}

impl Company {
    pub fn new(
        name: &'static str,
        href: &'static str,
        class: &'static str,
        image: picture::Handle,
    ) -> Self {
        Self {
            name,
            href,
            class,
            image,
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
                        div![picture(&company.image).class("company-logo")].class(company.class),
                        span!(company.name).class("company-name")
                    ]
                    .class("company-card"))
                    .href(company.href),
                )
            })
    }

    fn style(&self, _theme: &dyn rust_pages::theme::Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".company-grid")
                .flex()
                .flex_wrap("wrap")
                .justify_content("center")
                .align_items_center()
                .gap("1.5rem")
                .padding("1rem 0 2rem 0"),
            Style::new(".company-grid > a")
                .color_inherit()
                .text_decoration_none(),
            Style::new(".company-card")
                .flex()
                .flex_column()
                .align_items_center()
                .justify_content("space-between")
                .gap(".75rem")
                .width("16rem")
                .min_height("11rem")
                .padding("1rem")
                .border_radius("1rem"),
            Style::new(".company-card:hover")
                .background_color("color-mix(in oklab, white 6%, transparent)"),
            Style::new(".company-card > div")
                .width_full()
                .min_height("7rem")
                .border_radius(".75rem")
                .flex()
                .align_items_center()
                .justify_content("center"),
            Style::new(".company-logo, .company-logo img")
                .width_full()
                .height("auto"),
            Style::new(".company-logo img")
                .property("max-height", "4.5rem")
                .property("object-fit", "contain"),
            Style::new(".company-name")
                .text_align_center()
                .line_height("1.4rem"),
            Style::new(".bg-neutral").background_color("rgba(255,255,255,0.08)"),
            Style::new(".bg-neutral-50").background_color("rgba(255,255,255,0.04)"),
            Style::new(".bg-neutral-100").background_color("rgba(255,255,255,0.12)"),
            Style::new(".p-8").padding("2rem"),
            Style::new(".p-10").padding("2.5rem"),
            Style::new(".px-8").padding("1rem 2rem"),
        ]
    }
}
