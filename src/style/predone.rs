use crate::style::Style;

pub fn remove_default_styles() -> Vec<Style<()>> {
    vec![
        Style::new("body").margin("0"),
        Style::new("html").property("font-family", "sans-serif"),
        Style::new("a").text_decoration_none().color_inherit(),
    ]
}

pub fn animated_details() -> Vec<Style<()>> {
    vec![
        Style::new("details > summary")
            .flex()
            .items_center()
            .gap(".5rem"),
        Style::new("details > summary::-webkit-details-marker").display_none(),
        Style::new("details > summary::marker").content(""),
        Style::new("details > summary::after")
            .content("")
            .transform_origin("50%")
            .pointer_events_none()
            .justify_self("flex-end")
            .width(".375rem")
            .height(".375rem")
            .transition_property("rotate,translate")
            .transition_duration(".2s")
            .block()
            .translate("0 -1px")
            .rotate("-135deg")
            .box_shadow("inset 2px 2px"),
        Style::new("details[open] > summary::after")
            .translate("0 1px")
            .rotate("45deg"),
    ]
}
