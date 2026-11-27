use adaliszk_console::console_args;

// The command link lives on #[console_command(args = ...)], not here:
#[console_args("example")]
struct ExampleArgs
{
    #[arg(long)]
    force: bool,
}

fn main() {}
