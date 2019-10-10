extern crate maud;
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Job{
    company: String,
    duties: Vec<String>,
    notable_achievements: Vec<String>,
    technologies: Vec<String>,
    time: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project{
    description: String,
    technologies: Vec<String>,
    title:String
}

#[derive(Debug, Serialize, Deserialize)]
struct CV{
    projects: Vec<Project>,
    work: Vec<Job>
}

pub fn cv() -> Result<Markup, Box<dyn Error>> {
    let cv_file = read_to_string(Path::new("src/cv.json"))?;
    let cv_items:CV = serde_json::from_str(cv_file.as_str())?;

    let cv_html = html! {
        div.row{
            h1{"tom pridham"}
            h2{"contact"}
            ul{
                li{"email: pridham.tom@gmail.com"}
                li{"phone: 702-285-7906"}
                li{"website: tompridham.github.io"}
                li{"github: github.com/tompridham"}
            }
        }
        div.row{
            h2{"skills"}
            ul{
                li{"react"}
                li{"graphql"}
                li{"rust"}
                li{"typescript"}
                li{"angularjs"}
                li{"node"}
                li{"postgres"}
            }
        }
        div.row{
            h2{"job history"}
            div{
                @for cv_item in cv_items.work {
                    h3{(cv_item.company ) " - " (cv_item.time)}
                    p{(cv_item.title)}
                    h4{"duties"}
                    ul{
                        @for duty in cv_item.duties{
                            li{(duty)}
                        }
                    }
                    h4{"notable achievements"}
                    ul{
                        @for n in cv_item.notable_achievements{
                            li{(n)}
                        }
                    }
                    h4{"technologies used"}
                    ul{
                        @for t in cv_item.technologies{
                            li{(t)}
                        }
                    }
                }
            }
        }
        div.row{
            h2{"projects"}
            div{
                @for project in cv_items.projects{
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
        div.row{
            h2{"education"}
            div{
                h3{"devmoutain - May 2016"}
                p{"studied web development focused on the mean-stack. devmountain is a programming bootcamp - https://devmounta.in/web-immersive"}
                h3{"salt lake community college - december 2015"}
                p{"a.s. in mathematics"}
            }
        }
    };
    Ok(cv_html)
}
