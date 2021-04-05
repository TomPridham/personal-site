extern crate maud;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use std::error::Error;

mod brutemoji;
pub use brutemoji::brutemoji;

pub fn projects() -> Result<Markup, Box<dyn Error>> {
    let projects_html = html! {
        h1{"projects"}
        ul.list{
            li{
                a href="/projects/brutemoji"{"brutemoji"}
            }
        }
    };
    Ok(projects_html)
}
