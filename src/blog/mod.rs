extern crate maud;

mod a_b_testing_with_cloudflare_workers;
mod getting_the_first_job;
mod svg_vs_icon_font;
mod why_is_rust_so_fast;
mod windows_event_data_with_rust;
mod windows_event_data_with_rust_2;

pub use a_b_testing_with_cloudflare_workers::a_b_testing_with_cloudflare_workers;
pub use getting_the_first_job::getting_the_first_job;
use maud::{html, Markup};
use std::error::Error;
pub use svg_vs_icon_font::svg_vs_icon_font;
pub use why_is_rust_so_fast::why_is_rust_so_fast;
pub use windows_event_data_with_rust::windows_event_data_with_rust;
pub use windows_event_data_with_rust_2::windows_event_data_with_rust_2;

pub fn blog() -> Result<Markup, Box<dyn Error>> {
    let cv_html = html! {
        h2{"things i've written"}
        ul.list{
            li{
                a href="/blog/windows_event_data_with_rust_2"{"accessing windows event data with rust part 2"}
            }
            li{
                a href="/blog/windows_event_data_with_rust"{"accessing windows event data with rust part 1"}
            }
            li{
                a href="/blog/a_b_testing_with_cloudflare_workers"{"a/b testing with cloudflare workers"}
            }
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
