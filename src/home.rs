extern crate maud;
use maud::{html, Markup};

pub fn home() -> Markup {
    let name = "Tom Pridham";
    html! {
        p{"Hi, I'm " (name) "!"}
        p{"Nice to meet you."}
        p{"This is my website. It's written in Rust using a HTML templating library called Maud."}
        p{"I like making things that are accessible, testable, and performant."}
        p{
            "My contact info is here."
            ul{
                li{"Email: pridham.tom@gmail.com"}
                li{"Phone: 702-285-7906"}
                li{"Github: " a href="https://github.com/tompridham"{"github.com/tompridham"}}
                li{"Linkedin: " a href="https://www.linkedin.com/in/tompridham/"{"linkedin.com/in/tompridham/"}}
            }
        }
    }
}
