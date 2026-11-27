use adaliszk_anymap::Resolvable;
use std::rc::Rc;

// The container stores `Send + Sync` values, so a non-`Send` type cannot be resolved.
#[derive(Resolvable, Clone)]
struct NotSend
{
    value: Rc<String>,
}

fn main() {}
