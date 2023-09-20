use std::process::Command;

pub fn move_workspace(direction :i32) -> Result<(), String> {

    let output = Command::new("hyprctl")
        .arg("dispatch")
        .arg("workspace")
        .arg(format!("e{}{}", if direction >= 0 {"+"} else {"-"}, direction.abs()))
        .output();

    match output {
        Ok(s) => if s.status.success() { Ok(()) } else { Err(format!("Failed to move workspace!\nStatus: {}", s.status))},
        Err(err) => Err(format!("Error running command!\nReason: {}", err.kind()))
    }
}