extern crate maud;
use maud::{html, Markup};
use std::error::Error;

mod brutemoji;
mod random_task;
mod wasmsweeper;
pub use brutemoji::brutemoji;
pub use random_task::random_task;
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
            li{
                a href="/projects/random_task"{"Random Task"}
            }
        }
    };
    Ok(projects_html)
}
