use std::{process::Command, str};
use serde::Serialize;
use crate::hyprland_utils;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActiveWindowInfo {
    title : String,
    class : String
}

pub fn get_active_window_info() -> Result<(), String> {
    let get_active_window_output = Command::new("hyprctl")
        .arg("activewindow")
        .arg("-j")
        .output();

    let initial_active_window_info : ActiveWindowInfo = get_active_window_output.map_err(|_| "Failed to run command!".to_owned())
        .and_then(|s| str::from_utf8(&s.stdout).map(|js| js.to_owned()).map_err(|_| "Failed to get command output!".to_owned()))
        .and_then(|js| serde_json::from_str::<hyprland_utils::HyprctlActiveWindowObject>(&js).map_err(|e| format!("Failed to deserialize command output!\nReason: \"{:?}\"", e)))
        .map(|j| ActiveWindowInfo {
            title: j.title,
            class: j.class
        })?;

    let serialised_initial_active_window_info = serde_json::to_string(&initial_active_window_info)
        .map_err(|_| "Failed to serialise active window info".to_owned())?;

    println!("{}", serialised_initial_active_window_info);

    let hyprland_events = hyprland_utils::get_hyprland_events()?;

    for event in hyprland_events {
        let hyprland_utils::HyprlandEvent { event, data } = event?;
    
        if event.as_str() == "activewindow" {
            let split_data : Vec<&str> = data.split(',').collect();
            assert!(split_data.len() == 2, "Data length is incorrect!");
        
            let active_window_info = ActiveWindowInfo {
                class: split_data[0].to_owned(),
                title: split_data[1].to_owned()
            };

            let serialised_active_window_info = serde_json::to_string(&active_window_info)
                .map_err(|_| "Failed to serialise active window info".to_owned())?;
            println!("{}", serialised_active_window_info);
        }
    }

    Ok(())
}
