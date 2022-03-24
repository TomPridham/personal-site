extern crate maud;
use maud::{html, Markup};

pub fn head() -> Markup {
    html! {
        head{
            meta charset="utf8";
            meta lang="en-US";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta name="author" content="Tom Pridham";
            meta name="description" content="Portfolio site for Tom Pridham. I likes being good at things.";
            title { "tompridham.me"}
            link rel="shortcut icon" href="/assets/favicon.ico" type="image/x-icon";
            link rel="stylesheet" href="/app.css";
        }
    }
}
