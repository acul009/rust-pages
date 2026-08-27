use base64::{Engine, engine::general_purpose::STANDARD};
use rust_pages::{
    style::Stylesheet,
    theme::Theme,
    widget::{ContextElement, ToElement, Widget},
};

pub struct BrowserContent<'a, Context> {
    content: ContextElement<'a, Context>,
}

impl<'a, Context> BrowserContent<'a, Context> {
    pub fn new(content: impl ToElement<'a, Context>) -> Self {
        Self {
            content: content.to_element(),
        }
    }
}

impl<Context> Widget<Context> for BrowserContent<'_, Context> {
    fn html(&self, output: &mut String) -> std::fmt::Result {
        let mut content = String::new();
        self.content.html(&mut content)?;
        let key: [u8; 16] = rand::random();
        let encrypted = content
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(index, byte)| byte ^ key[index % key.len()])
            .collect::<Vec<_>>();
        let encoded_content = STANDARD.encode(encrypted);
        let encoded_key = STANDARD.encode(key);

        output.push_str("<div data-content=\"");
        output.push_str(&encoded_content);
        output.push_str("\" data-content-key=\"");
        output.push_str(&encoded_key);
        output.push_str("\"></div>");
        Ok(())
    }

    fn style(&self, theme: &dyn Theme, stylesheet: &mut Stylesheet) {
        self.content.style(theme, stylesheet);
    }
}
