extern crate maud;

use maud::{html, Markup};

pub fn engineering_solutions() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Engineering Solutions"}
        p{
            "Usability is the primary concern and the only one that can't be compromised on. `Usability` is a broad term that encompasses accessibility, performance, intuitiveness, and many others. It doesn't really matter how elegant, performant or shiny a feature is if no one can or will use it. That's not to say those things aren't unimportant, but they are secondary and should not compromise usability. It boils down to `does this thing do the thing that people need it to do and can they do it with the minimum amount of inconvenience`. "
        }
        p{
            "Only after you have a working solution can you start to make other decisions and optimize for different variables. Once I have something that works, I will almost always turn to testability. It's great to have something that works, but it's better to be confident it will continue to work once you start changing things. For frontend projects, this goes hand in hand with ensuring it is accessible. Modern testing frameworks emphasize interacting with elements in an accessible way. This ensures that you test the app as closely as possible to how a real user will interact with it. Once you have tests that will enable you to ensure your feature continues to work, then you can move on to things that ensure the feature will be more maintainable and less likely to fail in the future."
        }
        p{
            "The next area I look at is maintainability. This can be as simple as renaming variables or completely refactoring a complicated piece of logic. In general you want to try to make your code clear enough that it won't confuse you in a few months when you have to come back to it."
        }
        p{
            "Performance is kind of a weird category because it can have impact on all the other categories we have talked about so far. If your feature is so slow it makes interacting with it difficult or unpleasant, that is impacting usability. If it's so slow it's causing your tests to time out in CI, that's impacting both maintainability and testability. You don't want to try and optimize performance too early, however. There's no point optimizing something that might not end up working."
        }
        p{
            "The final consideration is not technical, it's the expectations the business, product, and your coworkers have for the feature. You have to balance all of the constraints above with what the business needs. It doesn't help anyone if you deliver the perfect feature 6 months past the deadline. You might have to cut aspects of a feature to ensure it is delivered on time. Keeping everyone updated on progress, current challenges and their potential solutions is critical. You should be talking with your team at any critical juncture to make sure it meets everyones expectations."
        }
    };
    Ok(h)
}
