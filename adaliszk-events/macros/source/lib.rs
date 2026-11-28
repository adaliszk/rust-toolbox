// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;

mod event;
mod listen;

///
/// # Mark a struct as a dispatchable event
///
/// Attaches `#[derive(Resolvable, Clone)]` and implements `Dispatchable`, so the struct can be
/// emitted on the `EventDispatcher` and resolved by its listeners from the container:
///
/// ```rust
/// use adaliszk_events::event;
///
/// #[event]
/// struct MessageArrived
/// {
///     channel_id: u64,
///     message: String,
/// }
///
/// fn main() {}
/// ```
///
/// Do not derive `Resolvable` or `Clone` yourself; the attribute adds them. Other derives
/// (`Debug`, etc.) can still be added in your own `#[derive(...)]` list.
///
/// ## Panics
/// - Fails compilation when given any options.
/// - Fails compilation when not placed on a struct.
///
#[proc_macro_attribute]
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream
{
    event::expand(attr, item)
}

///
/// # Register a listener for an event from a free `async fn`
///
/// The `event = TheEvent` binding names the event the listener reacts to (the registry key,
/// like `console_command`'s `args = ...`). Every parameter (the event included) is resolved
/// from the `adaliszk_events::Container` by its type, in any position, when an emitted event
/// reaches the listener:
///
/// ```rust
/// use adaliszk_events::{event, listen, EventResult};
///
/// #[event]
/// struct MessageArrived
/// {
///     message: String,
/// }
///
/// /// Reacts to every arrived message
/// #[listen(event = MessageArrived)]
/// async fn on_message_arrived(event: &MessageArrived) -> EventResult
/// {
///     Ok(())
/// }
///
/// fn main() {}
/// ```
///
/// ## Panics
/// - Fails compilation when the `event = ...` binding is missing, or on unknown options.
/// - Fails compilation when not placed on an async function.
///
#[proc_macro_attribute]
pub fn listen(attr: TokenStream, item: TokenStream) -> TokenStream
{
    listen::expand(attr, item)
}
