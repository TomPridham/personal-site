extern crate maud;

mod svg_vs_icon_font;
mod why_is_rust_so_fast;

use maud::{html, Markup};
use std::error::Error;
pub use svg_vs_icon_font::svg_vs_icon_font;
pub use why_is_rust_so_fast::why_is_rust_so_fast;

pub fn blog() -> Result<Markup, Box<dyn Error>> {
    let cv_html = html! {
        h2{"things i've written"}
        div.col{
            a href="/blog/why_is_rust_so_fast"{"why is rust so fast"}
            a href="/blog/svg_vs_icon_font"{"svgs vs icon font"}
        }
    };
    Ok(cv_html)
}
