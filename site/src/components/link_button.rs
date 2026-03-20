use std::borrow::Cow;

use rust_pages::{
    a,
    style::Style,
    theme::Theme,
    widget::{Component, ToElement},
};

pub struct LinkButton<'a> {
    href: Cow<'a, str>,
    label: Cow<'a, str>,
}

impl<'a> LinkButton<'a> {
    pub fn new() -> Self {
        Self {
            href: Cow::Borrowed(""),
            label: Cow::Borrowed(""),
        }
    }

    pub fn href(mut self, href: impl Into<Cow<'a, str>>) -> Self {
        self.href = href.into();
        self
    }

    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = label.into();
        self
    }
}

impl Component for LinkButton<'_> {
    fn view(&self) -> impl ToElement<'_, Self> {
        a(self.label.as_ref())
            .href(self.href.as_ref())
            .class("link-button")
    }

    fn style(&self, theme: &dyn Theme) -> Vec<Style<Self>> {
        vec![
            Style::new(".link-button")
                .display("inline-block")
                .padding(".5rem 1.5rem")
                .border_radius("1rem")
                .background_color(theme.primary_color())
                .text_decoration_none()
                .font_size("1.25rem")
                .property("font-weight", "700")
                .color_inherit(),
            Style::new(".link-button:hover").background_color(theme.interactive_hover_color()),
        ]
    }
}
