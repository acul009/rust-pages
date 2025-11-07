use crate::widget::{Class, ContextElement, Widget};
use std::{fmt::Write, marker::PhantomData};

pub struct Div<'a, Context> {
    children: Vec<ContextElement<'a, Context>>,
    class: Option<String>,
    context: PhantomData<Context>,
}

impl<'a, Context> Div<'a, Context> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            class: None,
            context: PhantomData,
        }
    }

    pub fn with_children(children: impl IntoIterator<Item = ContextElement<'a, Context>>) -> Self {
        Self {
            children: children.into_iter().collect(),
            class: None,
            context: PhantomData,
        }
    }

    pub fn child(mut self, child: impl Into<ContextElement<'a, Context>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn class(mut self, class: impl Class<Context>) -> Self {
        self.class = Some(class.resolve());
        self
    }
}

impl<Context> Widget<Context> for Div<'_, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<div")?;
        if let Some(class) = &self.class {
            write!(f, " class=\"{}\"", class)?;
        }
        write!(f, ">")?;
        for child in &self.children {
            child.html(f)?;
        }
        write!(f, "</div>")?;
        Ok(())
    }
}
