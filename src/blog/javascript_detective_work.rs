extern crate maud;

use maud::{html, Markup};

pub fn javascript_detective_work() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Javascript Detective Work 🕵️"}
        p{
            "When I first started at PDQ, I was poking around our marketing site trying to familiarize myself
with the code I would be working on. One of the first things I check are Lighthouse scores.
There are often easy wins called out that haven't been actioned on and they are a good way to
get your feet wet in a repo. One of the first things I noticed was that our index file was 7mb!
Much larger than it had any reason to be, but surprisingly it wasn't obscenely slow the way I
would have expected a JS file that large to be."
        }
        p{
            "I inspected the file and there were only a few thousand lines once it was un-uglified, definitely
not 7mb worth of lines. I was initially quite perplexed because this file was huge, but didn't look
like it should be, neither from the performance or apparent amount of code. Eventually I
resolved to just go through it more slowly line by line to see if I saw anything absolutely wacky."
        }
        p{
            "My efforts were rewarded when I stumbled on a single string that was unbelievably long.
Because normal JS strings don't allow line breaks mid string, it looked like a normal, if slightly
long, string until I put it in my editor with line wrapping. I had found the culprit! A 6mb JSON blob
was being included in our bundle somehow. That explained why the performance wasn't
absolute trash. JS engines can skip a lot of really complicated parsing rules if they know
something is JSON and not potentially a JS object that could contain actual
code that needs to be executed. It's actually a pretty nice performance trick to include large
objects(such as a Redux store for hydrating some SSR app) as "
            a href="https://v8.dev/blog/cost-of-javascript-2019#JSON"{"JSON blobs"}
            " instead of JS objects."
        }
        p{
            "Now I just had to figure out what it contained and why it was being included. Annoyingly, the
issue was only present in the production build, so all the variables were mangled and didn't offer
any insight into what the blob was being used for. Looking at the blob itself, there was a lot of
unhelpful stuff in there that seemed like generated code. I was able to pick out a few urls,
however, which gave me my first real lead."
        }
        p{
            "The urls pointed to images from our blog posts. Now I had a direction to look. I just had to
familiarize myself with both the Gatsby build system and its interaction with our CMS,
Contentful. After a few days of reading through the docs and talking through things with my
coworker who had done all of the work on the site up until I joined, I thought I had an idea of
where the issue was. There was an odd interaction in a GraphQL query between Gatsby and Contentful that was fetching a list of all our assets and then including that in the main bundle. I tried a few different things that didn't work. So I decided to shelve it for a bit and work on other things while it was percolating in the back of my mind."
        }
        p{
            "I came back to it a few weeks later and was able to form a better search incantation and
    stumbled upon an "
            a href="https://github.com/contentful/rich-text/issues/70"{"issue"}
            " that seemed to be talking about the portion of our build process that I
thought was to blame. There was a comment in there that had code that my coworker had
actually implemented and then forgotten about, but it was enough to get me started on the right
track. First I tried a few ways to remove the JSON blob since that was my main goal and I
figured I could work backwards from that to fix any issues."
        }
        p{
            "I was able to successfully remove the JSON blob! But, it also removed all images from our blog
posts, which was not great. It seemed I needed to include the images, but I was hoping I could
only include the images that were necessary for a specific page. I filtered out the images from
the main list that weren't referenced in a specific blog post and passed that to the component
rendering the blog posts. And it worked! The blogs were rendering their images again and that
gigantic JSON blob wasn't being included in our bundle anymore. I was able to get our bundle
from 6.8mb to 650kb!"
       }
        p{
            "I came up with a "
            a href="https://github.com/contentful/rich-text/issues/70#issuecomment-585367684"{"minimal repro"}
            " that I posted back to that thread to hopefully help other lost souls."
        }
    };
    Ok(h)
}
