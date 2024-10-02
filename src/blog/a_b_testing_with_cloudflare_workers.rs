extern crate maud;

use maud::{html, Markup};

pub fn a_b_testing_with_cloudflare_workers() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"A/B Testing with Cloudflare Workers"}
        p{
            "At my current job(PDQ.com), we had been wanting to implement A/B testing for our marketing site. Because our site is server side rendered and we were sending down fully formed HTML on each request this was trickier than it might have otherwise been. The easiest method is to replace content at runtime based on a feature flag fetched from an API."
        }
        pre{
            div.code{
                code{
r#"
const featureFlag = await getFlagFromApi()
...
return (
    <div>
        {featureFlag === "feature-flag-a" && <p>a content<p>}
        {featureFlag === "feature-flag-b" && <p>b content<p>}
    </div
)
"#
                }
            }
        }
        p{
            "This would have resulted in content popping in or shifting once the api finally resolved, which made it a non-starter. The other method that we saw commonly in articles online was to introduce an artificial delay to allow the request to resolve before actually displaying anything 🤮."
        }
        pre{
            div.code{
                code{
r#"
const featureFlag = await getFlagFromApi()
...
return (
    <div>
        {!featureFlag && <div class="full-screen-loading-screen" />}
        {featureFlag === "feature-flag-a" && <p>a content<p>}
        {featureFlag === "feature-flag-b" && <p>b content<p>}
    </div
)
"#
                }
            }
        }
        p{
            "Both those options were undesirable because they would negatively affect our users and would negate most of the benefits of server side rendering. The other option we saw that was tailored to work with server side rendered sites require maintaining a long lived branch for each A/B test. Since the amount of traffic our site gets is relatively small, our tests would be running for multiple weeks. Having multiple feature branches would require a lot of work to keep them up to date and also to redeploy them whenever our content team published a new article 😩. Thus began the quest for a sustainable A/B testing solution that would work with minimal maintenance from us."
        }
        p{
            "It seemed like any good solution would require us to build our site once for each feature flag. So we got to work implementing that. Initially we hardcoded multiple builds into our pipeline and had the builds deployed to our kubernetes cluster at different routes. Now we just needed to figure out how to route people to them."
        }
        p{
            "A dynamic routing solution that is tailored to Kubernetes workloads seemed like a good path to start down. Lo and behold, there is a solution that promised just that! There were some tutorials online that even showed our exact A/B testing use case! So I asked our devops team to implement an Istio service mesh. Eventually they thought it was in a good working state, so we rolled it out. A couple of days later we started getting reports of weird, intermittent errors. The worst kind of error!"
        }
        p{
            "Fast forward past a couple of days of 'it works on my machine', we finally discovered the cause. The Istio routing wasn't respecting the cookie we had set to ensure people would get routed to the same pod every time. So, for their first request they might get sent to feature-flag-a.pdq.com and get the index.html, this html would then request `feature-flag-a-some-hash.js` which would sometimes resolve to the correct pod and would sometimes 404 because it was going to the wrong pod. The inherent randomness was due to the routing rules we had to assign people to buckets randomly initially. So 50% would get featureFlagA and 50% would get featureFlagB. Which would be correct if the requests were sticky so that once someone got featureFlagA they ALWAYS got featureFlagA. Our devops team wasn't sure what the error was and wasn't sure when they were going to be able to get back to fix it, so it was back to the drawing board for us."
        }
        p{
            "We have a pretty slick setup for our dev environment that automatically creates a DNS record, new Kubernetes workspace, and deploys a build to that workspace whenever a pull request is opened or new commits are pushed to the pull request. This has worked really well for our internal testing and we had worked out most of the issues already. So we thought that we could leverage similar logic to get our to our desired A/B testing state. Adapting the logic to work for A/B testing was pretty easy. Basically just read an environment variable and run the build/deploy pipeline once for each | separated feature flag."
        }
        p{
            "Now we had multiple versions of our site being built and deployed based on our Google Optimize feature flags. We just needed to figure out how to direct people to the correct pod while obfuscating the fact that they were really accessing something like asdflkbeubfdf-1.ab.pdq.com."
        }
        p{
            "We use Cloudflare for our DNS and to manage redirects for a variety of use cases. They also offer a service they call 'Workers' that is basically just a serverless function that sits in front of their DNS servers. That seemed like the perfect opportunity to consolidate the mishmash of one off redirects and get us A/B testing. As an added bonus this would let us remove a bunch of untestable rules and add tests for the rest of the existing redirects. We got started replacing the redirects and adding tests(such a big spook that there weren't any to begin with 👻) for existing ones. Once that was working, it was time to implement the A/B testing functionality."
        }
        p{
            "The actual A/B testing logic in the worker is pretty simple. It's something like this"
        }
        pre{
            div.code{
                code{
r#"
let featureFlag = getCookies('feature-flag')
if (!featureFlag){
    featureFlag = assignFlagBasedOnWeights()
    res.headers.append('Set-Cookie', `feature-flag=${featureFlag}`)
}
return fetch(`${featureFlag}.ab.pdq.com`)
"#
                }
            }
        }
        p{
            "There is some more logic that handles some normalization tasks and other business requirements, but that's the meat of it. Now whenever that person visits our site all their requests should get routed to the same bucket. When our content team publishes something or we merge new code to main, it will trigger our normal deploy process and automatically be deployed to all the different pods! Now we can finally test changes to actually confirm they are beneficial instead of just someone being like 'feels better to me'."
        }
    };
    Ok(h)
}
