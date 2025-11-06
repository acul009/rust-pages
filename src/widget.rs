mod div;

pub trait Page {
    type Data;

    fn load_data(&self) -> anyhow::Result<Self::Data>;
    fn render(data: &Self::Data) -> impl Widget;
}

pub trait Widget {
    fn to_html(&self) -> String;
}

pub struct Element<'a> {
    widget: Box<dyn Widget + 'a>,
}

impl<'a> Element<'a> {
    pub fn new(widget: impl Widget + 'a) -> Self {
        Element {
            widget: Box::new(widget),
        }
    }
}

impl<'a, W: Widget + 'a> From<W> for Element<'a> {
    fn from(widget: W) -> Self {
        Element::new(widget)
    }
}
