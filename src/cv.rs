extern crate maud;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    company: String,
    duties: Vec<String>,
    #[serde(default)]
    hide: bool,
    notable_achievements: Vec<String>,
    technologies: Vec<String>,
    time: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    demo_url: Option<String>,
    description: String,
    #[serde(default)]
    hide: bool,
    repo_url: Option<String>,
    technologies: Vec<String>,
    title: String,
}

pub fn cv() -> Result<Markup, Box<dyn Error>> {
    let cv_file = read_to_string(Path::new("src/cv.json"))?;
    let cv_items: Vec<Job> = serde_json::from_str(cv_file.as_str())?;
    let projects_file = read_to_string(Path::new("src/projects/projects.json"))?;
    let projects: Vec<Project> = serde_json::from_str(projects_file.as_str())?;

    let cv_html = html! {
        div.p-row{
            h1{"Tom Pridham"}
            div.bottom-spacer{
                h2.p-hide{"Contact"}
                ul{
                    li{"Email: " a href="mailto:pridham.tom@gmail.com"{"pridham.tom@gmail.com"}}
                    li{"Phone: 702-285-7906"}
                    li{"Website: " a href="https://tompridham.me"{"tompridham.me"}}
                    li{"Linkedin: " a href="https://www.linkedin.com/in/tompridham/"{"linkedin.com/in/tompridham/"}}
                    li{"Github: " a href="https://github.com/tompridham"{"github.com/tompridham"}}
                }
            }
        }
        div.bottom-spacer{
            h2{"Skills"}
            ul.p-twocol{
                li{"React"}
                li{"Rust"}
                li{"Node"}
                li{"Typescript"}
                li{"Performance"}
                li{"Accessibility"}
                li{"Webassembly"}
            }
        }
        div.bottom-spacer{
            h2{"Job History"}
            div{
                @for cv_item in cv_items {
                    @if !cv_item.hide{
                        div.bottom-spacer.hr.job{
                            h3{(cv_item.company ) " | " (cv_item.time) " | " (cv_item.title)}
                            h4{"Duties"}
                            ul{
                                @for duty in cv_item.duties{
                                    li{(duty)}
                                }
                            }
                            h4{"Notable Achievements"}
                            ul{
                                @for n in cv_item.notable_achievements{
                                    li{(n)}
                                }
                            }
                            h4{"Technologies Used"}
                            ul.p-twocol{
                                @for t in cv_item.technologies{
                                    li{(t)}
                                }
                            }

                        }
                    }
                }
            }
        }
        div.bottom-spacer{
            h2 #projects{"Projects"}
            div{
                @for project in projects{
                    @if !project.hide{
                        div.bottom-spacer.hr.project{
                            h3{(project.title)}
                            @if project.demo_url.is_some() {
                                @let demo_url = project.demo_url.unwrap();
                                p{"Demo: " a href=(demo_url){(demo_url)}}
                            }
                            @if project.repo_url.is_some() {
                                @let repo_url = project.repo_url.unwrap();
                                p{"Repo: " a href=(repo_url){(repo_url)}}
                            }
                            p{(project.description)}
                            ul.p-twocol{
                                @for tech in project.technologies{
                                    li{(tech)}
                                }
                            }
                        }
                    }
                }
            }
        }
        div{
            h2{"Education"}
            div.education{
                h3{"DevMoutain - May 2016"}
                p{"Studied web development focused on the MEAN-Stack. DevMountain is a programming bootcamp - https://devmounta.in/web-immersive"}
                h3{"Salt Lake Community College - December 2015"}
                p{"A.S. in Mathematics"}
            }
        }
    };
    Ok(cv_html)
}
