extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub fn brutemoji() -> Result<Markup, Box<dyn Error>> {
    let brutemoji_html = html! {
        h1{"brutemoji"}
        p{
            "a picture replication program that works by randomly placing emoji on a blank canvas and comparing that to the original picture. if the new picture is closer than the old picture is, it keeps that image and repeats the process. the repo is "
                a href="https://github.com/tompridham/brutemoji"{"here"}
            "."
        }
    };
    Ok(brutemoji_html)
}
