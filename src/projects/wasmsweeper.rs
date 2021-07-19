extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub fn wasmsweeper() -> Result<Markup, Box<dyn Error>> {
    let wasmsweeper_html = html! {
        h1{"wasmsweeper"}
        p{
            "a minesweeper clone written in rust, using the library bevy, compiled into wasm. the repo is "
                a href="https://github.com/tompridham/wasmsweeper"{"here"}
            "."
        }
        script type="module"{
"
  import init from './minesweeper.js';
  init('./minesweeper_bg.wasm').then(function (wasm) {
    wasm.run();
  });
"

        }
    };
    Ok(wasmsweeper_html)
}
