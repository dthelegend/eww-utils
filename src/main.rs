use clap::{Parser, Subcommand};

pub mod hyprland_utils;
pub mod eww_helpers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct MainArgs  {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Subcommand)]
enum MainCommands {
    // Commands relating to fetching information for EWW
    EwwHelpers {
        #[command(subcommand)]
        command: eww_helpers::EwwHelperCommands
    },
}

fn main_handler(command: MainCommands) -> Result<(), String> {
    match command {
        MainCommands::EwwHelpers { command } => eww_helpers::eww_handler(command)
    }
}

fn main() -> Result<(), String> {
    let MainArgs { command } = MainArgs::parse();

    main_handler(command)
}
