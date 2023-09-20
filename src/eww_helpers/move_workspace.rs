use std::process::Command;

use clap::Args;

pub fn direction_parser(s: &str) -> Result<i32, String>{
    match s {
        "up" => Ok(1),
        "down" => Ok(-1),
        s => s.parse().map_err(|_| format!("`{s}` is not a valid direction"))
    }
}

#[derive(Args)]
pub struct MoveWorkspaceArgs {
    #[arg(value_parser = direction_parser)]
    pub direction: i32,
}

pub fn move_workspace(args : MoveWorkspaceArgs) -> Result<(), String> {

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