use std::marker::PhantomData;

use itertools::Itertools;

use crate::html_sanitize;

fn scope_name<Context>() -> String {
    std::any::type_name::<Context>().replace(':', "_")
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

pub struct Style<Context> {
    selector: String,
    properties: Vec<(String, String)>,
    context: PhantomData<Context>,
}

impl<Context> Style<Context> {
    pub fn new(selector: String) -> Self {
        Style {
            selector,
            properties: Vec::new(),
            context: PhantomData,
        }
    }

    pub fn property(mut self, name: String, value: String) -> Self {
        self.properties.push((name, value));
        self
    }

    pub fn to_stylesheet(&self) -> String {
        let selector = self.selector.replace(".", scope_name::<Context>().as_str());
        format!(
            "{} {{\n{}\n}}",
            selector,
            self.properties
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value))
                .join(";")
        )
    }
}
