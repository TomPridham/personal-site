extern crate maud;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    description: String,
    technologies: Vec<String>,
    title: String,
}

pub fn projects() -> Result<Markup, Box<dyn Error>> {
    let projects_file = read_to_string(Path::new("src/projects.json"))?;
    let projects: Vec<Project> = serde_json::from_str(projects_file.as_str())?;

    let projects_html = html! {
        h1{"Projects"}
        div.row{
            div{
                @for project in projects{
                    h3{(project.title)}
                    p{(project.description)}
                    ul{
                        @for tech in project.technologies{
                            li{(tech)}
                        }
                    }
                }
            }
        }
    };
    Ok(projects_html)
}
