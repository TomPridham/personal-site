#![feature(proc_macro_hygiene)]

extern crate maud;
use maud::html;
use std::error::Error;
use std::fs::File;
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
    let name = "tom pridham";
    let markup = html! {
        p { "hi, i'm " (name) "!" }
        p { "nice to meet you." }
    };
    match file.write_all(&markup.into_string().as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => println!("success!"),
    }
}
