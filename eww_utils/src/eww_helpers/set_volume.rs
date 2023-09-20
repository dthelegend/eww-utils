use std::process::Command;

pub fn set_volume(id: String, direction: i32, value: f32) -> Result<(), String> {

    let output = Command::new("wpctl")
        .arg("set-volume")
        .arg(id)
        .arg("--limit=1")
        .arg(format!("{}%{}", value.abs(), match direction {
            -1 => "-",
            1 => "+",
            _ => ""
        }))
        .output();

    match output {
        Ok(s) => if s.status.success() { Ok(()) } else { Err(format!("Failed to set volume!\nStatus: {}", s.status))},
        Err(err) => Err(format!("Error running command!\nReason: {}", err.kind()))
    }
}