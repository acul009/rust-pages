pub mod components;
mod layout;
pub mod logo;
mod services;
mod sites;

use rust_pages::{
    builder::SiteBuilder,
    style::predone::{animated_details, remove_default_styles},
    theme::Dark,
};

use crate::{
    layout::MainLayout,
    services::Services,
    sites::{
        about::About, contact::Contact, downloads::Downloads, emergency::Emergency,
        impressum::Impressum, index::Index, pricing::Pricing, redirect::Redirect,
        services_email::ServicesEmail, services_linux::ServicesLinux, team::Team,
    },
};

fn main() {
    let builder = SiteBuilder::new()
        .title("Rahn-IT")
        .base_url("https://it-rahn.de")
        .layout(MainLayout)
        .page(Index)
        .page(Emergency)
        .page(Services)
        .page(ServicesEmail)
        .page(ServicesLinux)
        .page(Pricing)
        .page(Downloads)
        .page(About)
        .page(Team)
        .page(Contact)
        .page(Impressum)
        .page(Redirect::new("remote", "/downloads"))
        .theme(Dark {})
        .styles(remove_default_styles())
        .styles(animated_details());

    builder.build().unwrap();
}
