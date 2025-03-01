extern crate maud;

use maud::{html, Markup};

pub fn updating_a_dsl() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Updating a DSL for Consistency and Testability"}
        p{
            "When I was working at Osmos, we had a Microsoft Excel like DSL that would allow users to apply functions to their data. The language had sort of grown organically over time and was largely untested. Because problems were addressed as they were brought up by customers, there were a number of inconsistencies in how different functions behaved and how type coercion worked. I undertook the large task of resolving some of those inconsistencies and adding a large number of test cases."
        }
        p{
            "This started out as just writing test cases for the existing behavior, since the majority of it was undocumented and there was uncertainty about current behavior vs desired behavior. As I was writing tests, I was also documenting the current behavior, providing an assessment of the intuitiveness and potential solutions for cases that were surprising. The main thing we wanted to reconcile were certain 'gotchas' for our users, where they were expecting the language to behave one way and were surprised when it acted a different way. Once I was done adding tests and documenting, I provided the document to our product and customer service people so they could review and provide feedback."
        }
        p{
            "We went through a couple iterations of feedback and implementing suggestions. The tests came in handy again because the feedback often would seem good in natural language, but would have unforeseen consequences that the test cases exposed. It was really nice to easily be able to show the effects of the changes with simple test diffs. Once we arrived at something everyone was happy with, we could begin the process of rolling it out."
        }
        p{
            "Because these changes mostly affected edge cases and often ended with customers contacting us about workarounds, we were pretty confident that the changes we had made wouldn't affect any of our existing customers. We did manually check a pretty big chunk of formulas currently in use to be extra sure it wouldn't have any unexpected effects. Once we had done that, we rolled out the feature and our CS team messaged our customers about the changes."
        }
        p{
            "This set of changes helped reduce the amount of times our CS team had to interface with confused customers about our DSL. It also greatly increased the amount of tested code in the DSL, documented previously undocumented behaviors and made that portion of the DSL more idiomatic and consistent."
        }
    };
    Ok(h)
}
