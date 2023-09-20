use clap::Subcommand;

pub mod get_active_window_info;
pub mod get_workspace_info_list;
pub mod move_workspace;
pub mod get_battery_info;
pub mod get_network_info;

#[derive(Subcommand)]
pub enum EwwHelperCommands {
    GetActiveWindowInfo,
    GetWorkspaceInfoList,
    MoveWorkspace(move_workspace::MoveWorkspaceArgs),
    GetBatteryInfo(get_battery_info::GetBatteryInfoArguments),
    GetNetworkInfo(get_network_info::GetNetworkInfoArguments)
}

pub fn eww_handler(command: EwwHelperCommands) -> Result<(), String>{
    match command {
        EwwHelperCommands::GetActiveWindowInfo => get_active_window_info::get_active_window_info(),
        EwwHelperCommands::GetWorkspaceInfoList => get_workspace_info_list::get_workspace_info_list(),
        EwwHelperCommands::MoveWorkspace(args) => move_workspace::move_workspace(args),
        EwwHelperCommands::GetBatteryInfo(args) => get_battery_info::get_battery_info(args),
        EwwHelperCommands::GetNetworkInfo(args) => get_network_info::get_network_info(args)
    }
}