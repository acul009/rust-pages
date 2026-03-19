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

use crate::{index::Index, layout::MainLayout};

fn main() {
    let builder = SiteBuilder::new()
        .title("Rahn-IT")
        .layout(MainLayout)
        .page(Index)
        .theme(Dark {})
        .styles(remove_default_styles())
        .styles(animated_details());

    builder.build().unwrap();
}
