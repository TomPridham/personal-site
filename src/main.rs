#![feature(proc_macro_hygiene)]

extern crate maud;
extern crate minifier;
extern crate serde;
extern crate serde_json;

mod about;
mod blog;
mod cv;
mod head;
mod header;
mod home;
mod projects;

use about::about;
use blog::{
    a_b_testing_with_cloudflare_workers, blog, getting_the_first_job, svg_vs_icon_font,
    why_is_rust_so_fast, windows_event_data_with_rust, windows_event_data_with_rust_2,
};
use cv::cv;
use fs_extra::dir;
use head::head;
use header::{header_html, header_script};
use home::home;
use maud::{html, Markup, DOCTYPE};
use minifier::css::minify;
use projects::{brutemoji, projects, wasmsweeper};
use std::error::Error;
use std::fs::{read_to_string, DirBuilder, File};
use std::io::prelude::*;
use std::path::Path;

type HtmlResult = Result<maud::PreEscaped<String>, Box<dyn Error>>;
type HtmlFunctionAndPath = (fn() -> HtmlResult, &'static str);

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
    let dist = Path::new("dist");
    let path = Path::new("index.html");
    DirBuilder::new().recursive(true).create(dist)?;

    let mut file = File::create(&dist.join(path))?;
    let markup = generate_markup(home(), "home");
    file.write_all(&markup.into_string().as_bytes())?;

    let html_files: Vec<HtmlFunctionAndPath> = vec![
        (about, "about"),
        (blog, "blog"),
        (
            a_b_testing_with_cloudflare_workers,
            "blog/a_b_testing_with_cloudflare_workers",
        ),
        (getting_the_first_job, "blog/getting_the_first_job"),
        (
            windows_event_data_with_rust,
            "blog/windows_event_data_with_rust",
        ),
        (
            windows_event_data_with_rust_2,
            "blog/windows_event_data_with_rust_2",
        ),
        (svg_vs_icon_font, "blog/svg_vs_icon_font"),
        (why_is_rust_so_fast, "blog/why_is_rust_so_fast"),
        (cv, "cv"),
        (projects, "projects"),
        (brutemoji, "projects/brutemoji"),
        (wasmsweeper, "projects/wasmsweeper"),
    ];

    html_files.iter().try_for_each(|(fun, name)| {
        let p = format!("dist/{}", name);
        let p = Path::new(&p);
        DirBuilder::new().recursive(true).create(p)?;
        let mut file = File::create(p.join(&path))?;
        let markup = generate_markup(fun()?, name);

        file.write_all(&markup.into_string().as_bytes())?;
        Ok::<(), Box<dyn Error>>(())
    })
}

fn generate_css_file() -> Result<(), Box<dyn Error>> {
    let css = read_to_string(Path::new("src/app.css"))?;
    let minified_css = minify(css.as_str())?;
    let mut file = File::create(Path::new("dist/app.css"))?;
    file.write_all(&minified_css.as_bytes())?;
    Ok(())
}

fn copy_static_files() -> Result<(), Box<dyn Error>> {
    dir::copy(
        "static/",
        "dist/",
        &dir::CopyOptions {
            overwrite: true,
            buffer_size: 64000,
            skip_exist: false,
            copy_inside: true,
            content_only: true,
            depth: 0,
        },
    )?;
    Ok(())
}

fn main() {
    match generate_html_files() {
        Err(why) => panic!(
            "something went wrong generating the html files: {}",
            why.to_string()
        ),
        Ok(_) => println!("html files generated"),
    }
    match generate_css_file() {
        Err(why) => panic!(
            "something went wrong generating the css file: {}",
            why.to_string()
        ),
        Ok(_) => println!("css file generated"),
    }
    match copy_static_files() {
        Err(why) => panic!(
            "something went wrong copying static files: {}",
            why.to_string()
        ),
        Ok(_) => println!("static files copied"),
    }
}
