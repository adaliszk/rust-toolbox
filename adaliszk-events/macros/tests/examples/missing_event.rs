use adaliszk_events::{event, listen, EventResult};

#[event]
struct MessageArrived
{
    message: String,
}

// The event binding is required:
#[listen]
async fn on_message(event: &MessageArrived) -> EventResult
{
    Ok(())
}

fn main() {}
