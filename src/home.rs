extern crate maud;
use maud::{html, Markup};

pub fn home() -> Markup {
    let name = "Tom Pridham";
    html! {
        p{"Hi, I'm " (name) "!"}
        p{"Nice to meet you."}
        p{"This is my website. It's written in Rust using a HTML templating library called Maud."}
        p{"I have been a software engineer since 2016. I started out doing purely frontend. I've ended up doing whatever was necessary to keep team momentum up. This has ranged from running meetings while our team lead was on paternity leave, to updating CI/CD pipelines to enable better testing, to updating C# apis, integrating data ingestion platforms to gather user metrics, and building a Windows Event agent in Rust. I enjoy learning new things and am never content to stand still. I like improving accessibility, testability, and performance."}
        p{
            "My contact info is here."
            ul{
                li{"Email: " a href="mailto:pridham.tom@gmail.com"{"pridham.tom@gmail.com"}}
                li{"Phone: " a href="tel:702-285-7906"{"702-285-7906"}}
                li{"Github: " a href="https://github.com/tompridham"{"github.com/tompridham"}}
                li{"Linkedin: " a href="https://www.linkedin.com/in/tompridham/"{"linkedin.com/in/tompridham/"}}
            }
        }
    }
}
