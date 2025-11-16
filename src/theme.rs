use crate::style::Style;

pub struct ThemeStyle {}

pub trait Theme {
    fn background_color(&self) -> &'static str;
    fn text_color(&self) -> &'static str;
    fn primary_color(&self) -> &'static str;
    fn primary_text_color(&self) -> &'static str;
    fn primary_active_color(&self) -> &'static str;
    fn css(&self) -> Vec<Style<ThemeStyle>>;
}

pub struct Dark {}

impl Theme for Dark {
    fn background_color(&self) -> &'static str {
        "var(--bc)"
    }

    fn text_color(&self) -> &'static str {
        "var(--tc)"
    }

    fn primary_color(&self) -> &'static str {
        "var(--pc)"
    }

    fn primary_text_color(&self) -> &'static str {
        "var(--tc)"
    }

    fn primary_active_color(&self) -> &'static str {
        "color-mix(in oklab,var(--tc)10%,transparent)"
    }

    fn css(&self) -> Vec<Style<ThemeStyle>> {
        vec![
            Style::new(":root")
                .property("--bc", "oklch(0.2533 0.016 252.42)")
                .property("--tc", "oklch(0.97807 0.029 256.847)")
                .property("--pc", "oklch(0.2326 0.014 253.1)"),
            Style::new("html")
                .background_color(self.background_color())
                .color(self.text_color()),
        ]
    }
}
