extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub fn wasmsweeper() -> Result<Markup, Box<dyn Error>> {
    let wasmsweeper_html = html! {
        h1{"wasmsweeper"}
        p{
            "a minesweeper clone written in rust, using the library bevy, compiled into wasm. the repo is "
                a href="https://github.com/tompridham/wasmsweeper"{"here"}
            ". it uses webgl2, so it doesn't currently work in safari. you might need to enable webgl2 for it to work. if it's not working in firefox, you might need to do: about:config => webgl.force-enabled = true"
        }
        p{"click anywhere to get started. right click mines to flag them"}
        script type="module"{
"
import init from './minesweeper.js';
init('./minesweeper_bg.wasm').then(function (wasm) {
    wasm.run();
});
"
        }
        script type="text/javascript"{
"
window.addEventListener('contextmenu', function(){
    document.querySelector('canvas').oncontextmenu= function(e){e.preventDefault()}
});
"
        }

    };
    Ok(wasmsweeper_html)
}
