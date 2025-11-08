use std::{collections::HashMap, marker::PhantomData};

use itertools::Itertools;

use crate::html_sanitize;

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
            "{} {{\n{}\n}}",
            selector,
            self.properties
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value))
                .join(";")
        )
    }
}
