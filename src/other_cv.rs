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
struct VolunteerExperience {
    #[serde(default)]
    hide: bool,
    organization: String,
    experience: Vec<String>,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cv {
    skills: Vec<String>,
    jobs: Vec<Job>,
    volunteering: Vec<VolunteerExperience>,
}

pub fn other_cv() -> Result<Markup, Box<dyn Error>> {
    let cv_file = read_to_string(Path::new("src/other_cv.json"))?;
    let cv_items: Cv = serde_json::from_str(cv_file.as_str())?;
    let jobs = cv_items.jobs;
    let skills = cv_items.skills;
    let volunteering_experience = cv_items.volunteering;

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
                        div.bottom-spacer.hr.other_job{
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
            h2 #volunteer{"Volunteer Experience"}
            div{
                @for vexp in volunteering_experience {
                    @if !vexp.hide{
                        div.bottom-spacer.hr.volunteer{
                            h3{(vexp.organization) " | " (vexp.time)}
                            h4{"Experience"}
                            ul{
                                @for exp in vexp.experience{
                                    li{(exp)}
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
