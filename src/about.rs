extern crate maud;

use maud::{html, Markup};
use std::error::Error;

pub fn about() -> Result<Markup, Box<dyn Error>> {
    let a = html! {
        p{
            "i started studying mechanical engineering at the university of utah and (too late) realized that i only really liked the parts of my courses that involved programming. i switched to cs, took some classes, and realized it would have been another three years before i could graduate. i ended up getting an associates in math because i had enough credits and didn't want to waste any more time at university."
        }
        p{
            "after i got my associates, i attended devmountain(a web dev bootcamp in provo, ut) where i primarily learned angular. i then freelanced for a couple months, mentored a class at devmountain(teaching react), mentored an after hours class at devmountain. i got my first software development job at sundance and haven't looked back."
        }
        p{
            "currently, i live in millcreek with my wife and three dogs. i spend most of my time playing around with rust, doing yardwork, playing video games, or woodworking. i enjoy mountain biking and snowboarding. pre pandemic, i used to be really into climbing."
        }
    };
    Ok(a)
}
