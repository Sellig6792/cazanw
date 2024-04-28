use cprint::ceprintln;

mod cli;

fn main() {
    let cli: cli::Cli = argh::from_env();

    if cli.version {
        println!(
            "{} version {}",
            env!("CARGO_BIN_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        return;
    }

    match cli.subcommand {
        Some(subcommand) => {
            subcommand.run();
        }
        None => {
            ceprintln!("No subcommand was used");
        }
    }
}
