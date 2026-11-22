use adaliszk_console::{ConsoleHandler, ConsoleResult, console};

struct Config;

struct Command;

#[console()]
impl ConsoleHandler<Config> for Command
{
    async fn handle(&self, _config: &Config) -> ConsoleResult
    {
        Ok(())
    }
}

fn main() {}
