use crate::widget::Widget;

pub mod page;
pub mod widget;

fn html_sanitize(input: &str) -> String {
    let mut sanitized = String::with_capacity(input.len());

    for char in input.chars() {
        let replacement = match char {
            '&' => "&amp;",
            '>' => "&gt;",
            '<' => "&lt;",
            '"' => "&quot;",
            '\'' => "&#x27;",
            '\\' => "&#x2F;",
            char => {
                sanitized.push(char);
                continue;
            }
        };

        sanitized.push_str(replacement);
    }

    sanitized
}

pub fn render(widget: impl Widget) -> String {
    let mut buffer = String::new();
    widget.html(&mut buffer).unwrap();
    buffer
}
