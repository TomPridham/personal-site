extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub fn wasmsweeper() -> Result<Markup, Box<dyn Error>> {
    let wasmsweeper_html = html! {
        h1{"Wasmsweeper"}
        p{
            "A minesweeper clone written in Rust, using the library Bevy, compiled into WASM. The repo is "
                a href="https://github.com/tompridham/wasmsweeper"{"here"}
            ". It uses WebGL2, so it doesn't currently work in Safari. You might need to enable WebGL2 for it to work. If it's not working in Firefox, you might need to do: about:config => webgl.force-enabled = true"
        }
        p{"Click anywhere to get started. Right click mines to flag them"}
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
