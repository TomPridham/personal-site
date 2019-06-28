#![feature(proc_macro_hygiene)]

extern crate maud;
extern crate minifier;
extern crate serde;
extern crate serde_json;

mod about;
mod cv;
mod head;
mod header;
mod home;
mod projects;
use about::about;
use cv::cv;
use head::head;
use header::{header_html, header_script};
use home::home;
use maud::{html, Markup, DOCTYPE};
use minifier::css::minify;
use projects::projects;
use std::error::Error;
use std::fs::{read_to_string, DirBuilder, File};
use std::io::prelude::*;
use std::path::Path;

fn generate_markup(html: Markup, path: &str) -> Markup {
    html! {
        (DOCTYPE)
        (head())
        body {
            (header_html())
            div id="container"{
                (html)
            }
            (header_script(path))
        }
    }
}

fn generate_html_files() -> Result<(), Box<dyn Error>> {
    let path = Path::new("index.html");

    let mut file = File::create(&path)?;
    let markup = generate_markup(home(), "home");
    file.write_all(&markup.into_string().as_bytes())?;

    let a_path = Path::new("about");
    DirBuilder::new().recursive(true).create(a_path)?;
    let mut file = File::create(a_path.join(&path))?;
    let markup = generate_markup(about(), a_path.to_str().unwrap());
    file.write_all(&markup.into_string().as_bytes())?;

    let cv_path = Path::new("cv");
    DirBuilder::new().recursive(true).create(cv_path)?;
    let mut file = File::create(cv_path.join(&path))?;
    let markup = generate_markup(cv()?, cv_path.to_str().unwrap());
    file.write_all(&markup.into_string().as_bytes())?;

    let p_path = Path::new("projects");
    DirBuilder::new().recursive(true).create(p_path)?;
    let mut file = File::create(p_path.join(&path))?;
    let markup = generate_markup(projects(), p_path.to_str().unwrap());
    file.write_all(&markup.into_string().as_bytes())?;
    Ok(())
}

fn generate_css_file() -> Result<(), Box<dyn Error>> {
    let css = read_to_string(Path::new("src/app.css"))?;
    let minified_css = minify(css.as_str())?;
    let mut file = File::create(Path::new("app.css"))?;
    file.write_all(&minified_css.as_bytes())?;
    Ok(())
}

fn main() {
    match generate_html_files() {
        Err(why) => panic!(
            "something went wrong generating the html files: {}",
            why.description()
        ),
        Ok(_) => println!("html files generated"),
    }
    match generate_css_file() {
        Err(why) => panic!(
            "something went wrong generating the css file: {}",
            why.description()
        ),
        Ok(_) => println!("css file generated"),
    }
}
