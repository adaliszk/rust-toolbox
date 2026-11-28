// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_events::{event, listen, resolve_events, Container, EventRegistration, EventResult};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::time::timeout;

async fn confirmations(receiver: &mut UnboundedReceiver<String>, count: usize) -> Vec<String>
{
    let mut received = Vec::new();
    for _ in 0..count {
        let confirmation = timeout(Duration::from_secs(5), receiver.recv())
            .await
            .expect("the listeners confirm in time")
            .expect("the channel stays open");
        received.push(confirmation);
    }
    received
}

#[event]
struct MessageArrived
{
    message: String,
}

#[event]
struct UserJoined
{
    _name: u8,
}

static MESSAGES: OnceLock<UnboundedSender<String>> = OnceLock::new();
static JOINS: AtomicU32 = AtomicU32::new(0);

/// Records every arrived message
#[listen(event = MessageArrived)]
async fn on_message_record(event: &MessageArrived) -> EventResult
{
    if let Some(confirm) = MESSAGES.get() {
        let _ = confirm.send(format!("record:{}", event.message));
    }
    Ok(())
}

/// A second listener for the same event
#[listen(event = MessageArrived)]
async fn on_message_count(event: &MessageArrived) -> EventResult
{
    if let Some(confirm) = MESSAGES.get() {
        let _ = confirm.send(format!("count:{}", event.message));
    }
    Ok(())
}

/// Reacts to a different event, so it must not run for messages
#[listen(event = UserJoined)]
async fn on_user_joined(_event: &UserJoined) -> EventResult
{
    JOINS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

#[tokio::test]
async fn emit_broadcasts_to_every_listener_of_the_event()
{
    let (confirm, mut confirmed) = unbounded_channel();
    MESSAGES.set(confirm).expect("the test owns the confirmation channel");

    let events = resolve_events();
    let receivers = events.emit(MessageArrived { message: "hello".to_owned() });
    assert_eq!(receivers, 2, "both message listeners received the event");

    let mut received = confirmations(&mut confirmed, 2).await;
    received.sort();
    assert_eq!(received, ["count:hello", "record:hello"]);
    assert_eq!(JOINS.load(Ordering::SeqCst), 0, "the unrelated listener did not run");
}

#[event]
struct Unheard
{
    _nobody: u8,
}

#[tokio::test]
async fn emit_without_listeners_reaches_nobody()
{
    let events = resolve_events();

    assert_eq!(events.emit(Unheard { _nobody: 0 }), 0);
}

#[event]
struct Simultaneous
{
    _marker: u8,
}

static MEETING: OnceLock<tokio::sync::Barrier> = OnceLock::new();
static MET: OnceLock<UnboundedSender<String>> = OnceLock::new();

fn meeting() -> &'static tokio::sync::Barrier
{
    MEETING.get_or_init(|| tokio::sync::Barrier::new(2))
}

/// Waits for its sibling, only possible when listeners run concurrently
#[listen(event = Simultaneous)]
async fn first_at_the_meeting(_event: &Simultaneous) -> EventResult
{
    meeting().wait().await;
    if let Some(confirm) = MET.get() {
        let _ = confirm.send("first".to_owned());
    }
    Ok(())
}

/// The sibling of the listener above
#[listen(event = Simultaneous)]
async fn second_at_the_meeting(_event: &Simultaneous) -> EventResult
{
    meeting().wait().await;
    if let Some(confirm) = MET.get() {
        let _ = confirm.send("second".to_owned());
    }
    Ok(())
}

/// Both listeners block on a shared barrier, so sequential processing would deadlock:
#[tokio::test]
async fn listeners_process_concurrently_on_the_runtime()
{
    let (confirm, mut confirmed) = unbounded_channel();
    MET.set(confirm).expect("the test owns the confirmation channel");

    let events = resolve_events();
    assert_eq!(events.emit(Simultaneous { _marker: 0 }), 2);

    let mut received = confirmations(&mut confirmed, 2).await;
    received.sort();
    assert_eq!(received, ["first", "second"]);
}

#[derive(adaliszk_anymap::Resolvable, Clone)]
struct Needed
{
    id: u32,
}

#[event]
struct WithDependency
{
    _marker: u8,
}

/// Needs a service beyond the event itself, and takes it FIRST: parameters resolve by
/// type in any position, the binding names the event.
#[listen(event = WithDependency)]
async fn on_with_dependency(needed: &Needed, _event: &WithDependency) -> EventResult
{
    assert_eq!(needed.id, 9);
    Ok(())
}

#[event]
struct ExplicitlyBound
{
    _marker: u8,
}

static EXPLICIT: OnceLock<UnboundedSender<String>> = OnceLock::new();

/// Bound to its event by the attribute
#[listen(event = ExplicitlyBound)]
async fn on_explicit(_event: &ExplicitlyBound) -> EventResult
{
    if let Some(confirm) = EXPLICIT.get() {
        let _ = confirm.send("explicit".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn an_explicitly_bound_listener_receives_its_event()
{
    let (confirm, mut confirmed) = unbounded_channel();
    EXPLICIT.set(confirm).expect("the test owns the confirmation channel");

    let events = resolve_events();
    assert_eq!(events.emit(ExplicitlyBound { _marker: 0 }), 1);

    assert_eq!(confirmations(&mut confirmed, 1).await, ["explicit"]);
}

#[event]
struct Measured
{
    _value: u64,
}

/// Ships measurements; wanted only when the telemetry tag is enabled
#[listen(event = Measured, tag = "telemetry")]
async fn on_measured_ship(_event: &Measured) -> EventResult
{
    Ok(())
}

/// Untagged, so every dispatcher subscribes it regardless of the enabled tags
#[listen(event = Measured)]
async fn on_measured_log(_event: &Measured) -> EventResult
{
    Ok(())
}

#[tokio::test]
async fn a_tagged_listener_subscribes_only_with_its_tag_enabled()
{
    let silent = resolve_events();
    assert_eq!(silent.emit(Measured { _value: 1 }), 1, "only the untagged listener");

    let telemetry = adaliszk_events::resolve_events_with_tags(&["telemetry"]);
    assert_eq!(telemetry.emit(Measured { _value: 1 }), 2, "the enabled tag adds its listener");
}

#[event]
struct SharedByClones
{
    _marker: u8,
}

/// Reached through either dispatcher clone, as they share the channels
#[listen(event = SharedByClones)]
async fn on_shared_by_clones(_event: &SharedByClones) -> EventResult
{
    Ok(())
}

#[tokio::test]
async fn a_cloned_dispatcher_shares_the_channels()
{
    let events = resolve_events();
    let cloned = events.clone();

    assert_eq!(cloned.emit(SharedByClones { _marker: 0 }), 1, "the clone reaches the listener");

    drop(events);
    assert_eq!(cloned.emit(SharedByClones { _marker: 0 }), 1, "channels outlive the original");
}

#[tokio::test]
async fn a_listener_dependency_resolves_through_a_manual_container()
{
    let mut container = Container::default();
    container.bind(WithDependency { _marker: 0 });
    container.bind(Needed { id: 9 });

    let listener = *adaliszk_events::inventory::iter::<&'static dyn EventRegistration>
        .into_iter()
        .find(|listener| listener.event() == std::any::TypeId::of::<WithDependency>())
        .expect("the listener is registered");

    listener.resolve(&container).await.expect("resolve succeeds");
}
