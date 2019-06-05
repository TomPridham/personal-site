#![feature(proc_macro_hygiene)]

extern crate maud;
mod cv;
mod head;
use maud::{DOCTYPE, html};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use cv::cv;
use head::head;

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
}
