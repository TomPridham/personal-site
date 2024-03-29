extern crate maud;
use maud::{html, Markup};
use std::error::Error;

mod brutemoji;
mod wasmsweeper;
pub use brutemoji::brutemoji;
pub use wasmsweeper::wasmsweeper;

pub fn projects() -> Result<Markup, Box<dyn Error>> {
    let projects_html = html! {
        h1{"Projects"}
        ul.list{
            li{
                a href="/projects/wasmsweeper"{"Wasmsweeper"}
            }
            li{
                a href="/projects/brutemoji"{"Brutemoji"}
            }
        }
    };
    Ok(projects_html)
}
