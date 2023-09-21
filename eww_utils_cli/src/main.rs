use clap::{Parser, Subcommand};
use eww_utils::eww_helpers;

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
        command: EwwHelperCommands
    },
}

fn main_handler(command: MainCommands) -> Result<(), String> {
    match command {
        MainCommands::EwwHelpers { command } => eww_handler(command)
    }
}

fn direction_parser(s: &str) -> Result<i8, String>{
    match s {
        "up" => Ok(1),
        "down" => Ok(-1),
        s => s.parse().map_err(|_| format!("`{s}` is not a valid direction"))
    }
}

#[derive(Subcommand)]
enum EwwHelperCommands {
    GetActiveWindowInfo,
    GetWorkspaceInfoList,
    SetVolume {
        id: String,
        #[arg(value_parser = direction_parser)]
        direction: i8,
        value: f32,
    },
    MoveWorkspace {
        #[arg(value_parser = direction_parser)]
        direction: i32,
    },
    GetNetworkInfo {
        #[arg(default_value_t = 5000)]
        poll_interval: u64,
    }
}



fn eww_handler(command: EwwHelperCommands) -> Result<(), String>{
    match command {
        EwwHelperCommands::GetActiveWindowInfo => eww_helpers::get_active_window_info::get_active_window_info(),
        EwwHelperCommands::GetWorkspaceInfoList => eww_helpers::get_workspace_info_list::get_workspace_info_list(),
        EwwHelperCommands::MoveWorkspace { direction } => eww_helpers::move_workspace::move_workspace(direction),
        EwwHelperCommands::GetNetworkInfo { poll_interval } => eww_helpers::get_network_info::get_network_info(poll_interval),
        EwwHelperCommands::SetVolume { id, direction, value } => eww_helpers::set_volume::set_volume(id, direction, value)
    }
}

fn main() -> Result<(), String> {
    let MainArgs { command } = MainArgs::parse();

    main_handler(command)
}
