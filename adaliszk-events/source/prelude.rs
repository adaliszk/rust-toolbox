// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{Container, Dispatchable, EventRegistration};
use tokio::sync::broadcast;

/// The broadcast bus connecting emitted events to their listeners; built by
/// [`resolve_events`].
///
/// Every `#[listen]` handler collected by `inventory` gets a [`tokio::sync::broadcast`]
/// receiver on its event's channel, processed on the tokio runtime. [`emit`](Self::emit) is
/// fire-and-forget — it broadcasts the event and returns immediately; listener failures are
/// reported to stderr, not to the emitter.
///
/// Dropping the dispatcher closes the channels, which ends the subscriber loops.
pub struct EventDispatcher
{
    channels: Container,
    capacity: usize,
}

/// Wire up the event dispatcher with every registered listener subscribed.
///
/// Must be called within a tokio runtime, as the subscriber loops run on it. Each event type
/// gets one broadcast channel with a capacity of 16 pending events; a listener that falls
/// further behind skips the lagged events ([`broadcast`] semantics). Use
/// [`resolve_events_with_capacity`] to size the buffer differently.
///
/// # Usage
///
/// ```rust
/// use adaliszk_events::{event, listen, resolve_events, EventResult};
///
/// #[event] // <- derives Resolvable and Clone, and implements Dispatchable
/// struct MessageArrived
/// {
///     message: String,
/// }
///
/// /// Reacts to every arrived message
/// #[listen(event = MessageArrived)]
/// async fn on_message_arrived(event: &MessageArrived) -> EventResult
/// {
///     println!("{}", event.message);
///     Ok(())
/// }
///
/// #[tokio::main]
/// async fn main()
/// {
///     let events = resolve_events();
///
///     let receivers = events.emit(MessageArrived { message: "Example".to_owned() });
///     assert_eq!(receivers, 1, "one listener got the event");
/// }
/// ```
pub fn resolve_events() -> EventDispatcher
{
    resolve_events_with_capacity(512)
}

/// [`resolve_events`] with a custom per-channel capacity of pending events.
pub fn resolve_events_with_capacity(capacity: usize) -> EventDispatcher
{
    let mut dispatcher = EventDispatcher { channels: Container::default(), capacity };
    for listener in inventory::iter::<&'static dyn EventRegistration> {
        listener.subscribe(&mut dispatcher);
    }
    dispatcher
}

impl EventDispatcher
{
    /// Broadcast an event to its listeners, returning how many received it.
    ///
    /// Fire-and-forget: the listeners process the event on the tokio runtime while the caller
    /// moves on. Returns `0` when no listener is registered for the event type.
    pub fn emit<EVENT: Dispatchable>(&self, event: EVENT) -> usize
    {
        self.channels
            .borrow::<broadcast::Sender<EVENT>>()
            .map(|sender| sender.send(event).unwrap_or(0))
            .unwrap_or(0)
    }

    #[doc(hidden)] // Called by the macro-generated `EventRegistration::subscribe` only
    pub fn attach<EVENT: Dispatchable>(&mut self, listener: &'static dyn EventRegistration)
    {
        let sender = match self.channels.borrow::<broadcast::Sender<EVENT>>() {
            Some(sender) => sender.clone(),
            None => {
                let (sender, _) = broadcast::channel::<EVENT>(self.capacity);
                self.channels.bind(sender.clone());
                sender
            }
        };

        let mut receiver = sender.subscribe();
        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        let mut container = Container::default();
                        container.bind(event);
                        if let Err(error) = listener.resolve(&container).await {
                            eprintln!("[adaliszk-events] listener failed: {error}");
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        });
    }
}
