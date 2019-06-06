extern crate maud;
use maud::{Markup,html};
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug,Serialize, Deserialize)]
struct CVItem{
    company: String,
    duties: Vec<String>,
    notable_achievements: Vec<String>,
    technology: Vec<String>,
    time: String,
    title:String
}

pub fn cv() -> Markup{
    let name = "tom pridham";
    let cv_file = match read_to_string(Path::new("src/cv.json")){
        Err(why)=>{
            panic!("somethings junked: {}", why.description());
        },
        Ok(file) => file,
    };
    let cv_items: Vec<CVItem> =match serde_json::from_str(cv_file.as_str()){
        Err(why)=>{
            panic!("what the what: {}", why.description())
        },
        Ok(r) => r
    };
    println!("{:?}", cv_items);

    html! {
        div id="container"{
            p { "hi, i'm " (name) "!" }
            p { "nice to meet you." }
            br;
            ul {

            }
        }
    }
}
