use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "devspaces - a CLI to manage development workspaces",
    long_about = "devppaces will simplify your work by creating reproducible dev environments for everyone"
)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init(SpaceArgs),
    Add(SpaceArgs),
    Remove(SpaceArgs),
}

#[derive(Args)]
struct SpaceArgs {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init(_args)) => todo!("implement init"),
        Some(Commands::Add(_args)) => todo!("implement remove"),
        Some(Commands::Remove(_args)) => todo!("implement remove"),
        None => todo!("Pass a valid command or check -h for help"),
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    fn get_cli() -> assert_cmd::Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn validate_init() {
        let mut cmd = get_cli();
        let assert = cmd.arg("init").assert();
        assert.failure().code(101);
    }

    #[test]
    fn validate_add() {
        let mut cmd = get_cli();
        let assert = cmd.arg("add").assert();
        assert.failure().code(101);
    }

    #[test]
    fn validate_remove() {
        let mut cmd = get_cli();
        let assert = cmd.arg("remove").assert();
        assert.failure().code(101);
    }
}
