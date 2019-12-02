extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub mod svg_vs_icon_font;

pub fn blog() -> Result<Markup, Box<dyn Error>> {
    let cv_html = html! {
        h2{"things i've written"}
        div.col{
            a href="/why_is_rust_so_fast.html"{"why is rust so fast"}
            a href="/blog/svg_vs_icon_font"{"svgs vs icon font"}
        }
    };
    Ok(cv_html)
}
