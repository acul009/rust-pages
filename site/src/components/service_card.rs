use rust_pages::{
    div, h2, picture,
    style::Style,
    theme::Theme,
    widget::{Component, ContextElement, ToElement, picture},
};

pub struct ServiceCard<'a, Context> {
    title: &'a str,
    image: &'a picture::Handle,
    body: Option<ContextElement<'a, Context>>,
}

impl<'a, Context> ServiceCard<'a, Context> {
    pub fn new(title: &'a str, image: &'a picture::Handle) -> ServiceCard<'a, Context> {
        Self {
            title,
            image,
            body: None,
        }
    }

    pub fn body(mut self, body: impl ToElement<'a, Context>) -> Self {
        self.body = Some(body.to_element());
        self
    }
}

impl<Context> Component for ServiceCard<'_, Context> {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![
            div![picture(self.image).class("service-image")].class("service-figure"),
            div![h2!(self.title), self.body.as_ref()].class("service-body")
        ]
        .class("service-card")
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".service-card")
                .position_relative()
                .margin("1rem 0")
                .border_radius("1.5rem")
                .property("overflow", "hidden")
                .property("min-height", "24rem")
                .box_shadow("0 1rem 3rem rgba(0,0,0,0.25)"),
            Style::new(".service-figure")
                .position_absolute()
                .property("inset", "0"),
            Style::new(".service-image, .service-image picture, .service-image img")
                .width_full()
                .height_full(),
            Style::new(".service-image img").property("object-fit", "cover"),
            Style::new(".service-body")
                .position_relative()
                .property("z-index", "1")
                .padding("2rem")
                .color("white")
                .background("linear-gradient(135deg, rgba(15,23,42,0.8), rgba(15,23,42,0.45))")
                .property("backdrop-filter", "blur(6px)")
                .property("min-height", "24rem"),
            Style::new(".service-body h2")
                .padding("0 0 1rem 0")
                .margin("0")
                .text_align_left()
                .font_size("2rem"),
            Style::new(".service-body h3")
                .padding("1rem 0 .25rem 0")
                .margin("0")
                .font_size("1.35rem"),
            Style::new(".service-body p")
                .padding(".35rem 0")
                .margin("0"),
            Style::new(".service-body a")
                .color("inherit")
                .text_decoration_underline(),
            Style::new(".service-body .callout")
                .margin("1rem 0 0 0")
                .padding("1rem 1.25rem")
                .border_radius("1rem")
                .background_color("rgba(255,255,255,0.08)"),
        ]
    }
}
