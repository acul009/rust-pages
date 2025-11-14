use crate::style::Style;

pub trait Theme {
    fn background_color(&self) -> &'static str;
    fn text_color(&self) -> &'static str;
    fn primary_color(&self) -> &'static str;
    fn primary_text_color(&self) -> &'static str;
    fn css(&self) -> Vec<Style<()>>;
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

    fn css(&self) -> Vec<Style<()>> {
        vec![
            Style::new(":root")
                .property("--bc", "oklab(0.97807 -0.00659901 -0.0282392 / 0.1)")
                .property("--tc", "oklch(0.97807 0.029 256.847)")
                .property("--pc", "oklab(0.97807 -0.00659901 -0.0282392 / 0.1)"),
            Style::new("body")
                .background_color(self.background_color())
                .color(self.text_color()),
        ]
    }
}
