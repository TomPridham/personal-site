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
      .getElementById('"(location)"'.split('/')[0])
      .classList
      .add('active-header')
"
        }
    }
}

pub fn header_html() -> Markup {
    html! {
        nav class="space-between"{
            a id="home" href="/"{
                span {"tompridham.me"}
            }

            div {
                input type="checkbox" id="hamburger-toggle";
                label for="hamburger-toggle" class="sr-only"{"toggle nav menu"}
                span class="hamburger"{}
                span class="hamburger"{}
                span class="hamburger"{}

                ul id="menu" {
                    li{
                        a id="cv" href="/cv"{"CV"}
                    }
                    li{
                        a id="about" href="/about"{"About"}
                    }
                    li{
                        a id="blog" href="/blog"{"Blog"}
                    }
                    li{
                        a id="projects" href="/projects"{"Projects"}
                    }
                    li{
                        a href="https://github.com/tompridham"{"Github"}
                    }
                }
            }

        }
    }
}
