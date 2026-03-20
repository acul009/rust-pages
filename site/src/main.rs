mod content;
mod index;
mod layout;
pub mod logo;
mod navbar;
mod no_cookies;
mod services;

use rust_pages::{
    builder::SiteBuilder,
    style::predone::{animated_details, remove_default_styles},
    theme::Dark,
};

use crate::{
    content::{About, Contact, Emergency, Impressum, Pricing, Remote, ServicesEmail, ServicesLinux, Team},
    index::Index,
    layout::MainLayout,
    services::Services,
};

fn main() {
    let builder = SiteBuilder::new()
        .title("Rahn-IT")
        .layout(MainLayout)
        .page(Index)
        .page(Emergency)
        .page(Services)
        .page(ServicesEmail)
        .page(ServicesLinux)
        .page(Pricing)
        .page(Remote)
        .page(About)
        .page(Team)
        .page(Contact)
        .page(Impressum)
        .theme(Dark {})
        .styles(remove_default_styles())
        .styles(animated_details());

    builder.build().unwrap();
}
