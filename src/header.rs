extern crate maud;
use maud::{html, Markup};

pub fn header_script(location: &str) -> Markup {
    html! {
        script type="text/javascript"{
"
const active = document
  .querySelector('.active-header');
if(active){
  active.classList.remove('active-header');
}
document
  .getElementById('"(location)"')
  .classList
  .add('active-header')"
        }
    }
}

pub fn header_html() -> Markup {
    html! {
        div id="header" class="space-between"{
            a id="home" href="/"{
                span {"tompridham.me"}
            }
            ul id="menu" {
                li{
                    a id="cv" href="/cv"{"cv"}
                }
                li{
                    a id="about" href="/about"{"about"}
                }
                li{
                    a id="blog" href="/blog"{"blog"}
                }
                //li{
                    //a id="projects" href="/projects"{"projects"}
                //}
                li{
                    a href="https://github.com/tompridham"{"github"}
                }
            }
        }
    }
}
