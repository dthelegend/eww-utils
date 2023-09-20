use std::{process::Command, str};
use serde::Serialize;

use crate::hyprland_utils::{HyprctlMonitorInfoObject, HyprctlWorkspaceInfoObject, get_hyprland_events, HyprlandEvent};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceInfo {
    id: i32,
    windows : i32,
    name : String,
    is_active: bool
}

pub fn get_workspace_info_list () -> Result<(), String> {
    let mut hyprctl_workspace_info_list_command = Command::new("hyprctl");
    hyprctl_workspace_info_list_command
            .arg("workspaces")
            .arg("-j");

    let mut hyprctl_monitor_info_list_command = Command::new("hyprctl");
    hyprctl_monitor_info_list_command
        .arg("monitors")
        .arg("-j");

    let mut get_workspace_info_list_corrollary = || -> Result<Vec<WorkspaceInfo>, String> {
        let hyprctl_workspace_info_list_output = hyprctl_workspace_info_list_command.output();

        let hyprctl_monitor_info_list_output = hyprctl_monitor_info_list_command.output();

        let hyprctl_monitor_info_list = hyprctl_monitor_info_list_output.map_err(|_| "Failed to run command!".to_owned())
        .and_then(|s| str::from_utf8(&s.stdout).map(|js| js.to_owned()).map_err(|_| "Failed to run command!".to_owned()))
        .and_then(|js| serde_json::from_str::<serde_json::Value>(&js).map_err(|_| "Failed to run command!".to_owned()))
        .and_then(|j| serde_json::from_value::<Vec<HyprctlMonitorInfoObject>>(j).map_err(|_| "Failed to get list!".to_owned()))?;

        let hyprctl_workspace_info_list = hyprctl_workspace_info_list_output.map_err(|_| "Failed to run command!".to_owned())
            .and_then(|s| str::from_utf8(&s.stdout).map(|js| js.to_owned()).map_err(|_| "Failed to run command!".to_owned()))
            .and_then(|js| serde_json::from_str::<serde_json::Value>(&js).map_err(|_| "Failed to run command!".to_owned()))
            .and_then(|j| serde_json::from_value::<Vec<HyprctlWorkspaceInfoObject>>(j).map_err(|_| "Failed to get list!".to_owned()))?;
    
        let mut workspace_info = hyprctl_workspace_info_list
            .iter()
            .map(|hyprctl_workspace_info| WorkspaceInfo {
                    id: hyprctl_workspace_info.id,
                    name: hyprctl_workspace_info.name.to_owned(),
                    windows: hyprctl_workspace_info.windows,
                    is_active: hyprctl_monitor_info_list
                        .iter()
                        .any(|monitor_info| {
                            monitor_info.active_workspace.id == hyprctl_workspace_info.id
                        })
            })
            .collect::<Vec<_>>();
    
        workspace_info.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(workspace_info)
    };

    let events = get_workspace_info_list_corrollary()?;
            
    let serialised_events = serde_json::to_string(&events).map_err(|_| "Failed to serialise active window info".to_owned())?;

    println!("{}", serialised_events);

    let hyprland_events = get_hyprland_events()?;

    for event_result in hyprland_events {
        let HyprlandEvent { event, data:_ } = event_result?;

        match event.as_str() {
            "workspace" | "focusedmon" => {
                let events = get_workspace_info_list_corrollary()?;
            
                let serialised_events = serde_json::to_string(&events).map_err(|_| "Failed to serialise active window info".to_owned())?;
                
                std::thread::sleep(std::time::Duration::from_millis(100));

                println!("{}", serialised_events);
            }
            _ => ()
        };
    };

    Ok(())
}
