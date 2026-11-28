> [!WARNING]
> Depend on [`adaliszk-events`](https://crates.io/crates/adaliszk-events) instead.
> The events crate re-exports every macro from here, so you never name this crate directly.

## What it provides

`#[event]`

Attaches `#[derive(Resolvable, Clone)]` to an event struct and implements `Dispatchable`, so it
can be handed to `dispatch` and resolved by its listeners from the container.

`#[listen(event = TheEvent, tag = "...")]`

Registers a free `async fn` in `inventory` as a listener, keyed by the required
`event = TheEvent` binding. Every parameter is resolved from a `Container` by type (in any
position) at dispatch. The optional `tag` makes the listener opt-in: it subscribes only when
the application enables the tag.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

The initial version was LLM generated, but iterations and refinements from there only involved
LLM as a search engine and debugger (meaning it is for learning only).

Contributions expected to follow **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
