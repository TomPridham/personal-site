#![feature(proc_macro_hygiene)]

extern crate maud;
extern crate minifier;
extern crate serde;
extern crate serde_json;

mod cv;
mod head;
use cv::cv;
use head::head;
use maud::{html, DOCTYPE};
use minifier::css::minify;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("index.html");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}", display, why.description());
        }
        Ok(file) => file,
    };
    let markup = html! {
        (DOCTYPE)
        (head())
        body {
            (cv())
        }
    };
    match file.write_all(&markup.into_string().as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => println!("success!"),
    }

    let css = match read_to_string(Path::new("src/app.css")) {
        Err(why) => panic!("couldn't read the css! tragedy:{}", why),
        Ok(c) => c,
    };
    let minified_css = match minify(css.as_str()) {
        Err(why) => panic!("frickin css {}", why),
        Ok(mc) => mc,
    };

    let mut file = match File::create(Path::new("app.css")) {
        Err(why) => {
            panic!("couldn't create {}: {}", display, why.description());
        }
        Ok(file) => file,
    };
    match file.write_all(&minified_css.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => println!("success!"),
    }
}
