extern crate maud;

use maud::{html, Markup};

pub fn svg_vs_icon_font() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"SVG vs Icon Fonts"}
        p{
            "At my work(Jane.com) we have been using font icons. For a long time this was the defacto solution and is still the go to for lots of companies. Our designers wanted to start using more icons, though. Which caused two issues: our already large font file was going to get larger for icons that would only be used sparingly or not at all in some cases and adding new icons would require publishing a new icon font every time there was a change. The icon font we were using and the icons the designers wanted to use had already drifted apart because creating and uploading a new icon font required that the designers upload the new icons, as well as all the old ones) to Icomoon, give the new icon font to someone with s3 access so they could upload it, then we would need to update our icon component to use the new font file and to include the css for the new icons, then we would need to publish a new version of our component library and update all the apps using it. If it sounds like a really involved process, that's because it is."
        }
        p{
            "We started looking for alternatives once we had uploaded a number of one off icons and realized we were just going to end up with more and more of these 'one off' cases. There are two main ways that fonts are recommended to be served; an icon font or SVGs. There were lots of posts saying how to do it or offering opinions on why one way was better, but not a lot of quantitative data about the actual performance impact of using one over the other. Additionally, most of the writeups that we saw didn't have very good solutions that required using complicated workarounds(ex: css-filter) to get a similar level of customizability as an icon font."
        }
        p{
            "Two of the most common methods we saw were either using the SVGs as the src in `img` tags or inlining the SVG itself. Using the SVGs in `img` tags is an easy solution that really only requires the SVGs be hosted somewhere to work most of the way. You end up having to do a lot of gross css workarounds that aren't supported everywhere and have differing implementations between browsers, however. "
        }
        pre{
            div.code{
                code{
                "
const colorMap = {
  blue: 'filter: invert(39%) sepia(88%) saturate(1890%) hue-rotate(165deg) brightness(101%) contrast(101%)'
  }
const Icon = ({name, height, color}) =>
  <img src={`https://myHostingService.com/${name}`} style={{height, filter: colorMap[color]}}/>
"
                }
            }
        }
        p{
            "This method allows for the pros of having your SVGs be cached by the browser and allowing for adding arbitrary SVG icons just by adding the SVG file to the hosting location. It has the rather significant downside of requiring the images to be styled using the `filter` css attribute above. There are formulas(see https://codepen.io/sosuke/pen/Pjoqqp) that can convert a hex value to the filter, but some colors just don't work. It also requires interacting with the SVGs like images instead of as native `svg` elements. we rejected this method for those reasons."
        }
        p{
            "The other method is inlining the SVGs. Initially this seems like it might be a good method, but it's shortcomings become apparent very quickly. Inlining the SVGs allows you to interact with them as HTML elements, which makes styling them a much nicer experience. But this method doesn't allow for caching by the browser which means that the icons will need to be fetched anew on every page load. It also results in a much heavier page because of the size of an SVG is going to be larger byte-wise than an `img` tag with the src set. This only gets worse as you include more SVG icons on your page. It also brings back the original problem of having to make code changes to add any new icons. It can also make bundle splitting out unused icons problematic if not done correctly. The below SVG is about 5x larger than the equivalent `img` tag above and will need to be readded to the dom every time it is used"
        }
        div.code{
            code{
            "
const Carrot = ({style}) => <svg style={style} xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 512 512\"><path d=\"M298.2 156.6c-52.7-25.7-114.5-10.5-150.2 32.8l55.2 55.2c6.3 6.3 6.3 16.4 0 22.6-3.1 3.1-7.2 4.7-11.3 4.7s-8.2-1.6-11.3-4.7L130.4 217 2.3 479.7c-2.9 6-3.1 13.3 0 19.7 5.4 11.1 18.9 15.7 30 10.3l133.6-65.2-49.2-49.2c-6.3-6.2-6.3-16.4 0-22.6 6.3-6.2 16.4-6.2 22.6 0l57 57 102-49.8c24-11.7 44.5-31.3 57.1-57.1 30.1-61.7 4.5-136.1-57.2-166.2zm92.1-34.9C409.8 81 399.7 32.9 360 0c-50.3 41.7-52.5 107.5-7.9 151.9l8 8c44.4 44.6 110.3 42.4 151.9-7.9-32.9-39.7-81-49.8-121.7-30.3z\"/></svg>
"
            }
        }
        p{
            "We came up with a third method that is a little more complicated, but that brings all the benefits of both approaches with none of the downsides. You can fetch the SVGs as images and then insert them into the DOM using `innerHtml` and then reference them with `use` tags. This allows for caching the responses, styling the SVGs using normal SVG selectors, and only adding required icons to the DOM once."
        }
        pre{
            div.code{
                code{
                "
const Icon = ({name}) => {
  if (!document.querySelector(`#${name}`)) {
    window.fetch(`https://myHostingService/${name}.svg`)
      .then(res => {
        if (res.ok) {
          res.text().then(svg => {
            const el = document.createElement('div')
            el.innerHTML = svg
            el.firstChild.id = icon
            document.querySelector('#icon-container').appendChild(el)
          })
        }
      })
    }
  }
  return <svg><use href={`#${name}`} xlinkHref={`#${name}`} /></svg>
}
                    "
                }
            }
        }
        p{
            "The only caveat that we have found so far was that iOS didn't start supporting the `href` attribute on `<svg>` tags until about July 2019. The workaround for this is adding that `xlinkHref` attribute to the SVG. While deprecated, it is necessary for this to be a feasible solution and won't cause any harm in more modern browsers."
        }
    };
    Ok(h)
}
