extern crate maud;

mod getting_the_first_job;
mod svg_vs_icon_font;
mod why_is_rust_so_fast;

pub use getting_the_first_job::getting_the_first_job;
use maud::{html, Markup};
use std::error::Error;
pub use svg_vs_icon_font::svg_vs_icon_font;
pub use why_is_rust_so_fast::why_is_rust_so_fast;

pub fn blog() -> Result<Markup, Box<dyn Error>> {
    let cv_html = html! {
        h2{"things i've written"}
        ul.list{
            li{
                a href="/blog/getting_the_first_job"{"getting the first job"}
            }
            li{
                a href="/blog/svg_vs_icon_font"{"svgs vs icon font"}
            }
            li{
                a href="/blog/why_is_rust_so_fast"{"why is rust so fast"}
            }
        }
    };
    Ok(cv_html)
}
