extern crate maud;

use maud::{html, Markup};

pub fn windows_data_with_actors_2() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"getting windows event data with actors part 2"}
        p{
            "back in "
                a href="/blog/windows_data_with_actors"{"part 1"}
            " we successfully subscribed to the logout/login windows events and were able to print them to the console. but that's only minimally useful if we can't send that information back to the rest of our program somehow. actor libraries are a pretty popular method of passing data around in rust programs. probably the most well known is "
                a href="https://github.com/actix/actix"{"actix"}
                " of "
                a href="https://github.com/actix/actix-web"{"actix-web"}
                " fame. the main downside for us was that you need to use their runtime. which caused enough issues integrating it later in the project that i had to look for an alternative. i found the "
                    a href="https://github.com/popzxc/messages-rs"{"messages"}
                " library, which is runtime agnostic, has an api inspired by actix and just worked. so, that's what we are going to be using today."
        }
        p{
            "so, first thing's first. we need to create our actors. the messages readme has an "
            a href="https://github.com/popzxc/messages-rs#with-runtime-features"{"example"}
            " that gets us most of the way. in a new file called event_actor.rs, i have the following."
        }
        pre{
            div.code{
                code{
r#"
use messages::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct EventData(pub String);

pub struct Queue;

#[async_trait]
impl Actor for Queue {}

#[async_trait]
impl Notifiable<EventData> for Queue {
    async fn notify(&mut self, event_data: EventData, _context: &Context<Self>) {
    // do something with the data here
    }
}
"#
                }
            }
        }
        p{
            "okay, now that we have that in place we can work on integrating it into our windows event handler! first we need to make our queue available to be used within our callback. we ended the last part with this."
        }
        pre{
            div.code{
                code{
r#"
let session = 0;
let signal_event = None;
let channel_path = "Security";
let query = "EventID=4624 or EventID=4634";
let bookmark = 0;
let context = std::ptr::null_mut();
let flags: u32 = EventLog::EvtSubscribeToFutureEvents.0 as u32;
unsafe {
    EventLog::EvtSubscribe(
        session,
        signal_event,
        channel_path,
        query
        bookmark,
        context,
        Some(event_callback),
    );
}
"#
                }
            }
        }
        p{
            "instantiating it is pretty easy. surely it won't be this easy to use it?"
        }
        pre{
            div.code{
                code{
r#"
//snip
let queue = Queue::new().spawn();
let context = queue;
unsafe {
    EventLog::EvtSubscribe(
        //snip
        context,
    );
}
"#
                }
            }
        }
        p{
            "nope. trying to run this we get the following error: `expected enum 'c_void', found struct`. so now we need to turn that struct into a raw pointer to be able to pass it as our context. to do this we need to familiarize ourselves with some of the spookier(imo) parts of unsafe rust, dealing with raw pointers. we can turn a reference to our struct into a compatible c pointer in safe rust. the docs for "
                a href="https://doc.rust-lang.org/std/ffi/enum.c_void.html"{"c_void"}
            " talk about it better than i could and link to more information on pointers. but the gist of what we are doing is coercing a reference to our queue into a const pointer, then casting it as c_void pointer."
         }
        pre{
            div.code{
                code{
r#"
//snip
let queue = Queue::new().spawn();
let raw_queue: *const Address<Queue> = &queue;
let context = raw_queue as *const c_void;

unsafe {
    EventLog::EvtSubscribe(
        //snip
        context,
    );
}
"#
                }
            }
        }
        p{
            "now inside of our callback we need to turn that pointer back into a usable rust struct. this does require unsafe rust because the normal guarantees that might prevent a dangling pointer can't be upheld by the compiler once the pointer has passed the ffi boundary. first we need to convert back into a pointer of the correct type, then we need to dereference that pointer so we can access the actual struct."
        }
        pre{
            div.code{
                code{
r#"
#[no_mangle]
extern "system" fn event_callback(
    action: EventLog::EVT_SUBSCRIBE_NOTIFY_ACTION,
    p_context: *const c_void,
    h_event: isize,
) -> u32 {
    //snip
    let queue_ptr = p_context as *mut Address<Queue>;
    let queue: Address<Queue> = unsafe { &mut *queue_ptr };
    //snip
}
"#
                }
            }
        }
        p{
            "great! now we can finally call this function and be done with this. but there is a problem, `queue.notify()` is an async function, which would require we make our extern function `async` to properly await it. that doesn't work either, unfortunately. because rust represents async functions with opaque return types, this changes the return type from u32 to something else. so, now we need to find a way to run our future in a sync context. luckily, i was able to find a wonderful "
            a href="https://stackoverflow.com/a/56258784/6168502"{"answer "}
            " by shepmaster on SO that details how to implement our own waker. since the notify function doesn't actually wait for a response from the actor, thisuse should only ever hit the ready case. add all that biz to a new file called `waker.rs` and then we can use that in our callback."
        }
        pre{
            div.code{
                code{
r#"
#[no_mangle]
extern "system" fn event_callback(
    action: EventLog::EVT_SUBSCRIBE_NOTIFY_ACTION,
    p_context: *const c_void,
    h_event: isize,
) -> u32 {
    //snip
    let queue_ptr = p_context as *mut Address<Queue>;
    let queue: Address<Queue> = unsafe { &mut *queue_ptr };

    // run our async message handler in this sync context
    if drive_to_completion(queue.notify(EventData(s))).is_err() {
        return 1;
    };
    0
}
"#
                }
            }
        }
        p{
            "okay, now the next time you run that you should be able to access the xml string inside the actor and do something with it. thank u 4 coming 2 my ted talk."
        }
    };
    Ok(h)
}
