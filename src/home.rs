extern crate maud;
use maud::{html, Markup};

pub fn home() -> Markup {
    let name = "tom pridham";
    html! {
        div id="container"{
            p{"hi, i'm " (name) "!"}
            p{"nice to meet you."}
            p{"this is my website. it's written in rust using a html templating library called maud. i think it's pretty neat."}
            p{
                "my contact info is here."
                ul{
                    li{"email: pridham.tom@gmail.com"}
                    li{"phone: 702-285-7906"}
                    li{"github: " a{"github.com/tompridham"}}
                }
            }
        }
    }
}
