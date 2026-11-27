use adaliszk_anymap::Resolvable;
use adaliszk_commands::{command, CommandResult};

#[derive(Resolvable, Clone)]
struct Config;

#[command]
fn handle_thing(_config: &Config) -> CommandResult
{
    Ok(())
}

fn main() {}
