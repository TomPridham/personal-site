extern crate maud;
use maud::{html, Markup};

pub fn header_script(location: &str) -> Markup {
    html! {
          script type="javascript"{
"document
  .querySelector('.active-header')
  .classList()
  .remove('active-header')"
    "document
  .getElementById("(location)")
  .classList()
  .add('active-header')"
          }
      }
}

pub fn header_html() -> Markup {
    html! {
        div id="header" class="space-between"{
            span id="home"{"tompridham.me"}
            ul id="menu" {
                li{
                    a id="cv" to="/cv"{"cv"}
                }
                li{
                    a id="about" to="/about"{"about"}
                }
                li{
                    a id="projects" to="/projects"{"projects"}
                }
                li{
                    a to="https://github.com/tompridham"{"github"}
                }
            }
        }
    }
}
