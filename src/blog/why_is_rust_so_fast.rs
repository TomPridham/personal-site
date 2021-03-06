extern crate maud;

use maud::{html, Markup};

pub fn why_is_rust_so_fast() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        script src="https://remarkjs.com/downloads/remark-latest.min.js"{}
        script src="https://cdn.plot.ly/plotly-latest.min.js"{}
        textarea id="source"{
            "
# Why is Rust so Fast?
---
# Memory Management
Rust's borrow system<sup>1</sup> enables it to have the same safety guarantees of a GC language with the performance of a manually managed language.
The rust compiler keeps track of references to variables and inserts drop statements at compile time. This eliminates the periodic pause for garbage collection and the wave cycle in used memory.

.footnote[1: As opposed to garbage collection in C#/JS/etc and manual memory management in C/C++.]
---
# Aggressive Optimization
Rust has a lot of built in language features that enable lots of optimizations to be made to idiomatic code. Immutability by default, algebraic data types, zero-cost abstractions<sub>2</sub> over complicated language features, and memory management model all allow for the rust compiler to perform some aggressive optimizations that either aren't possible or aren't safe to do in other languages.

.footnote[2: Zero-cost means there is no _extra_ cost to use an abstraction vs doing it yourself using lower level code. This means that there is no need to \"code for the compiler\", idiomatic Rust is almost always going to be optimal Rust.]
---
# LLVM
Rust uses the LLVM<sub>3</sub> as its final compile step and gets to take advantage of all of the optimizations and work that has gone into that project.

.footnote[3: Low Language Virtual Machine, originally developed for Swift, it now powers lots of languages(Julia, Erlang, Swift, Rust, to name a few).]
---
# Profile Guided Optimization
This came out in the last release. It allows you to run your program against a test data set, record which paths are the most likely to be taken(hot and cold) and then recompile it optimizing for those paths. You can specify hot and cold paths manually, but this takes a lot of the work out of it.
---
name:ab
---
name:hey
---
# Hello World Server Notes
* I used whatever hello world server was found on each websites respective website. The only things I changed were turning off logging in the Python, Ruby, and .Net projects.

* I used `ab` initially because it was already installed. It's super old and then just stopped working after I ran a couple tests, so I switched to the newer `hey` benchmarker. I'm not sure why the numbers are so different. I got 135k rps for the rust server the first time I ran ab, but only around 100k the time or two I ran it later.

* I really feel like I must have done something wrong with the Python server. Python is slow, but it can't be that slow. There is a big warning not to use it in a production environment, but the only difference I could see when reading about it was that you're supposed to put it behind something like nginx. So maybe it's just really bad at that part and ok at other server things.

* Not super scientific, but I already spent too much time on this.
                "

        }

        script{
          "
      remark.create();
      var layout = {
        title: 'Hello World Server - Apache Benchmark',
        yaxis: {title: 'Requests per Second'}, // set the y axis title
        autosize: false,
        widht: 750,
        height: 650,
      };
      var data = [
        {
          x: [
            'Python/Flask',
            'Ruby/Sinatra',
            'JS/node',
            'C#/.Net',
            'Rust/actix-web',
          ],
          y: [547, 3090, 34946, 92126, 108377],
          type: 'bar',
        },
      ];

      Plotly.newPlot('slide-ab', data, layout);
      var layout2 = {
        title: 'Hello World Server - hey',
        yaxis: {title: 'Requests per Second'}, // set the y axis title
        autosize: false,
        widht: 750,
        height: 650,
      };
      var data2 = [
        {
          x: [
            'Python/Flask',
            'Ruby/Sinatra',
            'JS/node',
            'C#/.Net',
            'Rust/actix-web',
          ],
          y: [244, 3073, 21645, 52775, 60314],
          type: 'bar',
        },
      ];

      Plotly.newPlot('slide-hey', data2, layout2);
          "
        }
    };
    Ok(h)
}
