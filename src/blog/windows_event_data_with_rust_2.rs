extern crate maud;

use maud::{html, Markup};

pub fn windows_event_data_with_rust_2() -> Result<Markup, Box<dyn std::error::Error>> {
    let h = html! {
        h1{"Getting Windows Event Data with Rust Part 2"}
        p{
            "Back in "
                a href="/blog/windows_event_data_with_rust"{"Part 1"}
            " we successfully subscribed to the logout/login Windows events and were able to print them to the console. But that's only minimally useful if we can't send that information back to the rest of our program somehow. One way to get that data back into our main program is via "
                a href="https://docs.rs/tokio/0.1.16/tokio/sync/mpsc/index.html" {"MPSC channels"}
            ". MPSC stands for `multiple producer, single consumer`. Which means that there will be a single handler, but you could potentially subscribe to multiple different Windows events with the same channel."
        }
        p{
            "First we need to make our sender available to be used within our callback. We ended the last part with this:"
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
            "Instantiating the channel is pretty easy. Surely it won't be this easy to use it?"
        }
        pre{
            div.code{
                code{
r#"
//snip
let (sender, mut receiver) = unbounded_channel();
let context = sender;
unsafe {
    EventLog::EvtSubscribe(
        //snip
        context,
        Some(event_callback),
    );
}

loop {
    if let Some(s_event) = receiver.recv().await {
        println!("received event: {}", s_event.trim());
    }
}
"#
                }
            }
        }
        p{
            "Nope. Trying to run this we get the following error: `expected enum 'c_void', found struct`. So now we need to turn that struct into a raw pointer to be able to pass it as our context. To do this we need to familiarize ourselves with some of the spookier(imo) parts of unsafe rust, dealing with raw pointers. We can turn a reference to our struct into a compatible c pointer in safe rust. The docs for "
                a href="https://doc.rust-lang.org/std/ffi/enum.c_void.html"{"c_void"}
            " talk about it better than I could and link to more information on pointers. But the gist of what we are doing is coercing a reference to our producer into a const pointer, then casting it as c_void pointer."
         }
        pre{
            div.code{
                code{
r#"
//snip
let (mut sender, mut receiver) = unbounded_channel();
let context = &mut sender as *mut UnboundedSender<String> as *const c_void;

unsafe {
    EventLog::EvtSubscribe(
        //snip
        context,
        Some(event_callback),
    );
}

loop {
    if let Some(s_event) = receiver.recv().await {
        println!("received event: {}", s_event.trim());
    }
}
"#
                }
            }
        }
        p{
            "Now inside of our callback we need to turn that pointer back into a usable rust struct. This does require unsafe rust because the normal guarantees that might prevent a dangling pointer can't be upheld by the compiler once the pointer has passed the FFI boundary. First we need to convert back into a pointer of the correct type, then we need to dereference that pointer so we can access the actual struct."
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
    //
    let sender_ptr = p_context as *mut UnboundedSender<String>;
    let sender = unsafe { &mut *sender_ptr };
    //snip
}
"#
                }
            }
        }
        p{
            "There is still a problem here though. The sender will get dropped after the first invocation of the callback, which isn't great because that subscription is relying on the callback function working until it is unsubscribed(i.e. forever). So after the first invocation of the callback we will have an invalid pointer. We need to do something to make the sender persist for as long as the program does. Enter "
            a href="https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak"{"Box::leak"}
            ". This will let us put the sender on the heap and then leak it to prevent its destructor from being run."
        }
        pre{
            div.code{
                code{
r#"
let (mut sender, mut receiver) = unbounded_channel();

let sender = Box::new(sender);
let s = Box::leak(sender.clone());

let context = s as *mut UnboundedSender<String> as *const c_void;
unsafe {
    EventLog::EvtSubscribe(
        //snip
        context,
        Some(event_callback),
    );
}
"#
                }
            }
        }
        p{
            "The last thing we need to do is update our pointer conversion in the callback."
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
    //
    let sender = unsafe {
        (p_context as *mut UnboundedSender<String>)
            .as_ref()
            .unwrap()
    };
    //snip
}
"#
                }
            }
        }
        p{
            "Okay, now the next time you run that you should be able to access the XML string inside the main loop and do something with it. You can view working code "
            a href="https://github.com/TomPridham/windows-events-rust"{"here"}
            ". thank u 4 coming 2 my ted talk."
        }
    };
    Ok(h)
}
