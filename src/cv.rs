extern crate maud;
use maud::{Markup,html};

pub fn cv() -> Markup{
    let name = "tom pridham";
    html! {
        div id="container"{
            p { "hi, i'm " (name) "!" }
            p { "nice to meet you." }
        }
    }
}
