extern crate maud;

use maud::{html, Markup};

pub fn developing_the_right_way() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Developing A Big Feature the Right Way"}
        p{
            "When I worked at Jane(a now defunct ecommerce company with a similar model to Amazon), we had a web app that allowed third party sellers to manage and sell their products. A major constraint was that when things were initially set up, a seller was just a single person with an account. So, as more sellers grew their businesses and hired additional employees, if they wanted to have those employees help manage inventory or orders on our site they would have to share their login credentials. Which is obviously not ideal."
        }
        p{
            "The process to move sellers from a single account to an organization had been started shortly before I joined, with the backend just starting development. The change was going to be a total rethink of what a `seller` was, which would require a lot of changes to both backend and frontend systems. I was working exclusively on frontend at this point. A lot of the initial work was to move the app to a more modern React stack. The app was large enough that doing it all in one go was not a viable strategy. So we had to do it piecemeal in such a way that we could continue to support the legacy system and the new workflows being developed."
        }
        p{
            "Replacing old code 1 for 1 is something I really enjoy. We were able to modernize pieces of the codebase that hadn't been updated in years. We were also able to significantly increase the test coverage from near zero to around 70% by the time the project ended. Doing the project piecemeal allowed for the work to be chunked up in such a way that there wasn't one mega branch that required constant rebasing to avoid merge conflicts."
        }
        p{
            "The overall process was relatively smooth for the majority of the project. Our product manager(Whitney Johnson) was awesome and did a great job gathering requirements from customers and synthesizing those requirements into manageable chunks of work. Most of the problems we encountered were from trying out newer technologies(GraphQL) that were still maturing or from our own lack of knowledge. There were certain things our backend team wanted to do, but just wasn't possible given the state of the C# library they were using. Luckily, the GraphQL ecosystem for the web was more developed."
        }
        p{
            "Figuring out a good organization and testing strategy was one of the biggest early hurdles. Originally, everything was in one `view` file. This was nice for only needing to look one place for everything but made it really hard to test specific pieces of functionality. It also made testing in general difficult because we couldn't easily mock pieces of functionality since they weren't exposed in the main file. We eventually broke things out into a couple distinct logical pieces: GraphQL queries, presentational React components and recompose HOCs. Splitting things up in this way let us test every piece of the app as distinct, small, deterministic units."
        }
        p{
            "Integrating with the changes the backend team was making was made relatively painless because we agreed on contracts ahead of time and could make small adjustments easily by adjusting the GraphQL query we were using. Despite challenges with library maturity, working with GraphQL was a joy and made the whole project significantly nicer from the FE."
        }
        p{
            "We were able to gradually release features because we had feature flags that determined if it used the old or new service. It was really nice to be able to turn the features on for specific customers to get feedback before doing a general release. Because we were still supporting both paths, we were also able to turn off the new features when we identified workflow issues. Because we were getting feedback throughout the process, had rigourous testing and QA processes, and were able to gradually introduce changes the final release process was relatively painless. Once we had the full set of features and had done a final round of QA and customer feedback we were able to turn the feature on for everyone."
        }
        p{
            "The whole feature was a pretty big success and was well received by our seller community. It solved a lot of longstanding issues and modernized a lot of our tech stack. Overall, it was only possible because the whole team was great and worked so well together."
        }
    };
    Ok(h)
}
