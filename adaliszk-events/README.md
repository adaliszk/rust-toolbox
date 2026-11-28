Event dispatch over [`inventory`](https://crates.io/crates/inventory)-collected listeners, wired
through the [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap) dependency-injection
container. An event is a plain struct; listeners are free `async fn`s discovered at compile time,
and every listener parameter (the event included) is resolved from the container by type.

## Installation & Usage

```sh
cargo add adaliszk-events
```

```rust
use adaliszk_events::{event, listen, resolve_events, EventResult};

#[event] // <- derives Resolvable and Clone, and implements Dispatchable
struct MessageArrived
{
    channel_id: u64,
    message: String,
}

/// Reacts to every arrived message
#[listen(event = MessageArrived)]
async fn on_message_arrived(event: &MessageArrived) -> EventResult
{
    println!("#{}: {}", event.channel_id, event.message);
    Ok(())
}

// Start the bus once, then emit events anywhere in your code:
#[tokio::main]
async fn main()
{
    let events = resolve_events();

    let receivers = events.emit(MessageArrived {
        channel_id: 1,
        message: "Example".to_owned(),
    });
    assert_eq!(receivers, 1);
}
```

`resolve_events` wires up the dispatcher with every registered listener subscribed on a [`tokio::sync::broadcast`] channel
per event type; `emit` is fire-and-forget: it broadcasts the event, returns how many listeners
received it, and the listeners process it on the tokio runtime while the caller moves on. A
listener may take further parameters (services, config); they resolve from the container the
same way.

[`tokio::sync::broadcast`]: https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html

## Why do this?

Short answer: purely a tool for my own projects.

Long answer: in-process events decouple the code that notices something from the code that reacts
to it. This crate formalizes that once over the same inventory + container pattern as
`adaliszk-commands`, so a new reaction is just a free `async fn`, with no central listener list
to maintain.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
