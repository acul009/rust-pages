use std::{collections::HashMap, marker::PhantomData};

use itertools::Itertools;

use crate::html_sanitize;

pub fn remove_default_styles() -> Vec<Style<()>> {
    vec![
        Style::new("body").margin("0"),
        Style::new("html").property("font-family", "sans-serif"),
        Style::new("a").text_decoration_none().color_inherit(),
    ]
}

fn scope_name<Context>() -> String {
    let mut scope = std::any::type_name::<Context>().replace(':', "_");
    scope.push_str("__");
    scope
}

#[derive(Debug)]
pub struct Stylesheet {
    styles: HashMap<std::any::TypeId, String>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet {
            styles: HashMap::new(),
        }
    }

    pub fn contains<Context: 'static>(&self) -> bool {
        self.styles.contains_key(&std::any::TypeId::of::<Context>())
    }

    pub fn add_styles<Context: 'static>(&mut self, styles: &[Style<Context>]) {
        let type_id = std::any::TypeId::of::<Context>();
        let stylesheet = styles.iter().map(|style| style.to_stylesheet()).join(" ");

        self.styles.entry(type_id).or_insert_with(|| stylesheet);
    }

    pub fn to_css(&self) -> String {
        self.styles.values().join(" ")
    }
}

pub trait Class<Context> {
    // basically just appends the type id to the class name
    fn resolve(&self) -> String;
}

impl<Context> Class<Context> for &str {
    fn resolve(&self) -> String {
        let scope_name = scope_name::<Context>();
        self.split_ascii_whitespace()
            .map(|class| format!("{}-{}", scope_name, html_sanitize(class)))
            .join(" ")
    }
}

pub struct Style<Context: ?Sized> {
    selector: &'static str,
    properties: Vec<(&'static str, &'static str)>,
    context: PhantomData<Context>,
}

impl<Context> Style<Context> {
    pub fn new(selector: &'static str) -> Self {
        Style {
            selector,
            properties: Vec::new(),
            context: PhantomData,
        }
    }

    pub fn property(mut self, name: &'static str, value: &'static str) -> Self {
        self.properties.push((name, value));
        self
    }

    pub fn to_stylesheet(&self) -> String {
        let selector = self.selector.replace(".", scope_name::<Context>().as_str());
        format!(
            "{}{{{}}}",
            selector,
            self.properties
                .iter()
                .map(|(name, value)| format!("{}:{}", name, value))
                .join(";")
        )
    }
}

impl<Context> Style<Context> {
    pub fn align_items(self, value: &'static str) -> Self {
        self.property("align-items", value)
    }

    pub fn items_center(self) -> Self {
        self.align_items("center").justify_items("center")
    }
}

impl<Context> Style<Context> {
    pub fn background(self, value: &'static str) -> Self {
        self.property("background", value)
    }
}

impl<Context> Style<Context> {
    pub fn border(self, value: &'static str) -> Self {
        self.property("border", value)
    }

    pub fn border_radius(self, value: &'static str) -> Self {
        self.property("border-radius", value)
    }
}

impl<Context> Style<Context> {
    pub fn box_shadow(self, value: &'static str) -> Self {
        self.property("box-shadow", value)
    }
}

impl<Context> Style<Context> {
    pub fn color(self, value: &'static str) -> Self {
        self.property("color", value)
    }

    pub fn color_inherit(self) -> Self {
        self.color("inherit")
    }
}

impl<Context> Style<Context> {
    pub fn content(self, value: &'static str) -> Self {
        self.property(
            "content",
            Box::new(format!(
                "\"{}\"",
                value.replace("\\", "\\\\").replace("\"", "\\\"")
            ))
            .leak(),
        )
    }
}

impl<Context> Style<Context> {
    pub fn cursor(mut self, value: &'static str) -> Self {
        self.properties.push(("cursor", value));
        self
    }

    pub fn cursor_pointer(self) -> Self {
        self.cursor("pointer")
    }
}

impl<Context> Style<Context> {
    pub fn display(self, value: &'static str) -> Self {
        self.property("display", value)
    }

    pub fn display_none(self) -> Self {
        self.display("none")
    }

    pub fn flex(self) -> Self {
        self.display("flex")
    }

    pub fn block(self) -> Self {
        self.display("block")
    }

    pub fn grid(self) -> Self {
        self.display("grid")
    }
}

impl<Context> Style<Context> {
    pub fn font_size(self, value: &'static str) -> Self {
        self.property("font-size", value)
    }
}

impl<Context> Style<Context> {
    pub fn gap(self, value: &'static str) -> Self {
        self.property("gap", value)
    }
}

impl<Context> Style<Context> {
    pub fn height(self, value: &'static str) -> Self {
        self.property("height", value)
    }

    pub fn height_full(self) -> Self {
        self.height("100%")
    }
}

impl<Context> Style<Context> {
    pub fn justify_items(self, value: &'static str) -> Self {
        self.property("justify-items", value)
    }

    pub fn justify_self(self, value: &'static str) -> Self {
        self.property("justify-self", value)
    }
}

impl<Context> Style<Context> {
    pub fn list_style(self, value: &'static str) -> Self {
        self.property("list-style", value)
    }

    pub fn list_style_none(self) -> Self {
        self.list_style("none")
    }
}

impl<Context> Style<Context> {
    pub fn margin(self, value: &'static str) -> Self {
        self.property("margin", value)
    }
}

impl<Context> Style<Context> {
    pub fn padding(self, value: &'static str) -> Self {
        self.property("padding", value)
    }
}

impl<Context> Style<Context> {
    pub fn pointer_events(self, value: &'static str) -> Self {
        self.property("pointer-events", value)
    }

    pub fn pointer_events_none(self) -> Self {
        self.pointer_events("none")
    }
}

impl<Context> Style<Context> {
    pub fn position(mut self, value: &'static str) -> Self {
        self.properties.push(("position", value));
        self
    }

    pub fn position_fixed(mut self) -> Self {
        self.properties.push(("position", "fixed"));
        self
    }
}

impl<Context> Style<Context> {
    pub fn rotate(self, value: &'static str) -> Self {
        self.property("rotate", value)
    }
}

impl<Context> Style<Context> {
    pub fn text_decoration(mut self, value: &'static str) -> Self {
        self.properties.push(("text-decoration", value));
        self
    }

    pub fn text_decoration_underline(self) -> Self {
        self.text_decoration("underline")
    }

    pub fn text_decoration_none(self) -> Self {
        self.text_decoration("none")
    }
}

impl<Context> Style<Context> {
    pub fn transform_origin(self, value: &'static str) -> Self {
        self.property("transform-origin", value)
    }
}

impl<Context> Style<Context> {
    pub fn transition_property(self, value: &'static str) -> Self {
        self.property("transition-property", value)
    }

    pub fn transition_duration(self, value: &'static str) -> Self {
        self.property("transition-duration", value)
    }
}

impl<Context> Style<Context> {
    pub fn translate(self, value: &'static str) -> Self {
        self.property("translate", value)
    }
}

impl<Context> Style<Context> {
    pub fn width(self, value: &'static str) -> Self {
        self.property("width", value)
    }

    pub fn width_full(self) -> Self {
        self.width("100%")
    }
}
