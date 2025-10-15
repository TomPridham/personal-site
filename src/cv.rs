extern crate maud;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    company: String,
    #[serde(default)]
    hide: bool,
    notable_achievements: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
struct Cv {
    skills: Vec<String>,
    jobs: Vec<Job>,
}

pub fn cv() -> Result<Markup, Box<dyn Error>> {
    let cv_file = read_to_string(Path::new("src/cv.json"))?;
    let cv_items: Cv = serde_json::from_str(cv_file.as_str())?;
    let jobs = cv_items.jobs;
    let skills = cv_items.skills;
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
            ul.fourcol.p-fourcol{
                @for skill in skills{
                    li{(skill)}
                }
            }
        }
        div.bottom-spacer{
            h2{"Job History"}
            div{
                @for cv_item in jobs{
                    @if !cv_item.hide{
                        div.bottom-spacer.hr.job{
                            h3{(cv_item.company ) " | " (cv_item.time) " | " (cv_item.title)}
                            h4{"Notable Achievements"}
                            ul{
                                @for n in cv_item.notable_achievements{
                                    li{(n)}
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
