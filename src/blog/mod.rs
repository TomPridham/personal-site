extern crate maud;

mod a_b_testing_with_cloudflare_workers;
mod developing_the_right_way;
mod getting_the_first_job;
mod javascript_detective_work;
mod perf_testing_js;
mod solution_engineering;
mod svg_vs_icon_font;
mod updating_a_dsl;
mod why_is_rust_so_fast;
mod windows_event_data_with_rust;
mod windows_event_data_with_rust_2;

pub use a_b_testing_with_cloudflare_workers::a_b_testing_with_cloudflare_workers;
pub use developing_the_right_way::developing_the_right_way;
pub use getting_the_first_job::getting_the_first_job;
pub use javascript_detective_work::javascript_detective_work;
use maud::{html, Markup};
pub use perf_testing_js::perf_testing_js;
pub use solution_engineering::engineering_solutions;
use std::error::Error;
pub use svg_vs_icon_font::svg_vs_icon_font;
pub use updating_a_dsl::updating_a_dsl;
pub use why_is_rust_so_fast::why_is_rust_so_fast;
pub use windows_event_data_with_rust::windows_event_data_with_rust;
pub use windows_event_data_with_rust_2::windows_event_data_with_rust_2;

pub fn blog() -> Result<Markup, Box<dyn Error>> {
    let cv_html = html! {
        h2{"Things I've written"}
        ul.list{
            li{
                a href="/blog/perf_testing_js"{"Performance testing Javascript"}
            }
            li{
                a href="/blog/updating_a_dsl"{"Updating a DSL for Consistency and Testability"}
            }
            li{
                a href="/blog/developing_the_right_way"{"Developing a Big Feature the Right Way"}
            }
            li{
                a href="/blog/engineering_solutions"{"Engineering Solutions"}
            }
            li{
                a href="/blog/javascript_detective_work"{"Javascript Detective Work 🕵️"}
            }
            li{
                a href="/blog/windows_event_data_with_rust_2"{"Accessing Windows Event Data with Rust Part 2"}
            }
            li{
                a href="/blog/windows_event_data_with_rust"{"Accessing Windows Event Data with Rust Part 1"}
            }
            li{
                a href="/blog/a_b_testing_with_cloudflare_workers"{"A/B Testing with Cloudflare Workers"}
            }
            li{
                a href="/blog/getting_the_first_job"{"Getting the First Job"}
            }
            li{
                a href="/blog/svg_vs_icon_font"{"SVGs Vs Icon Fonts"}
            }
            li{
                a href="/blog/why_is_rust_so_fast"{"Why is Rust so Fast"}
            }
        }
    };
    Ok(cv_html)
}
