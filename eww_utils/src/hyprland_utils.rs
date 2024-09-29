use serde::Deserialize;
use regex::Regex;
use std::{str, os::unix::net::UnixStream, io::BufReader, io::BufRead, env::{VarError, var}};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprctlShortWorkspaceObject {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprctlActiveWindowObject {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [i32; 2],
    pub size: [u32; 2],
    pub workspace: HyprctlShortWorkspaceObject,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: i32,
    pub class: String,
    pub title: String,
    pub initial_class: String,
    pub initial_title: String,
    pub pid: u32,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: i32,
    pub fullscreen_client: i32,
    pub grouped: Vec<String>,
    pub swallowing: String,
    pub focus_history_i_d: i32
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprctlWorkspaceInfoObject {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub windows : i32,
    pub hasfullscreen: bool,
    pub lastwindow: String,
    pub lastwindowtitle: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyprctlMonitorInfoObject {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
    pub x: i32,
    pub y: i32,
    pub active_workspace: HyprctlShortWorkspaceObject,
    pub special_workspace: HyprctlShortWorkspaceObject,
    pub reserved: [i32; 4],
    pub scale: f32,
    pub transform: i32,
    pub focused: bool,
    pub dpms_status: bool,
    pub vrr: bool
}

pub struct HyprlandEvent {
    pub event: String,
    pub data: String
}

pub fn get_hyprland_events() -> Result<impl Iterator<Item=Result<HyprlandEvent, String>>, String>
{
    let hyprland_instance_signature = var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|err| {
            match err {
                VarError::NotPresent => "No HYPRLAND_INSTANCE_SIGNATURE detected!".to_owned(),
                VarError::NotUnicode(string) => format!("HYPRLAND_INSTANCE_SIGNATURE is not Unicode? Got: \"{:?}\"", string)
            }
        })?;

    let xdg_runtime_dir = var("XDG_RUNTIME_DIR")
        .map_err(|err| {
            match err {
                VarError::NotPresent => "No XDG_RUNTIME_DIR detected!".to_owned(),
                VarError::NotUnicode(string) => format!("XDG_RUNTIME_DIR is not Unicode? Got: \"{:?}\"", string)
            }
        })?;
    
    let re = match Regex::new(r"(?P<EVENT>\w+)>>(?P<DATA>.+)") {
        Ok(re) => re,
        Err(_) => return Err("Failed to compile regex!".to_owned())
    };

    let socket_address = format!("{xdg_runtime_dir}/hypr/{hyprland_instance_signature}/.socket2.sock");

    let buffered_connection = UnixStream::connect(&socket_address)
            .map_err(|err| format!("Failed to connect to Unix Socket at \"{}\"!\nReason: {:?}", socket_address, err.kind()))
            .map(BufReader::new);

    buffered_connection.map( |b| {
        b.lines().map(move |line_result| {
            line_result
                .map_err(|err| format!("Error reading line!\nReason: {}", err.kind()))
                .and_then(|line| {
                match re.captures(&line) {
                    Some(caps) => Ok(HyprlandEvent {
                        event: caps["EVENT"].to_owned(),
                        data: caps["DATA"].to_owned(),
                    }),
                    None => Err(format!("Failed to parse line \"{}\"", line))
                }
            })
        })
    })
}