use std::{str, os::unix::net::UnixStream, io::BufReader, io::BufRead, env::{VarError, var}, process::Command};
use regex::Regex;
use clap::{Parser, Subcommand, Args};

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

#[derive(Subcommand)]
enum EwwHelperCommands {
    GetActiveWindowTitle,
    ListWorkspaces,
    MoveWorkspace(MoveWorkspaceArgs),
}

fn main() -> Result<(), String> {
    let MainArgs { command } = MainArgs::parse();

    main_handler(command)
}

fn main_handler(command: MainCommands) -> Result<(), String> {
    match command {
        MainCommands::EwwHelpers { command } => eww_handler(command)
    }
}

fn eww_handler(command: EwwHelperCommands) -> Result<(), String>{
    match command {
        EwwHelperCommands::GetActiveWindowTitle => get_active_window_title(),
        EwwHelperCommands::ListWorkspaces => get_workspaces(),
        EwwHelperCommands::MoveWorkspace(args) => move_workspace(args)
    }
}

fn get_workspaces () -> Result<(), String> {
    Ok(())
}

fn get_active_window_title() -> Result<(), String> {
    let output = Command::new("hyprctl")
        .arg("activewindow")
        .arg("-j")
        .output();

    match output {
        Ok(output) => match str::from_utf8(&output.stdout) {
            Ok(output) => match serde_json::from_str::<serde_json::Value>(output) {
                Ok(output) => match serde_json::from_value::<String>((&output["title"]).clone()) {
                    Ok(title) => println!("{}", title),
                    Err(err) => return Err(format!("Error converting to String!\nReason: {:?}", err))
                },
                Err(err) => return Err(format!("Error converting to JSON!\nReason: {:?}", err))
            },
            Err(err) => return Err(format!("Error collecting stdout!\nReason: {:?}", err))
        },
        Err(err) => return Err(format!("Error running command!\nReason: {}", err.kind()))
    };

    let hyprland_instance_signature = match var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(key) => key,
        Err(err) => {
            match err {
                VarError::NotPresent => panic!("No HYPRLAND_INSTANCE_SIGNATURE detected!"),
                VarError::NotUnicode(string) => panic!("HYPRLAND_INSTANCE_SIGNATURE is not Unicode? Got: \"{:?}\"", string)
            }
        }
    };

    let socket_address = format!("/tmp/hypr/{}/.socket2.sock", hyprland_instance_signature);

    match UnixStream::connect(&socket_address) {
        Ok(stream) => {
            let re = match Regex::new(r"(?P<EVENT>\w+)>>(?P<DATA>.+)") {
                Ok(re) => re,
                Err(_) => return Err(format!("Failed to compile regex!"))
            };

            let buffered_reader = BufReader::new(stream);
            
            for line in buffered_reader.lines() {
                match line {
                    Ok(line) => {
                        let caps = match re.captures(&line) {
                            Some(caps) => caps,
                            None => return Err(format!("Failed to parse line \"{}\"", line))
                        };
                        
                        match &caps["EVENT"] {
                            "activewindow" => {
                                let data : Vec<&str> = (&caps["DATA"]).split(",").collect();
                                assert!(data.len() == 2, "Data length is incorrect!");
                                // let class = data[0];
                                let title = data[1];

                                println!("{}", title)
                            },
                            _ => ()
                        }                        
                    },
                    Err(err) => return Err(format!("Error reading line!\nReason: {}", err.kind()))
                }
            }
        },
        Err(err) => return Err(format!("Failed to connect to Unix Socket at \"{}\"!\nReason: {:?}", socket_address, err.kind()))
    };

    Ok(())
}

fn direction_parser(s: &str) -> Result<i32, String>{
    match s {
        "up" => Ok(1),
        "down" => Ok(-1),
        s => s.parse().map_err(|_| format!("`{s}` is not a valid direction"))
    }
}

#[derive(Args)]
struct MoveWorkspaceArgs {
    #[arg(value_parser = direction_parser)]
    direction: i32,
}

fn move_workspace(args : MoveWorkspaceArgs) -> Result<(), String> {

    let output = Command::new("hyprctl")
        .arg("dispatch")
        .arg("workspace")
        .arg(format!("e{}{}", if args.direction >= 0 {"+"} else {"-"}, args.direction.abs()))
        .output();

    match output {
        Ok(s) => if s.status.success() { Ok(()) } else { Err(format!("Failed to move workspace!\nStatus: {}", s.status))},
        Err(err) => Err(format!("Error running command!\nReason: {}", err.kind()))
    }
}

// fn kill_window_on_lose_focus(args : KillWindowOnLoseFocusArgs) {
//     let KillWindowOnLoseFocusArgs { window_selector } = args;
    
//     let hyprland_instance_signature = match std::env::var("HYPRLAND_INSTANCE_SIGNATURE") {
//         Ok(key) => key,
//         Err(err) => {
//             match err {
//                 VarError::NotPresent => panic!("No HYPRLAND_INSTANCE_SIGNATURE detected!"),
//                 VarError::NotUnicode(string) => panic!("HYPRLAND_INSTANCE_SIGNATURE is not Unicode? Got: \"{:?}\"", string)
//             }
//         }
//     };

//     let socket_address = format!("/tmp/hypr/{}/.socket2.sock", hyprland_instance_signature);

//     match UnixStream::connect(&socket_address) {
//         Ok(stream) => {
//             let re = match Regex::new(r"(?P<EVENT>\w+)>>(?P<DATA>.+)") {
//                 Ok(re) => re,
//                 Err(_) => panic!("Failed to compile regex!")
//             };
            
//             let selecta = match window_selector.strip_prefix("title:") {
//                 Some(stripped_selector) => WindowSelector::Title(match Regex::new(stripped_selector) {
//                     Ok(re) => re,
//                     Err(_) => panic!("Failed to compile Window Selector (title) regex \"{}\"", stripped_selector)
//                 }),
//                 None => WindowSelector::Class(match Regex::new(&window_selector) {
//                     Ok(re) => re,
//                     Err(_) => panic!("Failed to compile Window Selector (class) regex \"{}\"", window_selector)
//                 })
//             };

//             println!("Connected to Unix Socket at \"{}\"", socket_address);
//             let buffered_reader = BufReader::new(stream);

//             let window_to_be_killed : Some(str) = None;
            
//             for line in buffered_reader.lines() {
//                 match line {
//                     Ok(line) => {
//                         let caps = match re.captures(&line) {
//                             Some(caps) => caps,
//                             None => panic!("Failed to parse line \"{}\"", line)
//                         };
                        
//                         match &caps["EVENT"] {
//                             "activewindow" => {
//                                 let data : Vec<&str> = (&caps["DATA"]).split(",").collect();
//                                 assert!(data.len() == 2, "Data length is incorrect!");
//                                 let title = data[0];
//                                 let class = data[1];
                                
//                                 let is_to_be_killed = match &selecta {
//                                     WindowSelector::Title(s) => s.is_match(title),
//                                     WindowSelector::Class(s) => s.is_match(class)
//                                 };
                                
//                                 println!("{}", &caps["DATA"])
//                             },
//                             _ => ()
//                         }                        
//                     },
//                     Err(err) => panic!("Error reading line!\nReason: {}", err.kind())
//                 }
//             }
//         },
//         Err(err) => panic!("Failed to connect to Unix Socket at \"{}\"!\nReason: {:?}", socket_address, err.kind())
//     };
// }