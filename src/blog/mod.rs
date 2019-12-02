extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub mod svg_vs_icon_font;

pub fn blog() -> Result<Markup, Box<dyn Error>> {
    let cv_html = html! {
        div.row{
            h1{"tom pridham"}
            h2{"contact"}
            ul{
                li{"email: pridham.tom@gmail.com"}
                li{"phone: 702-285-7906"}
                li{"website: tompridham.github.io"}
                li{"github: github.com/tompridham"}
            }
        }
        div.row{
            h2{"skills"}
            ul{
                li{"react"}
                li{"graphql"}
                li{"rust"}
                li{"typescript"}
                li{"angularjs"}
                li{"node"}
                li{"postgres"}
            }
        }
    };
    Ok(cv_html)
}
