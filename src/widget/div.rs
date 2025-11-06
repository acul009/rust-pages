use crate::widget::{Element, Widget};

pub struct Div<'a> {
    children: Vec<Element<'a>>,
}

impl<'a> Div<'a> {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl Into<Element<'a>>) -> Self {
        self.children.push(child.into());
        self
    }
}
