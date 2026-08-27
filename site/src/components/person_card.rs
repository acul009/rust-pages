use rust_pages::{
    div, h2, picture,
    style::Style,
    theme::Theme,
    widget::{Component, ToElement, Widget, container::Container, picture as picture_handle},
};

struct StoredWidget<'a, Context> {
    inner: Box<dyn Widget<Context> + 'a>,
}

impl<Context> Widget<Context> for StoredWidget<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        self.inner.html(f)
    }

    fn style(&self, theme: &dyn rust_pages::theme::Theme, stylesheet: &mut rust_pages::style::Stylesheet) {
        self.inner.style(theme, stylesheet);
    }
}

impl<Context> Widget<Context> for &StoredWidget<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        self.inner.html(f)
    }

    fn style(&self, theme: &dyn rust_pages::theme::Theme, stylesheet: &mut rust_pages::style::Stylesheet) {
        self.inner.style(theme, stylesheet);
    }
}

pub struct PersonCard<'a> {
    image: &'a picture_handle::Handle,
    name: &'a str,
    caption: &'a str,
    body: StoredWidget<'a, Self>,
}

impl<'a> PersonCard<'a> {
    pub fn new(
        image: &'a picture_handle::Handle,
        name: &'a str,
        caption: &'a str,
        body: impl Widget<Self> + 'a,
    ) -> Self {
        Self {
            image,
            name,
            caption,
            body: StoredWidget { inner: Box::new(body) },
        }
    }
}

impl Component for PersonCard<'_> {
    fn view(&self) -> impl ToElement<'_, Self> {
        div![
            div![
                picture(self.image)
                    .alt(format!("Bild von {}", self.name))
                    .class("person-image")
            ]
            .class("person-figure"),
            div![
                h2!(self.name),
                div![Container::new("i").child(self.caption)].class("caption"),
                StoredWidget {
                    inner: Box::new(&self.body)
                }
            ]
            .class("person-body")
        ]
        .class("person-card")
    }

    fn style(&self, _theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".person-card")
                .flex()
                .gap("0")
                .margin("2.5rem 0")
                .border_radius("1.75rem")
                .background_color("rgba(255,255,255,0.06)")
                .property("overflow", "hidden")
                .box_shadow("0 1rem 3rem rgba(0,0,0,0.2)"),
            Style::new(".person-figure").width("22rem").flex_basis("22rem"),
            Style::new(".person-image, .person-image picture, .person-image img")
                .width_full()
                .height_full(),
            Style::new(".person-image img").property("object-fit", "cover"),
            Style::new(".person-body").padding("2rem"),
            Style::new(".person-body h2")
                .padding("0")
                .margin("0")
                .text_align_left(),
            Style::new(".person-body .caption").padding(".5rem 0 1rem 0").margin("0"),
            Style::new(".person-body p").padding(".35rem 0").margin("0"),
        ]
    }
}
