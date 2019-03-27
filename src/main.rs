#![feature(proc_macro_hygiene)]

extern crate maud;
use maud::html;

fn main() {
    let name = "Tom Pridham";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string())
}
