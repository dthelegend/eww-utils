use std::{process::Command, str,io::BufRead, num::ParseIntError, time::Duration};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag="type")]
enum NetworkInfo {
    #[serde(rename_all = "camelCase")]
    Wifi {
        signal_strength: u8,
        network_ssid: String
    },
    #[serde(rename_all = "camelCase")]
    Ethernet {
        connection_name: String
    },
    Disconnected
}

pub fn get_network_info(poll_interval: u64) -> Result<(), String> {
    let mut wifi_info_command = Command::new("nmcli");
    wifi_info_command
        .arg("-terse")
        .arg("-fields=SSID,SIGNAL,ACTIVE")
        .arg("device")
        .arg("wifi");

    let mut connection_info_command = Command::new("nmcli");
    connection_info_command
        .arg("-terse")
        .arg("-fields=TYPE,STATE,CONNECTION")
        .arg("device")
        .arg("status");

    let mut get_wifi_info = || -> Result<NetworkInfo, String> {
        let wifi_info_output = wifi_info_command.output()
            .map_err(|err| format!("Failed to run nmcli command!\nReason: \"{}\"", err.kind()))?;

        for line in wifi_info_output.stdout.lines() {
            let line_string = line
                .map_err(|err| format!("Failed to read line\nReason: \"{}\"", err.kind()))?;
            let line_values: Vec<&str> = line_string.split(':').collect();

            match line_values.as_slice() {
                &[ssid, signal, active] => if active == "yes" {
                    let signal_strength: u8 = signal
                        .parse()
                        .map_err(|err: ParseIntError| {format!("failed to parse SIGNAL!\nReason: {:?}", err)})?;

                    return Ok(NetworkInfo::Wifi {
                        network_ssid: ssid.to_owned(),
                        signal_strength
                    })
                },
                _ => return Err(format!("Line has unexpected format \"{}\"", line_string))
            }
        }

        Err("No connected wifi network!".to_owned())
    };

    let mut get_connection_info = || -> Result<NetworkInfo, String> {
        let con_info_output = connection_info_command.output()
            .map_err(|err| format!("Failed to run nmcli command!\nReason: \"{}\"", err.kind()))?;

        for line in con_info_output.stdout.lines() {
            let line_string = line
                .map_err(|err| format!("Failed to read line\nReason: \"{}\"", err.kind()))?;
            let line_values: Vec<&str> = line_string.split(':').collect();

            let connection = match line_values.as_slice() {
                &[connection_type, state, connection_name] => match (state, connection_type) {
                    ("connected", "wifi") => get_wifi_info(),
                    ("connected", "ethernet") => Ok(NetworkInfo::Ethernet { connection_name: connection_name.to_owned() }),
                    _ => Ok(NetworkInfo::Disconnected),
                },
                _ => Err(format!("Line has unexpected format \"{}\"", line_string))
            }?;
            
            match connection {
                NetworkInfo::Disconnected => continue,
                v => return Ok(v)
            }
        }

        Ok(NetworkInfo::Disconnected)
    };

    loop {
        let connection_info = get_connection_info()?;
        let net_info_str = serde_json::to_string(&connection_info)
            .map_err(|e| format!("Failed to serialize network info!\nReason: {:?}", e))?;

        println!("{}", net_info_str);

        std::thread::sleep(Duration::from_millis(poll_interval))
    }
}