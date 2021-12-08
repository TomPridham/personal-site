extern crate maud;

use maud::{html, Markup};

pub fn javascript_detective_work() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"javascript detective work 🕵️"}
        p{
            "when i first started at pdq, i was poking around our marketing site trying to familiarize myself
with the code i would be working on. one of the first things i check are lighthouse scores.
there are often easy wins called out that haven't been actioned on and they are a good way to
get your feet wet in a repo. one of the first things i noticed was that our index file was 7mb!
much larger than it had any reason to be, but surprisingly it wasn't obscenely slow the way i
would have expected a js file that large to be."
        }
        p{
            "i inspected the file and there were only a few thousand lines once it was un-uglified, definitely
not 7mb worth of lines. i was initially quite perplexed because this file was huge, but didn't look
like it should be, neither from the performance or apparent amount of code. eventually i
resolved to just go through it more slowly line by line to see if i saw anything absolutely wacky."
        }
        p{
            "my efforts were rewarded when i stumbled on a single string that was unbelievably long.
because normal js strings don't allow line breaks mid string, it looked like a normal, if slightly
long, string until i put it in my editor with line wrapping. i had found the culprit. a 6mb json blob
was being included in our bundle somehow. that explained why the performance wasn't
absolute trash. js engines can skip a lot of really complicated parsing rules if they know
something is json and not potentially a javascript object that could contain actual javascript
code that needs to be executed. it's actually a pretty nice performance trick to include large
objects(such as a redux store for hydrating some ssr app) as "
            a href="https://v8.dev/blog/cost-of-javascript-2019#json"{"json blobs"}
            " instead of js objects."
        }
        p{
            "now i just had to figure out what it contained and why it was being included. annoyingly, the
issue was only present in the production build, so all the variables were mangled and didn't offer
any insight into what the blob was being used for. looking at the blob itself, there was a lot of
unhelpful stuff in there that seemed like generated code. i was able to pick out a few urls,
however, which gave me my first real lead."
        }
        p{
            "the urls pointed to images from our blog posts. so, now i had a direction to look. i just had to
familiarize myself with both the gatsby build system and its interaction with our cms,
contentful. after a few days of reading through the docs and talking through things with my
coworker who had done all of the work on the site up until i joined, i thought i had an idea of
where the issue was. there was an odd interaction in a graphql query between gatsby and contentful that was fetching a list of all our assets and then including that in the main bundle. i tried a few different things that didn't work. so i decided to shelve it for a bit and work on other things while it was percolating in the back of my mind."
        }
        p{
            "i came back to it a few weeks later and was able to form a better search incantation and
    stumbled upon an "
            a href="https://github.com/contentful/rich-text/issues/70"{"issue"}
            " that seemed to be talking about the portion of our build process that i
thought was to blame. there was a comment in there that had code that my coworker had
actually implemented and then forgotten about, but it was enough to get me started on the right
track. first i tried a few ways to remove the json blob since that was my main goal and i
figured i could work backwards from that to fix any issues."
        }
        p{
            "i was able to successfully remove the json blob! but it also removed all images from our blog
posts, which was not great. it seemed i needed to include the images, but i was hoping i could
only include the images that were necessary for a specific page. i filtered out the images from
the main list that weren't referenced in a specific blog post and passed that to the component
rendering the blog posts. and it worked! the blogs were rendering their images again and that
gigantic json blob wasn't being included in our bundle anymore. i was able to get our bundle
from 6.8mb to 650kb!"
       }
        p{
            "i came up with a "
            a href="https://github.com/contentful/rich-text/issues/70#issuecomment-585367684"{"minimal repro"}
            " that i posted back to that thread to hopefully help other lost souls."
        }
    };
    Ok(h)
}
