extern crate maud;

use maud::{html, Markup};
use std::error::Error;

pub fn about() -> Result<Markup, Box<dyn Error>> {
    let a = html! {
        p{
            "I started studying mechanical engineering at the University of Utah and (too late) realized that I only really liked the parts of my courses that involved programming. I switched to cs, took some classes, and realized it would have been another three years before I could graduate. I ended up getting an associates in math because I had enough credits and didn't want to waste any more time at university."
        }
        p{
            "After I got my associates, I attended DevMountain(a web dev bootcamp in Provo, UT) where I primarily learned AngularJS. I then freelanced for a couple months, mentored a class at DevMountain(teaching react), mentored an after hours class at DevMountain. I got my first software development job at Sundance and haven't looked back."
        }
        p{
            "Currently, I live in Millcreek with my wife and three dogs. I spend most of my time playing around with rust, doing yardwork, playing video games, or woodworking. I enjoy mountain biking, snowboarding, and indoor climbing."
        }
    };
    Ok(a)
}
