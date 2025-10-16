extern crate maud;

use maud::{html, Markup};

pub fn perf_testing_js() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Performance testing JavaScript"}
        p{
            "Performance in web development can refer to quite a few different things. How fast your site loads initially, how fast your site handles changes, how fast certain operations take. Generally the only way to make things faster is to do less work. This can mean caching expensive computations, sending less data over the wire, or reducing amount of wasted or unneccessary work. Compilers and browsers are advanced enough that they are quite good at implementing as many optimizations as they safely can. So, there isn't usually much to gain by `optimizing for the compiler`. We're going to go through a few different things you can do to make your site faster. These will be overviews of different options."
        }
        p{
            "In addition to the economic benefit of having a fast site, it can also be an accessibility concern. This isn't necessarily a11y accessibility, but there is some overlap. Developing nations and poorer areas don't have access to the gigabit fiber and overpowered laptops that a lot of developers are working with. There are also still people who are operating under data caps, so it's kind of rude if your site takes 5 minutes to load and uses a bunch of their data for the month."
        }
        h2{"Reducing data sent over the network"}
        p{
            "If you are running an image heavy site, like an ecommerce site, it can pay pretty big dividends to optimize your images. When I was working at Jane, I was able to shave a couple seconds off our home page load time by reducing the image size of our banner image and product images. This one is more of an art than a science because people have different tolerances for fuzziness and where they are viewing the image can impact how much you can change the images before it's noticeably degraded. If your customers are primarily viewing your site/app on mobile phones, it doesn't make a lot of sense to send a 4k resolution PNG for image previews. Image formats are not all created equal, WebP images currently provide the best general size/clarity compromise and are supported by all major browsers now. Converting uploaded images and adjusting their size/quality can be a good way to ensure that product images remain consistent and fast."
        }
        p{
            "One of the easiest things you can do to reduce the amount of data being sent over the network on the first load of your app is to reduce or eliminate packages. Almost everywhere I have worked that has a project more than a few months old ends up with multiple libraries doing the same thing. This can mean competing toast libraries, different component libraries, form or state management libraries. Consolidating duplicate packages is an easy way to reduce the amount of code you are sending. It also has the benefit of making your code more maintainable and reduces the amount cognitive load when you need to use one of those duplicated packages. Even if it's not a huge amount of bytes saved, the maintainability aspect is pretty compelling for me."
        }
        p{
            "Once you have removed any duplicate or unused packages, it can make sense to start looking at the packages you ARE using. Some packages provide A LOT of functionality, which you may or may not actually care about. Making sure you are only including the pieces of the library you are using is another way to slim down your bundle. While removing duplicate or unused code is going to be a win pretty much no matter what, we will need some data to prove further changes are having the effects we want. This is most easily done by using a `bundle-analyzer`. There are packages available for most(all?) popular build tools if you search `bundle analyzer <your build tool>`. Running your build with the bundle-analyzer enabled will generate an interactable rectangular treemap that represents heavier packages as larger rectangles. You can hover or click on any of the rectangles to get more data or zoom in. From there you can look at the different packages and make sure you aren't including the entirety of MaterialUI when you're only really using a Button. Some of the newer tools(NextJS) either automatically handle it or provide options to manually include packages for optimization."
        }
        p{
            "Removing or reducing the amount of JavaScript sent over the network also has a benefit on computation time. Even if it is never used, all of that JavaScript has to be parsed by the browser, which is a waste if you aren't even using the packages. So, there can be an improvement in how fast your app can begin starting(how long it takes for all your HTML, JS, CSS, to be loaded) AND how fast it actually starts(how long the browser takes to parse your app and any initial setup you have before the app is interactable)."
        }
        h2{"Reducing computation time"}
        p{
            "First thing is to measure the problem with something like a flame graph. Optimizing blindly often doesn't yield the desired result. I usually then start commenting out code to further narrow down the actual problem. Once I have that isolated, there are a number of different optimizations that might help. Memoizing data and children to prevent re-rendering, hiding offscreen data or stuff that isn't relevant to the current actions, you might also fake the speed by implementing some partial rendering or loading screen that makes the changes feel faster even if the whole operation is taking the same amount of time. Ultimately the fastest code is code that doesn't get run, so identifying if there are expensive operations that can either be deferred, done in the background, parallelized, or removed completely is also usually a good avenue if the datasets are big. Using a web worker can also provide some speed gains assuming the speed gains outweigh the transfer losses. "
        }
    };
    Ok(h)
}
