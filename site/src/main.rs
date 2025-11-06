use std::{fs::File, io::Write};

use rust_pages::{
    div, nav, render, ul,
    widget::{Element, a, details::details},
};

fn main() {
    let html = render(div![navbar(), "Hello 2"]);

    let mut file = File::create("output.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
    println!("{}", html);
}

fn navbar() -> Element<'static> {
    nav!(ul![
        a("Notdienst").href("/emergency"),
        details(ul![
            a("Dienste").href("/services"),
            a("E-Mail").href("/services/email"),
            a("Linux & Proxmox").href("/services/linux")
        ])
        .summary("Leistungen"),
        a("Preise").href("/pricing"),
        a("Fernwartung").href("/remote"),
        details(ul![
            a("Unsere Prinzipien").href("/about_us"),
            a("Das Team").href("/about_us/people")
        ])
        .summary("Unternehmen"),
        a("Kontakt").href("/contact")
    ])
    .into()
}
