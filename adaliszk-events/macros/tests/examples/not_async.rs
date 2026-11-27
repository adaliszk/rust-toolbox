use adaliszk_events::{event, listen, EventResult};

#[event]
struct MessageArrived
{
    message: String,
}

#[listen]
fn on_message(event: &MessageArrived) -> EventResult
{
    Ok(())
}

fn main() {}
