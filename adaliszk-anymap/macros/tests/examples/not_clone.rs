use adaliszk_anymap::Resolvable;

// The derive clones the bound value out, so the type must be `Clone`.
#[derive(Resolvable)]
struct NotClone
{
    value: String,
}

fn main() {}
