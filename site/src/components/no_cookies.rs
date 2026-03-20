use rust_pages::{div, raw_html, style::Style, widget::Component};

pub struct NoCookies;

impl Component for NoCookies {
    fn view(&self) -> impl rust_pages::widget::ToElement<'_, Self> {
        div![
        raw_html(
            r##"<svg xmlns="http://www.w3.org/2000/svg" class="h-40 w-auto" viewBox="0 0 64 64"><title>no-cookie</title><g><circle cx="31.806" cy="32.872" r="18.194" style="fill:#bf7e68;stroke:#4c241d;stroke-linecap:round;stroke-linejoin:round;stroke-width:2px"/><circle cx="31.806" cy="32.872" r="9.194" style="fill:#6b4f5b"/><path d="M46.958 34.666a15.15 15.15 0 0 1-2.429 6.634" style="fill:none;stroke:#fc8c29;stroke-linecap:round;stroke-linejoin:round;stroke-width:2px"/><path d="M19 33a4 4 0 0 1 8 0v7.5a4.5 4.5 0 0 0 9 0v-5.682A3.82 3.82 0 0 1 39.818 31 4.18 4.18 0 0 0 44 26.818a1.71 1.71 0 0 1 1.711-1.711H50.1A19.86 19.86 0 0 0 12.626 38H14a5 5 0 0 0 5-5" style="fill:#fff;stroke:#4c241d;stroke-linecap:round;stroke-linejoin:round;stroke-width:2px"/><path d="m32 28 2-3M15 31l2-3M29 19l2-3" style="fill:none;stroke:#f96e43;stroke-linecap:round;stroke-linejoin:round;stroke-width:2px"/><path d="m22.779 20.173 2.442 2.654M30.54 38l1.681 1.827M39.779 19.173l2.442 2.654" style="fill:none;stroke:#77a052;stroke-linecap:round;stroke-linejoin:round;stroke-width:2px"/><path d="m10.518 12.518 40.963 40.963" style="fill:none;stroke:#f53e28;stroke-linejoin:round;stroke-width:2px"/><circle cx="32" cy="32" r="29" style="fill:none;stroke:#f53e28;stroke-linecap:round;stroke-linejoin:round;stroke-width:2px"/></g></svg>"##,
        )].class("no-cookies")
    }

    fn style(&self, _theme: &dyn rust_pages::theme::Theme) -> Vec<rust_pages::style::Style<Self>> {
        vec![Style::new(".no-cookies svg").height("8rem")]
    }
}
