use crate::widget::Widget;

pub struct RawHtml<'a> {
    html: &'a str,
}

impl<'a> RawHtml<'a> {
    pub fn new(html: &'a str) -> Self {
        Self { html }
    }
}

impl<'a, Context> Widget<Context> for RawHtml<'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        f.push_str(self.html);
        Ok(())
    }

    fn style(&self, _stylesheet: &mut crate::style::Stylesheet) {}
}
