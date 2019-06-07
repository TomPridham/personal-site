extern crate maud;
use maud::{html, Markup};

pub fn head() -> Markup {
    html! {
        head{
            meta charset="utf8";
            meta lang="en-US";
            meta name="author" content="tom pridham";
            meta name="description" content="portfolio site for tom pridham. he likes being good at things.";
            title { "tompridham.me "}
            link rel="shortcut icon" href="favicon.ico" type="image/x-icon";
            link rel="stylesheet" href="app.css";
        }
    }
}
