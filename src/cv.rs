extern crate maud;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct CVItem {
    company: String,
    duties: Vec<String>,
    notable_achievements: Vec<String>,
    technologies: Vec<String>,
    time: String,
    title: String,
}

fn cv_to_html(cv: Vec<CVItem>) -> Markup {
    html! {
        @for cv_item in  cv{
            p{(cv_item.company ) " - " (cv_item.time)}
            p{(cv_item.title)}
            p{"duties"}
            ul{
                @for duty in cv_item.duties{
                    li{(duty)}
                }
            }
            p{"notable achievements"}
            ul{
                @for n in cv_item.notable_achievements{
                    li{(n)}
                }
            }
            ul{
                @for t in cv_item.technologies{
                    li{(t)}
                }
            }
        }
    }
}

pub fn cv() -> Markup {
    let name = "tom pridham";
    let cv_file = match read_to_string(Path::new("src/cv.json")) {
        Err(why) => {
            panic!("somethings junked: {}", why.description());
        }
        Ok(file) => file,
    };
    let cv_items: Vec<CVItem> = match serde_json::from_str(cv_file.as_str()) {
        Err(why) => panic!("what the what: {}", why.description()),
        Ok(r) => r,
    };

    html! {
        div id="container"{
            p { "hi, i'm " (name) "!" }
            p { "nice to meet you." }
            br;
            (cv_to_html(cv_items))
        }
    }
}
