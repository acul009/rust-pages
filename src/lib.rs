pub mod builder;
pub mod layout;
pub mod page;
pub mod style;
pub mod widget;

fn html_sanitize<'a>(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
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
