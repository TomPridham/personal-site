#![feature(proc_macro_hygiene)]

extern crate maud;
use maud::html;

pub fn cv() {
    let name = "tom pridham";
    let markup = html! {
        p { "hi, i'm " (name) "!" }
        p { "nice to meet you." }
    };
}
